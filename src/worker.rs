use std::{net::TcpStream, sync::{Mutex, Arc, mpsc, RwLock}, thread::{JoinHandle, self}, collections::HashMap};

use chrono::Local;

use crate::{go::Go, init::Init, log::Log, fastcgi::{Record, FASTCGI_MAX_REQUEST_LEN, FastCGI, RecordType, HeaderType, ContentData}, price::Price, cache::Cache};

#[derive(Debug)]
pub enum Message {
    Terminate,          // Зупинити всі потоки
    Job(TcpStream),     // Прийняти з’єднання fastCGI з веб-сервера
}

#[derive(PartialEq, Debug)]
pub enum Status {
    None,               // Nothing or Init
    Begin,              // Receive a "Begin" request
    Param,              // Receive a "Param" request
    ParamEnd,           // Receive a empty "Param" request
    Stdin,              // Receive a "Stdin" request
    Work,               // Receive a empty "Stdin" request and start CRM system
    End,                // Finish
  } 

#[derive(Debug)]
pub struct Worker {
    pub start: bool,                    // Worker запущено
    pub stop: bool,                     // Відправлен "stop" сигнал
    thread: Option<JoinHandle<()>>,     // Потік 
    status: Status,                     // Статус
    pub init: Arc<RwLock<Init>>,
    pub log: Arc<RwLock<Log>>,
    pub cache: Arc<Mutex<Cache>>,
}

impl Worker {
    pub fn new(go: Arc<Mutex<Go>>, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, init: Arc<RwLock<Init>>, log: Arc<RwLock<Log>>) -> Arc<Mutex<Worker>> {
        let cache;
        {
            let g = Mutex::lock(&go).unwrap();
            cache = Arc::clone(&g.cache.as_ref().unwrap());
        }
        let worker = Worker {
            start: false,
            stop: false,
            thread: None,
            status: Status::None,
            init: Arc::clone(&init),
            log: Arc::clone(&log),
            cache,
        };

        let worker = Arc::new(Mutex::new(worker));
        let worker_thread = Arc::clone(&worker);

        let thread = thread::spawn(move || {
            let log = RwLock::read(&log).unwrap();
            loop {
                let mut begin_record: Option<Record> = None;
                let mut param_record: HashMap<String, String> = HashMap::with_capacity(128);
                let mut stdin_record: Option<Vec<u8>> = None;
                let wait = Mutex::lock(&receiver).unwrap();
                match wait.recv() {
                    Ok(message) => match message {
                        Message::Job(stream) => {
                            Worker::fastcgi_connection(Arc::clone(&worker_thread), stream, &mut begin_record, &mut param_record, &mut stdin_record);
                            {
                                let mut w = Mutex::lock(&worker_thread).unwrap();
                                w.start = false;
                                w.status = Status::None;
                            }
                            {
                                let mut g = Mutex::lock(&go).unwrap();
                                g.use_connection -= 1;
                            }
                        },
                        Message::Terminate => break,
                    },
                    Err(err) => log.exit(500, &err.to_string()),
                }
            }
        });
        {
            let mut w = Mutex::lock(&worker).unwrap();
            w.thread = Some(thread);
        }
        worker
    }
    
    pub fn join(worker: Arc<Mutex<Worker>>) {
        let thread;
        {
            let mut w = Mutex::lock(&worker).unwrap();
            thread = w.thread.take();
        }
        
        if let Some(main) = thread {
            main.join().unwrap();
        }
    }

    pub fn fastcgi_connection(worker: Arc<Mutex<Worker>>, mut stream: TcpStream, begin_record: &mut Option<Record>, param_record: &mut HashMap<String, String>, stdin_record: &mut Option<Vec<u8>>) {
        let mut buffer: [u8; FASTCGI_MAX_REQUEST_LEN] = [0; FASTCGI_MAX_REQUEST_LEN];
        let mut seek: usize = 0;
        let mut size: usize = 0;
        let mut need_read = true;
        loop {
            {
                let w = Mutex::lock(&worker).unwrap();
                if w.stop {
                    break;
                }
            }
            let record = match FastCGI::read_record(&mut seek, &mut size, &mut need_read, &mut buffer[..], &mut stream) {
                RecordType::None => continue,
                RecordType::Some(record) => record,
                RecordType::ErrorStream | RecordType::StreamClosed => break,
            };
            match record.header.header_type {
                HeaderType::BeginRequest => {
                    // Got "Begin" record
                    *begin_record = Some(record);
                    let mut w = Mutex::lock(&worker).unwrap();
                    if Status::None != w.status {
                        break;
                    }
                    w.status = Status::Begin;

                },
                HeaderType::AbortRequest => {
                    // Got "Abort" record
                    if let Some(record) = begin_record {
                        FastCGI::write_abort(&record.header, &mut stream).unwrap_or(());
                    }
                    break;
                },
                HeaderType::Params => {
                    // Got "Param" record
                    {
                        let w = Mutex::lock(&worker).unwrap();
                        match w.status {
                            Status::Begin | Status::Param => {},
                            _ => break,
                        }
                    }
                    match record.data {
                        ContentData::Param(data) => {
                        if param_record.len() == 0 {
                            *param_record = data;
                        } else {
                            for (key, value) in data {
                                param_record.insert(key, value);
                            }
                        } 
                        {
                            let mut w = Mutex::lock(&worker).unwrap();
                            w.status = Status::Param;
                        }
                        },
                        ContentData::None => {
                            let mut w = Mutex::lock(&worker).unwrap();
                            w.status = Status::ParamEnd;
                        },
                        _ => break, 
                    }
                },
                HeaderType::Stdin => {
                    // Got "Stdin" record
                    {
                        let w = Mutex::lock(&worker).unwrap();
                        match w.status {
                            Status::Begin | Status::ParamEnd | Status::Stdin => {},
                            _ => break,
                        }
                    }
                    match record.data {
                        ContentData::Stream(data) => {
                            match stdin_record {
                                Some(param) => param.extend_from_slice(&data[..]),
                                None => *stdin_record = Some(data),
                            } 
                            {
                                let mut w = Mutex::lock(&worker).unwrap();
                                w.status = Status::Stdin;
                            }
                        },
                        ContentData::None => {
                            // Got empty "Stdin" record, so we start the CRM
                            {
                                let mut w = Mutex::lock(&worker).unwrap();
                                w.status = Status::Work;
                            }
                            // Start
                            let mut price = Price::new(Arc::clone(&worker));
                            println!("start price {}", Local::now().format("%Y.%m.%d %H:%M:%S%.9f %:z").to_string()); 
                            let answer = price.calc(param_record);
                            println!("finish price {}", Local::now().format("%Y.%m.%d %H:%M:%S%.9f %:z").to_string()); 
                            {
                                let mut w = Mutex::lock(&worker).unwrap();
                                w.status = Status::End;
                                if w.stop {
                                    break;
                                }
                            }
                            // Write ansewer to the WEB server
                            if let Some(record) = begin_record {
                                FastCGI::write_response(&record.header, answer, &mut stream).unwrap_or(());
                            }
                            break;
                        },
                        _ => break,
                    }
                },
                _ => {},
            };
        }
    }

}
use std::{sync::{Arc, Mutex, mpsc, RwLock}, net::{TcpListener, SocketAddr, TcpStream, Shutdown}, io::{ErrorKind, Read, Write}, time::Duration, thread::{self, JoinHandle}, process};

use crate::{init::Init, log::Log, queue::Queue, worker::{Worker, Message}, cache::Cache};

pub const MS1: std::time::Duration = Duration::from_millis(1);
pub const MS1000: std::time::Duration = Duration::from_millis(1000);

#[derive(Debug)]
pub struct Go {
    pub stop: bool,                                                     // Зупинка системи
    queue: Arc<Mutex<Queue>>,                                           // Черга з'єднань
    tcp: Option<JoinHandle<()>>,                                        // Потік прийому повідомлень від WEB сервера
    sender: Option<JoinHandle<()>>,                                     // Потік обробки черги та відправлення сигналу
    workers: Vec<(Arc<Mutex<Worker>>, mpsc::Sender<Message>)>,          // Потоки обробки даних
    pub use_connection: usize,                                          // Скільки потоків уже запущено
    pub cache: Option<Arc<Mutex<Cache>>>,                                   // Кеш
}

impl Go {
    pub fn run(init: Init, log: Log) {
        let irc = match TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], init.irc))){
            Ok(irc) => irc,
            Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => log.exit(300, ""),
            ErrorKind::AddrInUse => log.exit(301, ""),
            ErrorKind::AddrNotAvailable => log.exit(302, ""),
            _ => log.exit(303, &e.to_string()),
            },
        };

        if let Err(err) = irc.set_nonblocking(true) {
            log.exit(304, &err.to_string());
        }

        let queue = Arc::new(Mutex::new(Queue::new(65536))); 
        let max = init.max;
        let init = Arc::new(RwLock::new(init));
        let log = Arc::new(RwLock::new(log));

        let go = Arc::new(Mutex::new(Go {
            stop: false,
            queue,
            tcp: None,
            sender: None,
            workers: Vec::with_capacity(max),
            use_connection: 0,
            cache: None,
        }));

        let cache = Cache::new(Arc::clone(&go), Arc::clone(&init), Arc::clone(&log));
        loop {
            {
                let c = Mutex::lock(&cache).unwrap();
                if c.load {
                    break;
                }
            }
            thread::sleep(MS1000);
        }
        {
            let mut g = Mutex::lock(&go).unwrap();
            g.cache = Some(cache);
        }

        Go::create_workers(Arc::clone(&go), Arc::clone(&init), Arc::clone(&log));
        Go::create_sender(Arc::clone(&go), Arc::clone(&init), Arc::clone(&log));
        Go::create_tcp(Arc::clone(&go), Arc::clone(&init), Arc::clone(&log));
        

        // Читати irc канал для зупинки
        for stream in irc.incoming() {
            match stream {
                Ok(mut stream) => if let Ok(_) = Go::set_stop(Arc::clone(&go), &mut stream) {
                    break;
                },
                Err(err) => match err.kind() {
                    ErrorKind::WouldBlock => thread::sleep(MS1000),
                    _ => {},
                },
            };
        }
    }

    fn set_stop(go: Arc<Mutex<Go>>, stream: &mut TcpStream) -> Result<(), ()> {
        if let Err(_) = stream.set_read_timeout(Some(Duration::from_secs(1))) {
            if let Err(_) = stream.shutdown(Shutdown::Both) { }
            return Err(());
        }
        let mut buffer: [u8; 1024] = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => match size {
                0 | 1024.. => {
                    if let Err(_) = stream.shutdown(Shutdown::Both) { }
                    return Err(());
                },
                _ => {
                    let data = &buffer[..size];
                    if data == b"stop" {
                        Go::do_stop(go);
                        Go::answer_stop(stream);
                        return Ok(());
                    }
                    if let Err(_) = stream.shutdown(Shutdown::Both) { }
                    return Err(());
                },
            },
            Err(_) => return Err(()), 
        }
    }

    fn do_stop(go: Arc<Mutex<Go>>) {
        let tcp;
        let sender;
        let cache;
        {
            let mut g = Mutex::lock(&go).unwrap();
            g.stop = true;
            tcp = g.tcp.take();
            sender = g.sender.take();
            cache = g.cache.take();
            for (item, sender) in &g.workers {
                {
                    let mut w = Mutex::lock(&item).unwrap();
                    w.stop = true;  
                }
                sender.send(Message::Terminate).unwrap()
            }
        }
        {
            let g = Mutex::lock(&go).unwrap();
            for (item, _) in &g.workers {
                Worker::join(Arc::clone(item));
            }
        }
        if let Some(tcp) = tcp {
            tcp.join().unwrap();
        }
        if let Some(sender) = sender {
            sender.join().unwrap();
        }
        if let Some(cache) = cache {
            Cache::join(cache);
        }
    }

    fn answer_stop(stream: &mut TcpStream) {
        if let Err(_) = stream.write_all(process::id().to_string().as_bytes()) { }
    }

    fn create_workers(go: Arc<Mutex<Go>>, init: Arc<RwLock<Init>>, log: Arc<RwLock<Log>>) {
        let max;
        {
            max = RwLock::read(&init).unwrap().max;
        }
        for _ in 0..max {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
      
            let w = Worker::new(Arc::clone(&go), Arc::clone(&receiver), Arc::clone(&init), Arc::clone(&log));
            {
              let mut g = Mutex::lock(&go).unwrap();
              g.workers.push((w, sender));
            }
        }
    }

    fn create_tcp(go: Arc<Mutex<Go>>, init: Arc<RwLock<Init>>, log: Arc<RwLock<Log>>) {
        let go_init = Arc::clone(&go);
        let tcp = thread::spawn(move || {
            let init = RwLock::read(&init).unwrap();
            let log = RwLock::read(&log).unwrap();
            let bind = match TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], init.port))) {
                Ok(bind) => bind,
                Err(err) => match err.kind() {
                    ErrorKind::PermissionDenied => log.exit(400, ""),
                    ErrorKind::AddrInUse => log.exit(401, ""),
                    ErrorKind::AddrNotAvailable => log.exit(402, ""),
                    _ => log.exit(403, &err.to_string()),
                }
            };
            match bind.set_nonblocking(true) {
                Ok(_) => {
                    for stream in bind.incoming() {
                        {
                            let g = Mutex::lock(&go).unwrap();
                            if g.stop {
                                break;
                            }
                        }
                        match stream {
                            Ok(stream) => {
                                let mut str = stream;
                                let queue;
                                {
                                    let g = Mutex::lock(&go).unwrap(); 
                                    queue = Arc::clone(&g.queue);
                                }
                                loop {
                                    {
                                        let g = Mutex::lock(&go).unwrap(); 
                                        if g.stop {
                                            break;
                                        }
                                    }
                                    {
                                        let mut q = Mutex::lock(&queue).unwrap(); 
                                        match q.push(str) {
                                          Some(s) => str = s,
                                          None => break,
                                        }
                                    }
                                    thread::sleep(MS1);
                                }
                            },
                            Err(err) => match err.kind() {
                                ErrorKind::WouldBlock => thread::sleep(MS1),
                                _ => {},
                            },
                        };
                    }
                },
                Err(err) => log.exit(404, &err.to_string()),
            } 
        });
        {
            let mut g = Mutex::lock(&go_init).unwrap();
            g.tcp = Some(tcp);
        }
    }

    fn create_sender(go: Arc<Mutex<Go>>, init: Arc<RwLock<Init>>, log: Arc<RwLock<Log>>) {
        let go_init = Arc::clone(&go);
        let mut wait = false;
        let sender = thread::spawn(move || {
            let init = RwLock::read(&init).unwrap();
            let log = RwLock::read(&log).unwrap();
            let queue;
            {
                let g = Mutex::lock(&go).unwrap();
                queue = Arc::clone(&g.queue);
            }
            let mut used;
            let max = init.max;
            let mut index: Option<usize>;
            loop {
                {
                    let g = Mutex::lock(&go).unwrap();
                    if g.stop {
                        break;
                    }
                }
                if wait {
                    thread::sleep(MS1);
                    wait = false;
                }
                let tcp;
                {
                    let mut q = Mutex::lock(&queue).unwrap();
                    if q.len == 0 {
                        wait = true;
                        continue;
                    } else {
                        if let Some(t) = q.take() {
                            tcp = t;
                        } else {
                            log.exit(500, "");
                        }
                    }
                }
                loop {
                    index = None;
                    {
                        let g = Mutex::lock(&go).unwrap();
                        if g.stop {
                            break;
                        }
                        used = g.use_connection;
                    }
                    if used < max {
                        {
                            let mut g = Mutex::lock(&go).unwrap();
                            g.use_connection += 1;
                        }
                        for i in 0..max {
                            let g = Mutex::lock(&go).unwrap();
                            let (item, _) = g.workers.get(i).unwrap();
                            {
                                let mut w = Mutex::lock(item).unwrap();
                                if w.start == false {
                                    w.start = true;
                                    index = Some(i);
                                    break;
                                }
                            }
                        }
                        if let None = index {
                            log.exit(501, "");
                        }
                    }
                    if let Some(ind) = index {
                        let g = Mutex::lock(&go).unwrap();
                        let (_, sender) = g.workers.get(ind).unwrap();
                        sender.send(Message::Job(tcp)).unwrap();
                        break;
                    }
                    thread::sleep(MS1);
                }
            }
        });
        {
            let mut g = Mutex::lock(&go_init).unwrap();
            g.sender = Some(sender);
        }
    }
}
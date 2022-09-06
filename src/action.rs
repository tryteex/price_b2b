use std::{process::Command, env, net::{TcpListener, SocketAddr, TcpStream}, io::{ErrorKind, Write, Read}, time::Duration, str::from_utf8};

use crate::{init::Init, log::Log};

pub struct Action {}

impl Action {
    pub fn start(init: Init, log: Log, dir: &str, check: bool) {
        if check {
            if let Err(err) = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], init.irc))){
                if ErrorKind::AddrInUse == err.kind() {
                    println!("Сокет IRC занятий, можливо вже запущен один екземпляр сервера");
                    return; 
                }
            }
        }
        let file = env::current_exe().unwrap();
        match Command::new(file).arg("go").current_dir(dir).spawn() {
            Ok(p) => println!("Сервер запущено. PID={}", p.id()),
            Err(err) => log.exit(100, &err.to_string()),
        };
    }

    pub fn stop(init: Init, log: Log) {
        match TcpStream::connect_timeout(&SocketAddr::from(([127, 0, 0, 1], init.irc)), Duration::from_secs(2)) {
            Ok(mut tcp) => {
                match tcp.write_all(b"stop") {
                    Ok(()) => {
                        if let Err(err) = tcp.set_read_timeout(Some(Duration::from_secs(30))) {
                            log.exit(250, &err.to_string());
                        }
                        let mut buffer: Vec<u8> = Vec::with_capacity(128);
                        match tcp.read_to_end(&mut buffer) {
                            Ok(size) => match size {
                                0 => log.exit(206, ""),
                                _ => {
                                    match from_utf8(&buffer[..size]) {
                                        Ok(s) => {
                                            match s.parse::<i32>() {
                                                Ok(pid) => println!("Сервер зупинено. PID={}", pid),
                                                Err(err) => log.exit(208, &err.to_string()),
                                            };
                                        },
                                        Err(err) => log.exit(207, &err.to_string()),
                                    };
                                },
                            },
                            Err(e) => log.exit(260, &e.to_string()),
                        };
                    },
                    Err(err) => log.exit(203, &err.to_string()),
                };
            },
            Err(err) => match err.kind() {
                ErrorKind::TimedOut => log.exit(202, ""),
                _ => log.exit(201, &err.to_string()),
            },
        };
    }
}
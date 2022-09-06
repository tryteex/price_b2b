use std::{env, fs::{read_to_string, create_dir}, path::Path};

use serde_json::Value;

use crate::log::Log;

#[derive(Debug)]
pub enum AppInit {
    Help,
    Go,
    Start,
    Check,
    Stop,
}

#[derive(Debug)]
pub struct DBInit {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pwd: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Init {
    pub app: AppInit,

    pub irc: u16,
    pub port: u16,
    pub time_zone: String,
    pub max: usize,
    
    pub db_log: DBInit,
    pub db_b2b: DBInit,
    pub db_local: DBInit,

    pub dir: String,
    pub salt: String,
}

impl Init {
    pub fn new(log: &Log, dir: &str) -> Init {
        let mut args = env::args();
        args.next();
        let app = match args.next() {
          None => AppInit::Help,
          Some(arg) => match arg.as_str() {
            "start" => AppInit::Start,
            "check" => AppInit::Check,
            "stop" => AppInit::Stop,
            "go" => AppInit::Go,
            _ => AppInit::Help,
          },
        }; 

        let file_name;
        if cfg!(debug_assertions) {
            file_name = format!("{}/init_debug.config", dir);
        } else {
            file_name = format!("{}/init.config", dir);
        }
        let cfg = match read_to_string(&file_name) {
          Ok(cfg) => cfg,
          Err(err) => log.exit(100, &err.to_string()),
        };

        let val: Value = match serde_json::from_str(&cfg){
            Ok(val) => val,
            Err(err) => log.exit(101, &err.to_string()),
        };
        let port: u16 = match val.get("port") {
            Some(port) => match port.as_u64() {
                Some(port) => match u16::try_from(port) {
                    Ok(port) => match port {
                        0 => log.exit(103, ""),
                        port => port,
                    },
                    Err(err) => log.exit(103, &err.to_string()),
                },
                None => log.exit(103, ""),
            },
            None => log.exit(102, ""),
        };
        let time_zone: String = match val.get("time_zone") {
            Some(time_zone) => match time_zone.as_str() {
                Some(time_zone) => time_zone.to_owned(),
                None => log.exit(105, ""),
            },
            None => log.exit(104, ""),
        };
        let max: u8 = match val.get("max_thread") {
            Some(max) => match max.as_u64() {
                Some(max) => match u8::try_from(max) {
                    Ok(max) => match max {
                        0 => log.exit(107, ""),
                        max => max,
                    },
                    Err(err) => log.exit(107, &err.to_string()),
                },
                None => log.exit(107, ""),
            },
            None => log.exit(106, ""),
        };

        let db_log_host: String = match val.get("db_log_host") {
            Some(host) => match host.as_str() {
                Some(host) => host.to_owned(),
                None => log.exit(109, ""),
            },
            None => log.exit(108, ""),
        };
        let db_log_port: u16 = match val.get("db_log_port") {
            Some(port) => match port.as_u64() {
                Some(port) => match u16::try_from(port) {
                    Ok(port) => match port {
                        0 => log.exit(111, ""),
                        port => port,
                    },
                    Err(err) => log.exit(111, &err.to_string()),
                },
                None => log.exit(111, ""),
            },
            None => log.exit(110, ""),
        };
        let db_log_user: String = match val.get("db_log_user") {
            Some(user) => match user.as_str() {
                Some(user) => user.to_owned(),
                None => log.exit(113, ""),
            },
            None => log.exit(112, ""),
        };
        let db_log_pwd: String = match val.get("db_log_pwd") {
            Some(pwd) => match pwd.as_str() {
                Some(pwd) => pwd.to_owned(),
                None => log.exit(115, ""),
            },
            None => log.exit(114, ""),
        };
        let db_log_name: String = match val.get("db_log_name") {
            Some(name) => match name.as_str() {
                Some(name) => name.to_owned(),
                None => log.exit(117, ""),
            },
            None => log.exit(116, ""),
        };
        let db_log = DBInit{host: db_log_host, port: db_log_port, user: db_log_user, pwd: db_log_pwd, name: db_log_name};

        let db_b2b_host: String = match val.get("db_b2b_host") {
            Some(host) => match host.as_str() {
                Some(host) => host.to_owned(),
                None => log.exit(119, ""),
            },
            None => log.exit(118, ""),
        };
        let db_b2b_port: u16 = match val.get("db_b2b_port") {
            Some(port) => match port.as_u64() {
                Some(port) => match u16::try_from(port) {
                    Ok(port) => match port {
                        0 => log.exit(121, ""),
                        port => port,
                    },
                    Err(err) => log.exit(121, &err.to_string()),
                },
                None => log.exit(121, ""),
            },
            None => log.exit(120, ""),
        };
        let db_b2b_user: String = match val.get("db_b2b_user") {
            Some(user) => match user.as_str() {
                Some(user) => user.to_owned(),
                None => log.exit(123, ""),
            },
            None => log.exit(122, ""),
        };
        let db_b2b_pwd: String = match val.get("db_b2b_pwd") {
            Some(pwd) => match pwd.as_str() {
                Some(pwd) => pwd.to_owned(),
                None => log.exit(125, ""),
            },
            None => log.exit(124, ""),
        };
        let db_b2b_name: String = match val.get("db_b2b_name") {
            Some(name) => match name.as_str() {
                Some(name) => name.to_owned(),
                None => log.exit(127, ""),
            },
            None => log.exit(126, ""),
        };
        let db_b2b = DBInit{host: db_b2b_host, port: db_b2b_port, user: db_b2b_user, pwd: db_b2b_pwd, name: db_b2b_name};

        let db_local_host: String = match val.get("db_local_host") {
            Some(host) => match host.as_str() {
                Some(host) => host.to_owned(),
                None => log.exit(129, ""),
            },
            None => log.exit(128, ""),
        };
        let db_local_port: u16 = match val.get("db_local_port") {
            Some(port) => match port.as_u64() {
                Some(port) => match u16::try_from(port) {
                    Ok(port) => match port {
                        0 => log.exit(131, ""),
                        port => port,
                    },
                    Err(err) => log.exit(131, &err.to_string()),
                },
                None => log.exit(131, ""),
            },
            None => log.exit(130, ""),
        };
        let db_local_user: String = match val.get("db_local_user") {
            Some(user) => match user.as_str() {
                Some(user) => user.to_owned(),
                None => log.exit(133, ""),
            },
            None => log.exit(132, ""),
        };
        let db_local_pwd: String = match val.get("db_local_pwd") {
            Some(pwd) => match pwd.as_str() {
                Some(pwd) => pwd.to_owned(),
                None => log.exit(135, ""),
            },
            None => log.exit(134, ""),
        };
        let db_local_name: String = match val.get("db_local_name") {
            Some(name) => match name.as_str() {
                Some(name) => name.to_owned(),
                None => log.exit(137, ""),
            },
            None => log.exit(136, ""),
        };
        let db_local = DBInit{host: db_local_host, port: db_local_port, user: db_local_user, pwd: db_local_pwd, name: db_local_name};

        let irc: u16 = match val.get("irc") {
            Some(port) => match port.as_u64() {
                Some(port) => match u16::try_from(port) {
                    Ok(port) => match port {
                        0 => log.exit(139, ""),
                        port => port,
                    },
                    Err(err) => log.exit(139, &err.to_string()),
                },
                None => log.exit(139, ""),
            },
            None => log.exit(138, ""),
        };

        let cache = format!("{}/cache", dir);
        let path = Path::new(&cache);

        if path.exists() {
            if !path.is_dir() {
                log.exit(502, &cache);
            }
        } else if let Err(err) = create_dir(path) {
            log.exit(503, &err.to_string());
        }

        let salt: String = match val.get("salt") {
            Some(salt) => match salt.as_str() {
                Some(salt) => salt.to_owned(),
                None => log.exit(137, ""),
            },
            None => log.exit(136, ""),
        };

        Init {app, port, irc, time_zone, max: max.into(), db_log, db_b2b, db_local, dir: dir.to_owned(), salt, }
    }
}
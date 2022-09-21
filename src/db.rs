use std::sync::{Arc, RwLock};

use mysql::{Opts, Conn, prelude::{Queryable, FromRow}};

use crate::{init::DBInit, log::Log};

pub struct DB {
    conn: Conn,
    log: Option<Arc<RwLock<Log>>>,
}

impl DB {
    pub fn new(db: &DBInit, log: Arc<RwLock<Log>>) -> Option<DB> {
        let log_read = RwLock::read(&log).unwrap();
        let url: &str = &format!("mysql://{}:{}@{}:{}/{}?tcp_connect_timeout_ms=500", db.user, db.pwd, db.host, db.port, db.name);
        let opts = match Opts::try_from(url) {
            Ok(opts) => opts,
            Err(err) => {
                log_read.write(600, &format!("{}. Err: {}", url, err.to_string()));
                return None;
            },
        };
        let mut conn = match Conn::new(opts) {
            Ok(conn) => conn,
            Err(err) => {
                log_read.write(601, &format!("{}. Err: {}", url, err.to_string()));
                return None;
            },
        };
        if let Err(err) = conn.query_drop("SET NAMES 'utf8'") {
            log_read.write(603, &err.to_string());
            return None;
        }
        if let Err(err) = conn.query_drop("SET CHARACTER SET 'utf8'") {
            log_read.write(603, &err.to_string());
            return None;
        }
        if let Err(err) = conn.query_drop("SET SESSION collation_connection = 'utf8_general_ci'") {
            log_read.write(603, &err.to_string());
            return None;
        }

        Some(DB { 
            conn,
            log: Some(Arc::clone(&log)),
         })
    }

    pub fn simple(db: &DBInit) -> Option<DB> {
         let url: &str = &format!("mysql://{}:{}@{}:{}/{}?tcp_connect_timeout_ms=500", db.user, db.pwd, db.host, db.port, db.name);
        let opts = match Opts::try_from(url) {
            Ok(opts) => opts,
            Err(_) => return None,
        };
        let mut conn = match Conn::new(opts) {
            Ok(conn) => conn,
            Err(_) => return None,
        };
        if let Err(_) = conn.query_drop("SET NAMES 'utf8'") {
            return None;
        }
        if let Err(_) = conn.query_drop("SET CHARACTER SET 'utf8'") {
            return None;
        }
        if let Err(_) = conn.query_drop("SET SESSION collation_connection = 'utf8_general_ci'") {
            return None;
        }

        Some(DB { 
            conn,
            log: None,
         })
    }

    pub fn query<T>(&mut self, text: &str) -> Option<Vec<T>>
    where
        T: FromRow,
    {
        let result = match self.conn.query(text) {
            Ok(result) => Some(result),
            Err(e) => {
                let result = format!("sql: {}\nErr: {}", text, e.to_string());
                if let Some(l) = &self.log {
                    let log = RwLock::read(l).unwrap();
                    log.write(602, &result);
                }
                None
            },
        };
        result
    }
}

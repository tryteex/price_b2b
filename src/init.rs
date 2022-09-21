use std::{env, fs::{read_to_string, create_dir}, path::Path};

use serde_json::Value;

use crate::{log::Log, db::DB};

#[derive(Debug, PartialEq)]
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

    pub auth_company_capacity: usize,
    pub auth_user_capacity: usize,
    pub country_capacity: usize,
    pub target_capacity: usize,
    pub lock_capacity: usize,
    pub lock_item_capacity: usize,
    pub product_capacity: usize,
    pub bonus_company_capacity: usize,
    pub bonus_group_capacity: usize,
    pub stock_capacity: usize,
    pub stock_product_capacity: usize,
    pub category_capacity: usize,
    pub file_buffer_capacity: usize,
    pub file_flush_buffer_capacity: usize,
    pub small_buffer_capacity: usize,
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
                None => log.exit(141, ""),
            },
            None => log.exit(140, ""),
        };

        let mut db = match DB::simple(&db_b2b) {
            Some(db) => db,
            None => log.exit(180, ""),
        };
        let mut dbl = match DB::simple(&db_log) {
            Some(db) => db,
            None => log.exit(180, ""),
        };

        let mut auth_company_capacity: usize = 0;
        let mut auth_user_capacity: usize = 0;
        let mut country_capacity: usize = 0;
        let mut target_capacity: usize = 0;
        let mut lock_capacity: usize = 0;
        let mut lock_item_capacity: usize = 0;
        let mut product_capacity: usize = 0;
        let mut bonus_company_capacity: usize = 0;
        let mut bonus_group_capacity: usize = 0;
        let mut stock_capacity: usize = 0;
        let mut stock_product_capacity: usize = 0;
        let mut category_capacity: usize = 0;

        if app == AppInit::Go {
            if cfg!(debug_assertions) {
                println!("{} Start load init", chrono::Local::now().format("%Y.%m.%d %H:%M:%S%.9f").to_string());
            }
            let sql = "
                SELECT count(DISTINCT u.companyID)
                FROM users u INNER JOIN companies c ON c.companyID=u.companyID
                WHERE 
                    c.profilesID <> 0 AND c.status='registered' 
                    AND CONCAT(';', c.ApiPermissions, ';') LIKE '%;37;%'
                    AND u.roleID > 0 AND u.status NOT IN ('blocked', 'deleted')
            ";
            match db.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            auth_company_capacity = 100 + *val as usize;
                        },
                        None => log.exit(181, sql),
                    };
                },
                None => log.exit(181, sql),
            }

            let sql = "
                SELECT count(u.userID)
                FROM users u INNER JOIN companies c ON c.companyID=u.companyID
                WHERE 
                    c.profilesID <> 0 AND c.status='registered' 
                    AND CONCAT(';', c.ApiPermissions, ';') LIKE '%;37;%'
                    AND u.roleID > 0 AND u.status NOT IN ('blocked', 'deleted')
                GROUP BY c.companyID
                ORDER BY count(u.userID) DESC
                LIMIT 1
            ";
            match db.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            auth_user_capacity = 5 + *val as usize;
                        },
                        None => log.exit(181, sql),
                    };
                },
                None => log.exit(181, sql),
            }

            let sql = "
                SELECT count(*) FROM delivery_country
            ";
            match dbl.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            country_capacity = 5 + *val as usize;
                        },
                        None => log.exit(183, sql),
                    };
                },
                None => log.exit(183, sql),
            }

            let sql = "
                SELECT
                    count(*)
                FROM
                    delivery_targets t
                LEFT JOIN delivery_targets_postages p ON p.targetID=t.targetID
                WHERE
                    p.client='brain_b2b'
            ";
            match dbl.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            target_capacity = 20 + *val as usize;
                        },
                        None => log.exit(183, sql),
                    };
                },
                None => log.exit(183, sql),
            }

            let sql = "
                SELECT COUNT(DISTINCT t.companyID)
                FROM (
                    SELECT companyID, vendorID, ProductGroupID, classID, 0 FROM lockable_products 
                    UNION ALL
                    SELECT companyID, 0, 0, 0, productID FROM lockable_products_detailed 
                ) t
            ";
            match db.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            lock_capacity = 100 + *val as usize;
                        },
                        None => log.exit(181, sql),
                    };
                },
                None => log.exit(181, sql),
            }

            let sql = "
                SELECT COUNT(*)
                FROM (
                    SELECT companyID, vendorID, ProductGroupID, classID, 0 FROM lockable_products 
                    UNION ALL
                    SELECT companyID, 0, 0, 0, productID FROM lockable_products_detailed 
                ) t
                GROUP BY companyID
                ORDER BY COUNT(*) DESC
                LIMIT 1
            ";
            match db.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            lock_item_capacity = 100 + *val as usize;
                        },
                        None => log.exit(181, sql),
                    };
                },
                None => log.exit(181, sql),
            }

            let sql = "
                SELECT COUNT(*)
                FROM 
                    SC_products p
                    LEFT JOIN SC_vendors v ON v.vendorID = p.vendorID
                WHERE p.enabled=1 AND p.Price6 > 0 AND (p.statusnew=0 OR p.statusnew=4 OR p.statusnew IS NULL) AND p.isarchive=0 AND p.isdiler=1
            ";
            match db.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            product_capacity = 10000 + *val as usize;
                        },
                        None => log.exit(181, sql),
                    };
                },
                None => log.exit(181, sql),
            }

            let sql = "
                SELECT COUNT(DISTINCT companyID) FROM companies_bonuses
            ";
            match db.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            bonus_company_capacity = 100 + *val as usize;
                        },
                        None => log.exit(181, sql),
                    };
                },
                None => log.exit(181, sql),
            }

            let sql = "
                SELECT COUNT(*) FROM companies_bonuses GROUP BY companyID ORDER BY COUNT(*) DESC LIMIT 1
            ";
            match db.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            bonus_group_capacity = 5 + *val as usize;
                        },
                        None => log.exit(181, sql),
                    };
                },
                None => log.exit(181, sql),
            }

            let sql = "
                SELECT COUNT(DISTINCT stockid)
                FROM delivery_product_time 
                WHERE receipt_time <> '0000-00-00 00:00:00'
            ";
            match dbl.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            stock_capacity = 5 + *val as usize;
                        },
                        None => log.exit(183, sql),
                    };
                },
                None => log.exit(183, sql),
            }

            let sql = "
                SELECT count(*)
                FROM (
                    SELECT stockid, product_code
                    FROM delivery_product_time 
                    WHERE available>0 AND receipt_time <> '0000-00-00 00:00:00' AND (receipt_time-CURRENT_TIMESTAMP) < 0
                    
                    UNION ALL
                    
                    SELECT stockid, product_code
                    FROM delivery_product_time 
                    WHERE receipt_time <> '0000-00-00 00:00:00'
                )t
                GROUP BY stockid
                ORDER BY count(*) DESC
            ";
            match dbl.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            stock_product_capacity = 10000 + *val as usize;
                        },
                        None => log.exit(183, sql),
                    };
                },
                None => log.exit(183, sql),
            }

            let sql = "
                SELECT COUNT(categoryid) FROM SC_categories where disabled=0 ORDER BY sort_order
            ";
            match db.query(sql) {
                Some(result) => {
                    let row: Vec<u32> = result;
                    match row.get(0) {
                        Some(val) => {
                            category_capacity = 20 + *val as usize;
                        },
                        None => log.exit(181, sql),
                    };
                },
                None => log.exit(181, sql),
            }
            if cfg!(debug_assertions) {
                println!("{} Finish load init", chrono::Local::now().format("%Y.%m.%d %H:%M:%S%.9f").to_string());
            }
        }

        Init {
            app, port, irc, time_zone, max: max.into(), db_log, db_b2b, db_local, dir: dir.to_owned(), salt,

            auth_company_capacity,
            auth_user_capacity,
            country_capacity,
            target_capacity,
            lock_capacity,
            lock_item_capacity,
            product_capacity,
            bonus_company_capacity,
            bonus_group_capacity,
            stock_capacity,
            stock_product_capacity, 
            category_capacity,
            file_buffer_capacity: 10000000,
            file_flush_buffer_capacity: 9900000,
            small_buffer_capacity: 32768, 
        }
    }
}
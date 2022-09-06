use std::{sync::{Mutex, Arc, RwLock}, collections::HashMap, fs::{remove_file, read_to_string}, path::Path};

use crate::{worker::Worker, param::{Format, Param, PriceVolume}, cache::Cache, log::Log, init::Init};
use crate::PRODUCT_CAPACITY;

use chrono::{NaiveDateTime, Local, TimeZone, Duration};
use glob::glob;

#[derive(Debug)]
pub struct Price {
    worker: Arc<Mutex<Worker>>,
    items: Vec<PriceItem>,
}

#[derive(Debug)]
pub struct PriceItem {
    id: u32,
    code: String,
    stock: u32,
    available: u32,
    dayDelivery: u32,
    bg: String,
    bonusOpt: f32,
    bonus: f32,
    priceUSD: f32,
    priceInd: u32,
    recommendedPrice: f32,
    retailPrice: f32,
    internetPrice: f32,
    weight: f32,
    volume: f32,
    overall: u32,
    costDelivery: f32,
    categoryID: u32,
    group: String,
    articul: String,
    vendor: String,
    model: String,
    name: String,
    description: String,
    categoryName: String,
    ddp: u32,
    warranty: u32,
    note: String,
    url: String,
    uktved: String,
    groupID: u32,
    classID: u32,
    className: String,
    countryID: u32,
    country: String,
    is_exclusive: u32,
    lock: bool,
    EAN: String,
    fop: u32,
    priceUAH: f32,
}

impl Price {
    pub fn new(worker: Arc<Mutex<Worker>>) -> Price {
        Price {
            worker,
            items: Vec::with_capacity(PRODUCT_CAPACITY),
        }
    }
    
    pub fn calc(&self, param: &HashMap<String, String>) -> Vec<u8> {
        let log;
        let init;
        let cache;
        {
            let w = Mutex::lock(&self.worker).unwrap();
            if w.stop {
                return Vec::new();
            }
            log = Arc::clone(&w.log);
            init = Arc::clone(&w.init);
            cache = Arc::clone(&w.cache);
        }
        let param = match Param::new(param, Arc::clone(&init), Arc::clone(&log)) {
            Ok(param) => param,
            Err(err) => {
                let text = err.as_bytes();
                let mut answer = format!("HTTP/1.1 401 Unauthorized\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n", text.len()).as_bytes().to_vec();
                answer.extend_from_slice(&text[..]);
                return answer;
            },
        };

        let (corp, rozn, r3, profile_id) = match self.check_auth(&param, Arc::clone(&cache), Arc::clone(&log)) {
            Ok((corp, rozn, r3, profile_id)) => (corp, rozn, r3, profile_id),
            Err(err) => {
                let text = err.as_bytes();
                let mut answer = format!("HTTP/1.1 401 Unauthorized\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n", text.len()).as_bytes().to_vec();
                answer.extend_from_slice(&text[..]);
                return answer;
            },
        };

        let file = match self.get_file_name(&param, Arc::clone(&init), Arc::clone(&log)) {
            Ok(file) => file,
            Err(err) => {
                let text = err.as_bytes();
                let mut answer = format!("HTTP/1.1 401 Unauthorized\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n", text.len()).as_bytes().to_vec();
                answer.extend_from_slice(&text[..]);
                return answer;
            },
        };

        let mut answer: Vec<String> = Vec::with_capacity(16);
        let text = match self.get_price(&param, &file, corp, rozn, r3, profile_id) {
            Ok(text) => text,
            Err(err) => {
                let text = err.as_bytes();
                let mut answer = format!("HTTP/1.1 401 Unauthorized\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n", text.len()).as_bytes().to_vec();
                answer.extend_from_slice(&text[..]);
                return answer;
            },
        };

        answer.push("HTTP/1.1 200 OK\r\n".to_owned());
        match param.format {
            Format::XLSX => {
                answer.push("Content-Type: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet\r\n".to_owned());
            },
            Format::XML => {
                answer.push("Content-Type: application/xml\r\n".to_owned());
            },
            Format::JSON => {
                answer.push("Content-Type: application/json\r\n".to_owned());
            },
            Format::PHP => {
                answer.push("Content-Type: application/vnd.php.serialized\r\n".to_owned());
            },
        };
        let file = Path::new(&file).file_name().unwrap().to_str().unwrap().to_owned();
        answer.push(format!("Content-Disposition: attachment; filename=\"{}\"\r\n", file));
        answer.push(format!("Content-Length: {}\r\n\r\n", text.len()));
    
        let mut answer = answer.join("").into_bytes();
        answer.extend_from_slice(&text[..]);

        answer
    }

    fn get_file_name(&self, param: &Param, init: Arc<RwLock<Init>>, log: Arc<RwLock<Log>>) -> Result<String, String> {
        let log = RwLock::read(&log).unwrap();
        let init = RwLock::read(&init).unwrap();
        
        let path = format!("{}/cache/price_{}_{}_{}_{}_{}_{}_*.{}", init.dir, param.company_id, param.user_id, param.target_id, param.lang_str, param.volume_str, param.pc_vinga_str, param.format_str);
        let fls = match glob(&path) {
            Ok(fls) => fls,
            Err(_) => return Err(log.client_err(20)),
        };
        let mut log_cache = false;
        let mut filename: Option<String> = None;
        let now = Local::now();
        for entry in fls {
            if let Ok(fl) = entry {
                if log_cache {
                    if let Err(_) = remove_file(fl) {
                        return Err(log.client_err(21));
                    }
                } else {
                    let file = fl.display().to_string();
                    let parts: Vec<&str> = file.splitn(9, "_").collect();
                    if parts.len() == 9 {
                        let dt = &parts[8][..15];
                        match NaiveDateTime::parse_from_str(dt, "%Y%m%d_%H%M%S") {
                            Ok(tm) => {
                                let tm = Local.from_local_datetime(&tm).unwrap() + Duration::minutes(30);
                                if tm > now {
                                    filename = Some(file);
                                    log_cache = true;
                                } else {
                                    if let Err(_) = remove_file(fl) {
                                        return Err(log.client_err(21));
                                    }
                                }
                            },
                            Err(_) => {
                                if let Err(_) = remove_file(fl) {
                                    return Err(log.client_err(21));
                                }
                            },
                        };
                    } else {
                        if let Err(_) = remove_file(fl) {
                            return Err(log.client_err(21));
                        }
                    }
                }
            }
        }
        let file;
        if let None = filename {
            file = format!("{}/cache/price_{}_{}_{}_{}_{}_{}_{}.{}", init.dir, param.company_id, param.user_id, param.target_id, param.lang_str, param.volume_str, param.pc_vinga_str, now.format("%Y%m%d_%H%M%S").to_string(), param.format_str);
        } else if param.user_id == 449093 {
            file = format!("{}/cache/price_{}_{}_{}_{}_{}_{}_{}.{}", init.dir, param.company_id, param.user_id, param.target_id, param.lang_str, param.volume_str, param.pc_vinga_str, now.format("%Y%m%d_%H%M%S").to_string(), param.format_str);
        } else {
            file = filename.unwrap();
        }
        Ok(file)
    }

    fn check_auth(&self, param: &Param, cache: Arc<Mutex<Cache>>, log: Arc<RwLock<Log>>) -> Result<(bool, bool, bool, u32), String> {
        let log = RwLock::read(&log).unwrap();
        let auth;
        {
            let c = Mutex::lock(&cache).unwrap();
            auth = Arc::clone(&c.auth);
        }
        let corp: bool;
        let profile_id;
        let mut rozn: bool;
        let mut r3: bool;
        {
            let a = Mutex::lock(&auth).unwrap();
            match a.company.get(&param.company_id) {
                Some(company) => match company.users.get(&param.user_id) {
                    Some(user) => {
                        if user.profiles_id == 0 {
                            return Err(log.client_err(19));
                        }
                        profile_id = user.profiles_id;
                        corp = user.corp;
                        rozn = user.rozn;
                        r3 = user.r3;
                    },
                    None => return Err(log.client_err(18)),
                },
                None => return Err(log.client_err(17)),
            };
        }
        if param.api {
            rozn = true;
            r3 = true;
        }

        Ok((corp, rozn, r3, profile_id))
    }

    fn get_price(&self, param: &Param, file: &str, corp: bool, rozn: bool, r3: bool, profile_id: u32) -> Result<Vec<u8>, String> {
        let log;
        {
            let w = Mutex::lock(&self.worker).unwrap();
            log = Arc::clone(&w.log);
        }
        let log = RwLock::read(&log).unwrap();

        let path = Path::new(file);
        if path.exists() {
            match read_to_string(file) {
                Ok(res) => return Ok(res.as_bytes().to_vec()),
                Err(_) => {
                    if let Err(_) = remove_file(file) {
                        return Err(log.client_err(22));
                    }
                },
            };
        }

        let mut target_id = param.target_id;
        
        if target_id == 0 {
            return Err(log.client_err(23));
        }
        if param.volume == PriceVolume::FullUAH {
          target_id = 29;
        }
        let hostname = if corp { "corp.brain.com.ua" } else { "opt.brain.com.ua" };


        
        Ok("test".as_bytes().to_vec())
    }

}
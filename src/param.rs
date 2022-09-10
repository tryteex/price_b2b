use std::{collections::HashMap, sync::{Arc, RwLock}};

use sha2::{Sha512, Digest};
use urlencoding::decode;

use crate::{log::Log, init::Init};

#[derive(Debug)]
pub enum Format {
    XLSX,
    XML,
    JSON,
    PHP,
}

pub enum Lang {
    UA,
    RU,
}

#[derive(PartialEq)]
pub enum PriceVolume {
    Local,
    Full,
    Short,
    FullUAH,
}

pub struct Param {
    // time: u32,
    pub user_id: u32,
    pub company_id: u32,
    pub target_id: u32,
    pub format_str: String,
    pub format: Format,
    pub lang: Lang,
    pub lang_str: String,
    // token: String,
    pub pc_vinga: bool,
    pub pc_vinga_str: String,
    pub volume: PriceVolume,
    pub volume_str: String,
    pub uah: bool,
    pub nds_orig: bool,
    pub nds: bool,
    pub round: bool,
    pub ean: bool,
    pub api: bool,

    // log: Arc<RwLock<Log>>,
}

impl Param {

    pub fn new(param: &HashMap<String, String>, init: Arc<RwLock<Init>>, log: Arc<RwLock<Log>>) -> Result<Param, String> {
        let log_read = RwLock::read(&log).unwrap();
        let init_read = RwLock::read(&init).unwrap();
        let price_param = Param::get_price_param(param);
        let format_str;
        let format = match price_param.get("format") {
            Some(format) => {
                format_str = format.clone();
                match format as &str {
                    "xlsx" => Format::XLSX,
                    "xml" => Format::XML,
                    "json" => Format::JSON,
                    "php" => Format::PHP,
                    _ => return Err(log_read.client_err(2))
                }
            },
            None => return Err(log_read.client_err(1)),
        };
        let volume_str;
        let volume = match price_param.get("full") {
            Some(format) => {
                let f = match format as &str { 
                    "0" => PriceVolume::Local,
                    "1" => PriceVolume::Full,
                    "2" => PriceVolume::Short,
                    "3" => PriceVolume::FullUAH,
                    _ => return Err(log_read.client_err(4))
                };
                volume_str = format.clone();
                f
            },
            None => return Err(log_read.client_err(3)),
        };
        let uah = match price_param.get("cur") {
            Some(uah) => match uah as &str {
                "uah" => true,
                _ => false,
            },
            None => false,
        };
        let ean = match price_param.get("ean") {
            Some(ean) => match ean as &str {
                "1" => true,
                _ => false,
            },
            None => false,
        };
        let nds = match price_param.get("nds") {
            Some(nds) => match nds as &str {
                "1" => if uah { true } else { false },
                _ => false,
            },
            None => false,
        };
        let nds_orig = nds;
        let round = if nds && uah { true } else { false };
        let company_id: u32 = match price_param.get("companyID") {
            Some(company_id) => match company_id.parse() {
                Ok(company_id) => company_id,
                Err(_) => return Err(log_read.client_err(6)),
            },
            None => return Err(log_read.client_err(5)),
        };
        let target_id: u32 = match price_param.get("targetID") {
            Some(target_id) => match target_id.parse() {
                Ok(target_id) => target_id,
                Err(_) => return Err(log_read.client_err(8)),
            },
            None => return Err(log_read.client_err(7)),
        };
        let lang_str;
        let lang = match price_param.get("lang") {
            Some(lang) => {
                lang_str = lang.clone();
                match lang as &str {
                    "ua" => Lang::UA,
                    "ru" => Lang::RU,
                    _ => return Err(log_read.client_err(10))
                }
            },
            None => return Err(log_read.client_err(9)),
        };
        let time: u32 = match price_param.get("time") {
            Some(time) => match time.parse() {
                Ok(time) => time,
                Err(_) => return Err(log_read.client_err(12)),
            },
            None => return Err(log_read.client_err(11)),
        };
        let user_id: u32 = match price_param.get("userID") {
            Some(user_id) => match user_id.parse() {
                Ok(user_id) => user_id,
                Err(_) => return Err(log_read.client_err(14)),
            },
            None => return Err(log_read.client_err(13)),
        };
        let pc_vinga_str;
        let pc_vinga = match price_param.get("pcvinga") {
            Some(pc_vinga) => match pc_vinga as &str {
                "1" => {
                    pc_vinga_str = "1".to_owned();
                    true
                },
                _ => {
                    pc_vinga_str = "0".to_owned();
                    false
                },
            },
            None => {
                pc_vinga_str = "0".to_owned();
                false
            },
        };
        let api = match price_param.get("api") {
            Some(_) => true,
            None => false,
        };
        let token = match price_param.get("token") {
            Some(token) => token.clone(),
            None => return Err(log_read.client_err(15)),
        };
        let mut hasher = Sha512::new();
        hasher.update(format!("{}{}{}{}{}{}", company_id, target_id, format_str, lang_str, time, init_read.salt).as_bytes());
        let result = format!("{:#x}", hasher.finalize());
        if result != token {
            return Err(log_read.client_err(16))
        }

        Ok(Param {
            // time,
            user_id,
            company_id,
            target_id,
            format_str,
            format,
            lang,
            lang_str,
            // token,
            pc_vinga,
            pc_vinga_str,
            volume,
            volume_str,
            uah,
            nds,
            nds_orig,
            round,
            ean,
            api,

            // log: Arc::clone(&log),
        })
    }

    fn get_price_param(param: &HashMap<String, String>) -> HashMap<String, String> {
        let mut get: HashMap<String, String> = HashMap::with_capacity(16);
        let key = "QUERY_STRING";
        if param.contains_key(key) {
            let val = param.get(key).unwrap();
            let gets:Vec<&str> = val.split("&").collect();
            for v in gets {
                let key: Vec<&str> = v.splitn(2, "=").collect();
                match key.len() {
                    1 => get.insert(decode(v).unwrap_or_default().to_string(), "".to_owned()),
                    _ => get.insert(decode(key[0]).unwrap_or_default().to_string(), decode(key[1]).unwrap_or_default().to_string()),
                };
            }
        }
        get
    }
}
use std::{collections::HashMap, fs::{File, rename, read}, io::Write, sync::{RwLock, Arc}};

use crate::{price::{PriceItem, Show, ValueType}, param::PriceVolume, init::Init};

pub struct FormatJSON { }

impl FormatJSON {

    fn escape_json(val: &str) -> String {
        let mut new = val.replace("\\", "\\\\");
        new = new.replace("\"", "\\\"");
        new = new.replace("/", "\\/");
        new = new.replace("\t", "\\t");
        new = new.replace("\n", "\\n");
        new = new.replace("\r", "\\r");
        new
    }

    pub fn make(items: &HashMap<u32, PriceItem>, filename: &str, volume: &PriceVolume, rozn: bool, r3: bool, ean: bool, init: Arc<RwLock<Init>>) -> Option<Vec<u8>> {
        let init_read = RwLock::read(&init).unwrap();

        let mut show = Show::new();
        match volume {
            PriceVolume::Local => {
                for item in &mut show.list {
                    if item.local || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                    }
                }
            },
            PriceVolume::Full => {
                for item in &mut show.list {
                    if item.full || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                    }
                }
            },
            PriceVolume::Short => {
                for item in &mut show.list {
                    if item.short || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                    }
                }
            },
            PriceVolume::FullUAH => {
                for item in &mut show.list {
                    if item.full_uah || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                    }
                }
            },
        }
        let tmp = format!("{}.tmp", filename);
        let path = std::path::Path::new(&tmp);
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut data = String::with_capacity(init_read.file_buffer_capacity);
        data.push_str("{");
        for (_, price) in items {
            data.push_str(&format!("\"{}\":{{", price.id));
            for item in &show.list {
                if let Some(_) = &item.index {
                    match (item.get)(price) {
                        ValueType::String(v) => data.push_str(&format!("\"{}\":\"{}\",", FormatJSON::escape_json(&item.name), FormatJSON::escape_json(v))),
                        ValueType::Money(v) => data.push_str(&format!("\"{}\":{:.2},", FormatJSON::escape_json(&item.name), v)),
                        ValueType::Index(v) => data.push_str(&format!("\"{}\":{},", FormatJSON::escape_json(&item.name), v)),
                    }
                }
            }
            data.pop();
            if data.len() > init_read.file_flush_buffer_capacity {
                if let Err(_) = file.write_all(data.as_bytes()) {
                    return None;
                }
                data.clear();
            }
            data.push_str("},");
        }
        data.pop();
        data.push_str("}");
        if let Err(_) = file.write_all(data.as_bytes()) {
            return None;
        }
        if let Err(_) = rename(&tmp, filename) {
            return None;
        }
        let path = std::path::Path::new(filename);
        match read(path) {
            Ok(str) => Some(str),
            Err(_) => None,
        }
    }
}
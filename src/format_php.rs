use std::{collections::HashMap, fs::{File, rename, read}, io::Write};

use crate::{price::{PriceItem, Show, ValueType}, param::PriceVolume};

pub struct FormatPHP { }

impl FormatPHP {

    pub fn make(items: &HashMap<u32, PriceItem>, filename: &str, volume: &PriceVolume, rozn: bool, r3: bool, ean: bool) -> Option<Vec<u8>> {
        let mut show = Show::new();
        let mut col: u32 = 0;
        match volume {
            PriceVolume::Local => {
                for item in &mut show.list {
                    if item.local || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                        col += 1;
                    }
                }
            },
            PriceVolume::Full => {
                for item in &mut show.list {
                    if item.full || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                        col += 1;
                    }
                }
            },
            PriceVolume::Short => {
                for item in &mut show.list {
                    if item.short || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                        col += 1;
                    }
                }
            },
            PriceVolume::FullUAH => {
                for item in &mut show.list {
                    if item.full_uah || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                        col += 1;
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
        let mut data = String::with_capacity(10000000);
        data.push_str(&format!("a:{}:{{", items.len()));
        for (_, price) in items {
            data.push_str(&format!("i:{};a:{}:{{", price.id, col));
            for item in &show.list {
                if let Some(_) = &item.index {
                    match (item.get)(price) {
                        ValueType::String(val) => data.push_str(&format!("s:{}:\"{}\";s:{}:\"{}\";", item.name.len(), item.name, val.len(), val)),
                        ValueType::Money(v) => data.push_str(&format!("s:{}:\"{}\";i:{};", item.name.len(), item.name, v)),
                        ValueType::Index(v) => data.push_str(&format!("s:{}:\"{}\";d:{:.2};", item.name.len(), item.name, v)),
                    }
                    if data.len() > 9900000 {
                        if let Err(_) = file.write_all(data.as_bytes()) {
                            return None;
                        }
                        data.clear();
                    }
                }
            }
            data.push_str("}");
        }
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
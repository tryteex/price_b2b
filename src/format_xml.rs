use std::{collections::HashMap, sync::{Arc, RwLock}, fs::{File, rename, read}, io::Write};

use crate::{price::{PriceItem, Show, ValueType}, param::{PriceVolume, Lang}, init::Init, db::DB, log::Log, CATEGORY_CAPACITY, FILE_BUFFER_CAPACITY, SMALL_BUFFER_CAPACITY, FILE_FLUSH_BUFFER_CAPACITY};

pub struct Category {
    id: u32,
    name: String,
    parent_id: u32,
}

pub struct FormatXml { }

impl FormatXml {
    fn escape_xml(val: &str) -> String {
        let mut new = val.replace("&", "&amp;");
        new = new.replace("\"", "&quot;");
        new = new.replace("'", "&apos;");
        new = new.replace("<", "&lt;");
        new = new.replace(">", "&gt;");
        new
    }

    pub fn make(items: &HashMap<u32, PriceItem>, filename: &str, volume: &PriceVolume, rozn: bool, r3: bool, ean: bool, lang: &Lang, init: Arc<RwLock<Init>>, log: Arc<RwLock<Log>>) -> Option<Vec<u8>> {
        let mut show = Show::new();
        let cat;
        match volume {
            PriceVolume::Local => {
                cat = match FormatXml::get_categories(lang, init, log) {
                    Some(cat) => cat,
                    None => return None,
                };
                for item in &mut show.list {
                    if item.local || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                    }
                }
            },
            PriceVolume::Full => {
                cat = match FormatXml::get_categories(lang, init, log) {
                    Some(cat) => cat,
                    None => return None,
                };
                for item in &mut show.list {
                    if item.full || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                    }
                }
            },
            PriceVolume::Short => {
                cat = "".to_owned();
                for item in &mut show.list {
                    if item.short || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = Some("".to_owned());
                    }
                }
            },
            PriceVolume::FullUAH => {
                cat = "".to_owned();
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
        let mut data = String::with_capacity(FILE_BUFFER_CAPACITY);
        data.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<price>");
        data.push_str(&cat);
        data.push_str("<products>");
        for (_, price) in items {
            data.push_str("<product");
            for item in &show.list {
                if let Some(_) = &item.index {
                    match (item.get)(price) {
                        ValueType::String(v) => data.push_str(&format!(" {}=\"{}\"", FormatXml::escape_xml(&item.name), FormatXml::escape_xml(v))),
                        ValueType::Money(v) => data.push_str(&format!(" {}=\"{:.2}\"", FormatXml::escape_xml(&item.name), v)),
                        ValueType::Index(v) => data.push_str(&format!(" {}=\"{}\"", FormatXml::escape_xml(&item.name), v)),
                    }
                    if data.len() > FILE_FLUSH_BUFFER_CAPACITY {
                        if let Err(_) = file.write_all(data.as_bytes()) {
                            return None;
                        }
                        data.clear();
                    }
                }
            }
            data.push_str("/>");
        }
        data.push_str("</products>");
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

    fn get_categories(lang: &Lang, init: Arc<RwLock<Init>>, log: Arc<RwLock<Log>>) -> Option<String> {
        let init = RwLock::read(&init).unwrap();
        let mut db = match DB::new(&init.db_b2b, Arc::clone(&log)) {
            Some(db) => db,
            None => return None,
        };
        let sql = match lang {
            Lang::UA => "SELECT categoryid as id, name_ua as name, parent FROM SC_categories where disabled=0 ORDER BY sort_order",
            Lang::RU => "SELECT categoryid as id, name_ru as name, parent FROM SC_categories where disabled=0 ORDER BY sort_order",
        };
        match db.query(&sql) {
            Some(result) => {
                let row: Vec<(u32, String, u32)> = result;
                let mut data = String::with_capacity(FILE_BUFFER_CAPACITY);
                let mut cat: Vec<Category> = Vec::with_capacity(CATEGORY_CAPACITY);
                data.push_str("<categories>");
                for (category_id, name, parent_id) in row {
                    cat.push(Category { id: category_id, name, parent_id });
                }
                for c in &cat {
                    if c.parent_id == 1 {
                        let ct = FormatXml::build_tree_cat(&cat, c.id);
                        if ct.len() == 0 {
                            data.push_str(&format!("<category id=\"{}\" name=\"{}\"/>", c.id, FormatXml::escape_xml(&c.name)));
                        } else {
                            data.push_str(&format!("<category id=\"{}\" name=\"{}\">{}</category>", c.id, FormatXml::escape_xml(&c.name), ct));
                        }
                    }
                }
                data.push_str("</categories>");
                data.shrink_to_fit();
                Some(data)
            },
            None => None,
        }
    }

    fn build_tree_cat(cat: &Vec<Category>, id: u32) -> String {
        let mut data = String::with_capacity(SMALL_BUFFER_CAPACITY);
        for c in cat {
            if c.parent_id == id {
                let ct = FormatXml::build_tree_cat(&cat, c.id);
                if ct.len() == 0 {
                    data.push_str(&format!("<subcategory id=\"{}\" name=\"{}\"/>", c.id, FormatXml::escape_xml(&c.name)));
                } else {
                    data.push_str(&format!("<subcategory id=\"{}\" name=\"{}\">{}</subcategory>", c.id, FormatXml::escape_xml(&c.name), ct));
                }
            }
        }
        data.shrink_to_fit();
        data
    }

}
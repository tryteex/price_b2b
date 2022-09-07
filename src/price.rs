use std::{sync::{Mutex, Arc, RwLock}, collections::HashMap, fs::{remove_file, read_to_string}, path::Path};

use crate::{worker::Worker, param::{Format, Param, PriceVolume, Lang}, cache::Cache, log::Log, init::Init, data::{Product, LockList, Target, ProductStock, BonusGroup, Country}, db::DB};
use crate::PRODUCT_CAPACITY;

use chrono::{NaiveDateTime, Local, TimeZone, Duration};
use glob::glob;

#[derive(Debug)]
pub struct Price {
    worker: Arc<Mutex<Worker>>,
    items: HashMap<u32, PriceItem>,
}

#[derive(Debug)]
pub struct PriceItem {
    id: u32,
    code: String,
    stock: u32,
    available: u32,
    day_delivery: u32,
    bg: String,
    bonusOpt: f32,
    bonus: f32,
    price_usd: f32,
    price_ind: u32,
    recommended_price: f32,
    retail_price: f32,
    internet_price: f32,
    weight: f32,
    volume: f32,
    overall: u32,
    cost_delivery: f32,
    categoryID: u32,
    group: String,
    articul: String,
    vendor: String,
    model: String,
    name: String,
    description: String,
    category_name: String,
    ddp: u32,
    warranty: String,
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
    price_UAH: f32,
}

impl PriceItem {
    pub fn new(product_id: u32, param: &Param, p: Product, l: &LockList, s: &ProductStock, b: &BonusGroup, t: &Target, h: &str, c: &HashMap<u32, Country>) -> Option<PriceItem> {
        let fop: u32 = if p.seller.len() > 0 { 1 } else { 0 };
        let lock = if l.list.len() == 0 {
            false
        } else if l.list.contains_key(&format!("0:0:0:{}", product_id)) {
            true
        } else if l.list.contains_key(&format!("{}:0:0:0", p.vendor_id)) {
            true
        } else if l.list.contains_key(&format!("0:{}:0:0", p.group_id)) {
            true
        } else if l.list.contains_key(&format!("0:0:{}:0", p.class_id)) {
            true
        } else if l.list.contains_key(&format!("{}:{}:0:0", p.vendor_id, p.group_id)) {
            true
        } else if l.list.contains_key(&format!("{}:0:{}:0", p.vendor_id, p.class_id)) {
            true
        } else if l.list.contains_key(&format!("0:{}:{}:0", p.group_id, p.class_id)) {
            true
        } else if l.list.contains_key(&format!("{}:{}:{}:0", p.vendor_id, p.group_id, p.class_id)) {
            true
        } else {
            false
        };
        let stock;
        let available;
        let day_delivery;
        if lock {
            stock = 0;
            available = 0;
            day_delivery = 0;
        } else {
            match s.product.get(&product_id) {
                Some(st) => {
                    if st.available > 0 {
                        stock = 1;
                        available = st.available;
                        day_delivery = 0;
                    } else {
                        stock = 0;
                        available = 0;
                        day_delivery = st.day;
                    }
                },
                None => {
                    stock = 0;
                    available = 0;
                    day_delivery = 0;
                },
            }
        }
        if stock == 0 && (param.volume == PriceVolume::Local || param.volume == PriceVolume::FullUAH) {
            return None;
        }
        if stock == 0 && day_delivery == 0 && !lock {
            return  None;
        }
        let bonus = if lock {
            0.0
        } else if b.groups.contains_key(&p.bg) {
            p.bonus
        } else {
            0.0
        };
        let overall: u32 = if p.overall > 3 { 3 } else if p.overall < 0 { 3 } else { p.overall.try_into().unwrap() };
        let cost_delivery = if p.weight > 250.0 * p.volume {
            p.weight
        } else {
            250.0 * p.volume
        } * match overall {
            0 => t.postage_compact,
            1 => t.postage_middle,
            2 => t.postage_big,
            _ => t.postage_large,
        };
        let ddp: u32 = if p.ddp {
            1
        } else {
            0
        };
        let group; 
        let name;
        let description;
        let category_name;
        let url;
        let class;
        let country;
        match param.lang {
            Lang::UA => {
                group = p.group_ua;
                name = p.ua;
                description = p.desc_ua;
                category_name = p.category_ua;
                url = format!("https://{}/{}.html", h, p.url_ua);
                class = p.class_ua;
                country = match c.get(&p.country_id) {
                    Some(c) => c.ua.clone(),
                    None => "".to_owned(),
                };
            },
            Lang::RU => {
                group = p.group_ru;
                name = p.ru;
                description = p.desc_ru;
                category_name = p.category_ru;
                url = format!("https://{}/{}.html", h, p.url_ru);
                class = p.class_ru;
                country = match c.get(&p.country_id) {
                    Some(c) => c.ru.clone(),
                    None => "".to_owned(),
                };
            },
        }

        Some(PriceItem {
            id: product_id,
            code: p.code,
            stock,
            available,
            day_delivery,
            bg: p.bg,
            bonusOpt: p.bonus,
            bonus: bonus,
            price_usd: 0.0,
            price_ind: 0,
            recommended_price: 0.0,
            retail_price: 0.0,
            internet_price: 0.0,
            weight: p.weight,
            volume: p.volume,
            overall,
            cost_delivery,
            categoryID: p.category_id,
            group,
            articul: p.article,
            vendor: p.vendor,
            model: p.model,
            name,
            description,
            category_name,
            ddp,
            warranty: p.warranty,
            note: "".to_owned(),
            url,
            uktved: p.uktved,
            groupID: p.group_id,
            classID: p.class_id,
            className: class,
            countryID: p.country_id,
            country,
            is_exclusive: p.is_exclusive,
            lock,
            EAN: p.ean,
            fop,
            price_UAH: 0.0,
        })
    }
}

impl Price {
    pub fn new(worker: Arc<Mutex<Worker>>) -> Price {
        Price {
            worker,
            items: HashMap::with_capacity(PRODUCT_CAPACITY),
        }
    }
    
    pub fn calc(&mut self, param: &HashMap<String, String>) -> Vec<u8> {
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

    fn get_price(&mut self, param: &Param, file: &str, corp: bool, rozn: bool, r3: bool, profile_id: u32) -> Result<Vec<u8>, String> {
        let log;
        let init;
        {
            let w = Mutex::lock(&self.worker).unwrap();
            log = Arc::clone(&w.log);
            init = Arc::clone(&w.init);
        }
        let log_clone = Arc::clone(&log);
        let log = RwLock::read(&log).unwrap();
        let init = RwLock::read(&init).unwrap();

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

        let cache;
        {
            let w = Mutex::lock(&self.worker).unwrap();
            if w.stop {
                return Err(log.client_err(22));
            }
            cache = Arc::clone(&w.cache);
        }
        let prods;
        let locks;
        let targets;
        let store;
        let bgs;
        let world;
        let akurs;
        {
            let c = Mutex::lock(&cache).unwrap();
            prods = Arc::clone(&c.product);
        }
        {
            let c = Mutex::lock(&cache).unwrap();
            locks = Arc::clone(&c.lock);
        }
        {
            let c = Mutex::lock(&cache).unwrap();
            targets = Arc::clone(&c.target);
        }
        {
            let c = Mutex::lock(&cache).unwrap();
            store = Arc::clone(&c.stock);
        }
        {
            let c = Mutex::lock(&cache).unwrap();
            bgs = Arc::clone(&c.bg);
        }
        {
            let c = Mutex::lock(&cache).unwrap();
            world = Arc::clone(&c.world);
        }
        {
            let c = Mutex::lock(&cache).unwrap();
            akurs = Arc::clone(&c.kurs);
        }

        let kurs;
        {
            let k = Mutex::lock(&akurs).unwrap();
            kurs = k.clone();
        }
        let codes;
        {
            let p = Mutex::lock(&prods).unwrap();
            codes = p.code.clone();
        }
        let lock;
        {
            let l = Mutex::lock(&locks).unwrap();
            lock = match l.lock.get(&param.company_id) {
                Some(l) => l.clone(),
                None => LockList::new(),
            };
        }
        let target;
        {
            let t = Mutex::lock(&targets).unwrap();
            target = match t.target.get(&target_id) {
                Some(t) => t.clone(),
                None => return Err(log.client_err(25)),
            };
        }
        let stock;
        {
            let s = Mutex::lock(&store).unwrap();
            stock = match s.stock.get(&target.stock_id) {
                Some(s) => s.clone(),
                None => return Err(log.client_err(26)),
            };
        }
        let bg;
        {
            let b = Mutex::lock(&bgs).unwrap();
            bg = match b.bg.get(&param.company_id) {
                Some(b) => b.clone(),
                None => BonusGroup::new(),
            };
        }
        let country;
        {
            let w = Mutex::lock(&world).unwrap();
            country = w.countries.clone();
        }
        let hostname = if corp { "corp.brain.com.ua" } else { "opt.brain.com.ua" };

        let mut ids: Vec<String> = Vec::with_capacity(PRODUCT_CAPACITY);
        for (_, product_id) in codes {
            let p;
            {
                let pr = Mutex::lock(&prods).unwrap();
                p = match pr.product.get(&product_id) {
                    Some(p) => p.clone(),
                    None => continue,
                };
            }
            if let Some(p) = PriceItem::new(product_id, param, p, &lock, &stock, &bg, &target, hostname, &country) {
                self.items.insert(product_id, p);
                ids.push(product_id.to_string());
            }
        }
        let retail = if rozn { "p.RetailPrice" } else { "0" };
        let int = if r3 { "p.Price3" } else { "0" };
        let sql = format!("
            SELECT
                p.productID,
                CAST(
                    CASE i.PriceUSD
                        WHEN 'price' THEN p.Price
                        WHEN 'price2' THEN p.Price2
                        WHEN 'price3' THEN p.Price3
                        WHEN 'price4' THEN p.Price4
                        WHEN 'price5' THEN p.Price5
                        WHEN 'price6' THEN p.Price6
                        WHEN 'price7' THEN p.Price7
                        ELSE p.Price
                    END * CASE WHEN IFNULL(vd.value,0) <> 0 THEN (100-vd.value)/100 ELSE 1 END AS DECIMAL(10,2)
                ) AS PriceUSD, p.iprice AS Price_ind, p.PriceR AS RecommendedPrice,
                {} AS RetailPrice, {} AS InternetPrice
            FROM
                SC_products p
                LEFT JOIN (
                    SELECT ProductGroupId, GROUP_CONCAT(IF(ProfilesID = {}, fieldprice, null)) AS PriceUSD
                    FROM Profiles_Price
                    GROUP BY ProductGroupId
                ) i ON p.ProductGroupID = i.ProductGroupId
                LEFT JOIN companies cd ON cd.CompanyID={}
                LEFT JOIN discount_value vd ON vd.DiscountID=cd.DiscountID AND p.ProductGroupID=vd.ProductGroupID
            WHERE p.productID IN ({})
        ", retail, int, profile_id, param.company_id, ids.join(","));
        let mut db = match DB::new(&init.db_b2b, Arc::clone(&log_clone)) {
            Some(db) => db,
            None => return Err(log.client_err(27)),
        };

        match db.query(&sql) {
            Some(result) => {
                let row: Vec<(u32, f32, u32, f32, f32, f32)> = result;
                for (product_id, price_usd, price_ind,recommended_price, retail_price, internet_price) in row {
                    match self.items.get_mut(&product_id) {
                        Some(p) => {
                            if !p.lock {
                                p.price_usd = price_usd;
                                p.price_ind = price_ind;
                                p.retail_price = retail_price;
                                p.internet_price = internet_price;
                                p.price_UAH = price_usd * kurs;
                            }
                            p.recommended_price = recommended_price;
                        },
                        None => continue,
                    }
                }
            },
            None => return Err(log.client_err(28)),
        }
        Ok("test".as_bytes().to_vec())
    }

}
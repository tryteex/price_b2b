use std::{sync::{Mutex, Arc, RwLock}, thread::{JoinHandle, self}, time::Duration};

use chrono::Local;

use crate::{go::Go, init::Init, db::DB, log::Log, data::{Auth, World, Targets, Locks, Products, Bg, Store}};

pub const MS1000: std::time::Duration = Duration::from_millis(1000);
pub const M30: i64 = 5;

#[derive(Debug)]
pub struct Cache {
    thread: Option<JoinHandle<()>>,
    go: Arc<Mutex<Go>>,
    init: Arc<RwLock<Init>>,
    log: Arc<RwLock<Log>>,
    pub load: bool,

    pub auth: Arc<Mutex<Auth>>,
    kurs: Arc<Mutex<f32>>,
    country: Arc<Mutex<World>>,
    target: Arc<Mutex<Targets>>,
    lock: Arc<Mutex<Locks>>,
    product: Arc<Mutex<Products>>,
    stock: Arc<Mutex<Store>>,
    bg: Arc<Mutex<Bg>>,
}

impl Cache {
    pub fn new(go: Arc<Mutex<Go>>, init: Arc<RwLock<Init>>, log: Arc<RwLock<Log>>) -> Arc<Mutex<Cache>> {
        let cache = Arc::new(Mutex::new(Cache {
            thread: None,
            go,
            init,
            log,
            load: false,
            
            auth: Arc::new(Mutex::new(Auth::new())),
            kurs: Arc::new(Mutex::new(0.0)),
            country: Arc::new(Mutex::new(World::new())),
            target: Arc::new(Mutex::new(Targets::new())),
            lock: Arc::new(Mutex::new(Locks::new())),
            product: Arc::new(Mutex::new(Products::new())),
            stock: Arc::new(Mutex::new(Store::new())),
            bg: Arc::new(Mutex::new(Bg::new())),
        }));

        Cache::start(Arc::clone(&cache));
        cache
    }

    fn start(cache: Arc<Mutex<Cache>>) {
        let cache_thread = Arc::clone(&cache);
        let mut load = true;
        let mut first = true;
        let mut last = Local::now();
        let m30 = chrono::Duration::minutes(M30);
        let thread = thread::spawn(move || loop {
            if Cache::stop(Arc::clone(&cache_thread)) { break; }

            if load {
                let init;
                let log;
                {
                    let c = Mutex::lock(&cache_thread).unwrap();
                    init = Arc::clone(&c.init);
                    log = Arc::clone(&c.log);
                }
                let init = RwLock::read(&init).unwrap();
                let mut db_b2b = match DB::new(&init.db_b2b, Arc::clone(&log)) {
                    Some(db) => db,
                    None => continue,
                };
                let mut db_log = match DB::new(&init.db_log, Arc::clone(&log)) {
                    Some(db) => db,
                    None => continue,
                };

                println!("start {}", Local::now().format("%Y.%m.%d %H:%M:%S%.9f %:z").to_string()); 
                loop {
                    let res = Cache::load_auth(Arc::clone(&cache_thread), &mut db_b2b);
                    if res {
                        break;
                    }
                    thread::sleep(MS1000);
                }
                loop {
                    let res = Cache::load_currency(Arc::clone(&cache_thread), &mut db_b2b);
                    if res {
                        break;
                    }
                    thread::sleep(MS1000);
                }
                loop {
                    let res = Cache::load_country(Arc::clone(&cache_thread), &mut db_log);
                    if res {
                        break;
                    }
                    thread::sleep(MS1000);
                }
                loop {
                    let res = Cache::load_target(Arc::clone(&cache_thread), &mut db_log);
                    if res {
                        break;
                    }
                    thread::sleep(MS1000);
                }
                loop {
                    let res = Cache::load_lock(Arc::clone(&cache_thread), &mut db_b2b);
                    if res {
                        break;
                    }
                    thread::sleep(MS1000);
                }
                loop {
                    let res = Cache::load_product(Arc::clone(&cache_thread), &mut db_b2b);
                    if res {
                        break;
                    }
                    thread::sleep(MS1000);
                }
                loop {
                    let res = Cache::load_stock(Arc::clone(&cache_thread), &mut db_log);
                    if res {
                        break;
                    }
                    thread::sleep(MS1000);
                }
                loop {
                    let res = Cache::load_bg(Arc::clone(&cache_thread), &mut db_b2b);
                    if res {
                        break;
                    }
                    thread::sleep(MS1000);
                }
                println!("end {}", Local::now().format("%Y.%m.%d %H:%M:%S%.9f %:z").to_string());

                last = Local::now();
            } else {
                thread::sleep(MS1000);
            }

            if first {
                let mut c = Mutex::lock(&cache_thread).unwrap();
                c.load = true;
                first = false;
            }
            if Local::now() - last >= m30 {
                load = true;
            } else {
                load = false;
            }
        });
        {
            let mut c = Mutex::lock(&cache).unwrap();
            c.thread.replace(thread);
        }
    }

    fn stop(cache: Arc<Mutex<Cache>>) -> bool {
        let go;
        {
            let c = Mutex::lock(&cache).unwrap();
            go = Arc::clone(&c.go);
        }
        let g = Mutex::lock(&go).unwrap();
        g.stop
    }

    pub fn join(cache: Arc<Mutex<Cache>>) {
        let thread;
        {
            let mut c = Mutex::lock(&cache).unwrap();
            thread = c.thread.take();
        }
        if let Some(thread) = thread {
            thread.join().unwrap();
        }
    }

    fn load_auth(cache: Arc<Mutex<Cache>>, db: &mut DB) -> bool {
        let auth;
        {
            let c = Mutex::lock(&cache).unwrap();
            auth = Arc::clone(&c.auth);
        }
        let sql = "
            SELECT 
                u.companyID, u.userID, c.profilesID, IFNULL(c.corp, 0) corp, 
                CASE WHEN CONCAT(';', c.ApiPermissions, ';') LIKE '%;38;%' THEN 1 ELSE 0 END rozn,
                CASE WHEN CONCAT(';', c.ApiPermissions, ';') LIKE '%;39;%' THEN 1 ELSE 0 END r3
            FROM users u INNER JOIN companies c ON c.companyID=u.companyID
            WHERE 
                c.profilesID <> 0 AND c.status='registered' 
                AND CONCAT(';', c.ApiPermissions, ';') LIKE '%;37;%'
                AND u.roleID > 0
            ORDER BY u.companyID, u.userID
        ";
        match db.query(sql) {
            Some(result) => {
                let row: Vec<(u32, u32, u32, bool, bool, bool)> = result;
                {
                    let mut c = Mutex::lock(&auth).unwrap();
                    c.add_version();
                }
                for (company_id, user_id, profiles_id, corp, rozn, r3) in row {
                    let mut c = Mutex::lock(&auth).unwrap();
                    c.update(company_id, user_id, profiles_id, corp, rozn, r3);
                }
                {
                    let mut c = Mutex::lock(&auth).unwrap();
                    c.clear_old();
                }
                true
            },
            None => false,
        }
    }
    
    fn load_currency(cache: Arc<Mutex<Cache>>, db: &mut DB) -> bool{
        let kurs;
        {
            let c = Mutex::lock(&cache).unwrap();
            kurs = Arc::clone(&c.kurs);
        }
        let sql = "
            SELECT currency_value FROM SC_currency_types WHERE CID = 1
        ";
        match db.query(sql) {
            Some(result) => {
                let mut row: Vec<(f32,)> = result;
                if let Some(data) = row.pop() {
                    let mut k = Mutex::lock(&kurs).unwrap();
                    *k = data.0;
                };
                true
            },
            None => false,
        }
    }
        
    fn load_country(cache: Arc<Mutex<Cache>>, db: &mut DB) -> bool {
        let world;
        {
            let c = Mutex::lock(&cache).unwrap();
            world = Arc::clone(&c.country);
        }
        let sql = "
            SELECT countryID, name_ua, name_ru FROM delivery_country
        ";
        match db.query(sql) {
            Some(result) => {
                let row: Vec<(u32, String, String)> = result;
                for (country_id, ua, ru) in row {
                    let mut w = Mutex::lock(&world).unwrap();
                    w.update(country_id, ua, ru);
                }
                true
            },
            None => false,
        }
    }

    fn load_target(cache: Arc<Mutex<Cache>>, db: &mut DB) -> bool {
        let target;
        {
            let c = Mutex::lock(&cache).unwrap();
            target = Arc::clone(&c.target);
        }
        let sql = "
            SELECT
                t.targetid, t.regionstock, t.stockID, p.PostageCompactProduct, p.PostageBulkyGoodMid, p.PostageBulkyGood, p.PostageBulkyGoodVeryDimensional
            FROM
                delivery_targets t
            LEFT JOIN delivery_targets_postages p ON p.targetID=t.targetID
            WHERE
                p.client='brain_b2b'
        ";
        match db.query(sql) {
            Some(result) => {
                let row: Vec<(u32, bool, u32, f32, f32, f32, f32)> = result;
                {
                    let mut t = Mutex::lock(&target).unwrap();
                    t.add_version();
                }
                for (target_id, region_stock, stock_id, postage_compact, postage_middle, postage_big, postage_large) in row {
                    let mut t = Mutex::lock(&target).unwrap();
                    t.update(target_id, region_stock, stock_id, postage_compact, postage_middle, postage_big, postage_large);
                }
                {
                    let mut t = Mutex::lock(&target).unwrap();
                    t.clear_old();
                }
                true
            },
            None => false,
        }
    }

    fn load_lock(cache: Arc<Mutex<Cache>>, db: &mut DB) -> bool {
        let lock;
        {
            let c = Mutex::lock(&cache).unwrap();
            lock = Arc::clone(&c.lock);
        }
        let sql = "
            SELECT companyID, vendorID, ProductGroupID, classID, 0 FROM lockable_products 
            UNION ALL
            SELECT companyID, 0, 0, 0, productID FROM lockable_products_detailed 
        ";
        {
            let mut l = Mutex::lock(&lock).unwrap();
            l.add_version();
        }
        match db.query(sql) {
            Some(result) => {
                let row: Vec<(u32, u32, u32, u32, u32)> = result;
                for (company_id, vendor_id, group_id, class_id, product_id) in row {
                    let mut l = Mutex::lock(&lock).unwrap();
                    l.update(company_id, vendor_id, group_id, class_id, product_id);
                }
                {
                    let mut l = Mutex::lock(&lock).unwrap();
                    l.clear_old();
                }
                true
            },
            None => false,
        }
    }

    fn load_product(cache: Arc<Mutex<Cache>>, db: &mut DB) -> bool {
        let product;
        {
            let c = Mutex::lock(&cache).unwrap();
            product = Arc::clone(&c.product);
        }
        {
            let mut p = Mutex::lock(&product).unwrap();
            p.add_version();
        }
        let sql = "
            SELECT
                p.productID AS ProductID, p.bonus_opt as BonusOpt, IFNULL(p.vendorID, 0) AS vendorID, IFNULL(p.ProductGroupID, 0) AS pgid, 
                IFNULL(p.classID, 0), p.weight, p.volume, p.overall, IFNULL(p.categoryid, 0) AS CategoryID,
                p.warranty AS Warranty, p.DDP AS DDP, IFNULL(p.countryID, 0) as Country
            FROM 
                SC_products p
            WHERE p.enabled=1 AND (p.statusnew=0 OR p.statusnew=4 OR p.statusnew IS NULL) AND p.isarchive=0 AND p.isdiler=1
        ";
        match db.query(sql) {
            Some(result) => {
                let row: Vec<(u32, f32, u32, u32, u32, f32, f32, u32, u32, String, bool, u32)> = result;
                for (product_id, bonus, vendor_id, group_id, class_id, weight, volume, overall, category_id, warranty, ddp, country_id) in row {
                    let mut p = Mutex::lock(&product).unwrap();
                    p.update(product_id, bonus, vendor_id, group_id, class_id, weight, volume, overall, category_id, warranty, ddp, country_id);
                }
            },
            None => return false,
        }

        let sql = "
            SELECT
                p.productID AS ProductID, IFNULL(g.NameGroup, c.name_ua) AS GroupUa, IFNULL(g.NameGroupRus, c.name_ru) AS GroupRu, 
                IFNULL(p.brief_description_ua, '') AS DescriptionUa, IFNULL(p.brief_description_ru, '') AS DescriptionRu, 
                IFNULL(c.name_ua, '') AS CategoryNameUa, IFNULL(c.name_ru, '') AS CategoryNameRu,
                IFNULL(p.slug_ua, '') AS URLUa, IFNULL(p.slug, '') AS URLRu, IFNULL(l.name_ua, '') as ClassNameUa, IFNULL(l.name, '') as ClassNameRu
            FROM 
                SC_products p
                LEFT JOIN SC_categories c ON p.categoryid=c.categoryid
                LEFT JOIN SC_classes l ON p.classID=l.classID
                LEFT JOIN ProductGroup g ON p.ProductGroupID=g.ProductGroupID
            WHERE p.enabled=1 AND (p.statusnew=0 OR p.statusnew=4 OR p.statusnew IS NULL) AND p.isarchive=0 AND p.isdiler=1
        ";
        match db.query(sql) {
            Some(result) => {
                let row: Vec<(u32, String, String, String, String, String, String, String, String, String, String)> = result;
                for (product_id, group_ua, group_ru, desc_ua, desc_ru, category_ua, category_ru, url_ua, url_ru, class_ua, class_ru) in row {
                    let mut p = Mutex::lock(&product).unwrap();
                    p.update_lang(product_id, group_ua, group_ru, desc_ua, desc_ru, category_ua, category_ru, url_ua, url_ru, class_ua, class_ru);
                }
            },
            None => return false,
        }
        let sql = "
            SELECT
                p.productID AS ProductID, p.product_code AS Code, IFNULL(p.bg_code, '') as BG, IFNULL(p.EAN, '') AS EAN, IFNULL(p.sellerCode, '') AS sc, 
                IFNULL(p.articul, '') AS Article, IFNULL(v.name, '') AS Vendor, IFNULL(p.model, '') AS Model, 
                IFNULL(p.name_ua, '') AS NameUa, IFNULL(p.name_ru, '') AS NameRu, IFNULL(p.koduktved, '') AS UKTVED
            FROM 
                SC_products p
                LEFT JOIN SC_vendors v ON v.vendorID = p.vendorID
            WHERE p.enabled=1 AND (p.statusnew=0 OR p.statusnew=4 OR p.statusnew IS NULL) AND p.isarchive=0 AND p.isdiler=1
        ";
        match db.query(sql) {
            Some(result) => {
                let row: Vec<(u32, String, String, String, String, String, String, String, String, String, String)> = result;
                for (product_id, code, bg, ean, seller, article, vendor, model, ua, ru, uktved) in row {
                    let mut p = Mutex::lock(&product).unwrap();
                    p.update_str(product_id, code, bg, ean, seller, article, vendor, model, ua, ru, uktved);
                }
            },
            None => return false,
        }
        {
            let mut p = Mutex::lock(&product).unwrap();
            p.clear_old();
        }
        true
    }

    fn load_stock(cache: Arc<Mutex<Cache>>, db: &mut DB) -> bool {
        let stock;
        let product;
        {
            let c = Mutex::lock(&cache).unwrap();
            stock = Arc::clone(&c.stock);
            product = Arc::clone(&c.product);
        }
        {
            let mut s = Mutex::lock(&stock).unwrap();
            s.add_version();
        }
        let sql = "
            SELECT stockid, product_code, available
            FROM delivery_product_time 
            WHERE available>0 AND receipt_time <> '0000-00-00 00:00:00' AND (receipt_time-CURRENT_TIMESTAMP) < 0
        ";
        match db.query(sql) {
            Some(result) => {
                let row: Vec<(u32, String, u32)> = result;
                for (stock_id, code, available) in row {
                    let product_id;
                    {
                        let p = Mutex::lock(&product).unwrap();
                        product_id = match p.get_product_id(code) {
                            Some(product_id) => product_id,
                            None => continue,
                        }
                    }
                    {
                        let mut s = Mutex::lock(&stock).unwrap();
                        s.update_available(stock_id, product_id, available);
                    }
                }
            },
            None => return false,
        }
        let sql = "
            SELECT
                stockid, product_code,
            DATEDIFF(
                IF(
                receipt_time < CURRENT_TIMESTAMP,
                ADDTIME(CURRENT_TIMESTAMP,'00:30:00'),
                IF(
                    ADDTIME(receipt_time,'00:30:00') < ADDTIME(CURRENT_TIMESTAMP,'00:30:00'),
                    ADDTIME(receipt_time,'00:30:00'),
                    receipt_time
                )
                ),
                CURRENT_TIMESTAMP
            ) + 1 AS DayDelivery
            FROM
                delivery_product_time WHERE receipt_time <> '0000-00-00 00:00:00'
        ";
        match db.query(sql) {
            Some(result) => {
                let row: Vec<(u32, String, u32)> = result;
                for (stock_id, code, day) in row {
                    let product_id;
                    {
                        let p = Mutex::lock(&product).unwrap();
                        product_id = match p.get_product_id(code) {
                            Some(product_id) => product_id,
                            None => continue,
                        }
                    }
                    {
                        let mut s = Mutex::lock(&stock).unwrap();
                        s.update_day(stock_id, product_id, day);
                    }
                }
            },
            None => return false,
        }
        {
            let mut s = Mutex::lock(&stock).unwrap();
            s.clear_old();
        }
        true
    }

    fn load_bg(cache: Arc<Mutex<Cache>>, db: &mut DB) -> bool {
        let bg;
        {
            let c = Mutex::lock(&cache).unwrap();
            bg = Arc::clone(&c.bg);
        }
        let sql = "
            SELECT companyID, bg_code FROM companies_bonuses
        ";
        match db.query(sql) {
            Some(result) => {
                let row: Vec<(u32, String)> = result;
                {
                    let mut b = Mutex::lock(&bg).unwrap();
                    b.add_version();
                }
                for (company_id, group) in row {
                    let mut b = Mutex::lock(&bg).unwrap();
                    b.update(company_id, group);
                }
                {
                    let mut b = Mutex::lock(&bg).unwrap();
                    b.clear_old();
                }
                true
            },
            None => false,
        }
    }

}


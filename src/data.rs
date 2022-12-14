use std::{collections::{HashMap, hash_map::Entry}};

#[derive(Debug)]
pub struct Store {
    version: u32,
    capacity: usize, 
    pub stock: HashMap<u32, ProductStock>,
}

impl Store {
    pub fn new(cap: usize, capacity: usize) -> Store {
        Store {
            version: 0,
            stock: HashMap::with_capacity(cap),
            capacity,
        }
    }

    pub fn add_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }

    pub fn update_available(&mut self, stock_id: u32, product_id: u32, available: String) {
        let stock = match self.stock.entry(stock_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let p = ProductStock::new(self.capacity);
                v.insert(p)
            },
        };
        stock.update_available(self.version, product_id, available);
    }
    
    pub fn update_day(&mut self, stock_id: u32, product_id: u32, day: String) {
        let stock = match self.stock.entry(stock_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let p = ProductStock::new(self.capacity);
                v.insert(p)
            },
        };
        stock.update_day(self.version, product_id, day);
    }

    pub fn clear_old(&mut self) {
        self.stock.retain(|_, p| {
            if p.version != self.version {
                return false;
            }
            p.clear_old();
            true
        });
    }
}

#[derive(Debug, Clone)]
pub struct ProductStock {
    version: u32,
    pub product: HashMap<u32, Stock>,
}

impl ProductStock {
    pub fn new(cap: usize) -> ProductStock {
        ProductStock {
            version: 0,
            product: HashMap::with_capacity(cap),
        }
    }

    pub fn update_available(&mut self, version: u32, product_id: u32, available: String) {
        self.version = version;
        match self.product.entry(product_id) {
            Entry::Occupied(o) => {
                let s = o.into_mut();
                s.version = version;
                s.available = available;
            },
            Entry::Vacant(v) => {
                let s = Stock::new(version, available, "0".to_owned());
                v.insert(s);
            },
        };
    }
    
    pub fn update_day(&mut self, version: u32, product_id: u32, day: String) {
        self.version = version;
        match self.product.entry(product_id) {
            Entry::Occupied(o) => {
                let s = o.into_mut();
                s.version = version;
                s.day = day;
            },
            Entry::Vacant(v) => {
                let s = Stock::new(version, "0".to_owned(), day);
                v.insert(s);
            },
        };
    }

    pub fn clear_old(&mut self) {
        self.product.retain(|_, s| {
            if &s.day == "0" && &s.available == "0" {
                return false;
            }
            s.version == self.version
        });
    }
}

#[derive(Debug, Clone)]
pub struct Stock {
    version: u32,
    pub available: String,
    pub day: String,
}

impl Stock {
    pub fn new(version: u32, available: String, day: String) -> Stock {
        Stock {
            version,
            available,
            day,
        }
    }
}

#[derive(Debug)]
pub struct Bg {
    pub bg: HashMap<u32, BonusGroup>,
    capacity: usize,
    version: u32,
}

impl Bg {
    pub fn new(cap: usize, capacity: usize) -> Bg {
        Bg {
            bg: HashMap::with_capacity(cap),
            capacity,
            version: 0,
        }
    }

    pub fn add_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }

    pub fn update(&mut self, company_id: u32, group: String) {
        let g = match self.bg.entry(company_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let g = BonusGroup::new(self.capacity);
                v.insert(g)
            },
        };
        g.update(self.version, group);
    }

    pub fn clear_old(&mut self) {
        self.bg.retain(|_, g| {
            if self.version != g.version {
                return false;
            }
            g.groups.retain(|_, v| {
                *v == self.version
            });
            true
        })
    }
}

#[derive(Debug, Clone)]
pub struct BonusGroup {
    version: u32,
    pub groups: HashMap<String, u32>,
}

impl BonusGroup {
    pub fn new(cap: usize) -> BonusGroup {
        BonusGroup {
            version: 0,
            groups: HashMap::with_capacity(cap),
        }
    }

    pub fn update(&mut self, version: u32, group: String) {
        self.version = version;
        match self.groups.entry(group) {
            Entry::Occupied(o) => {
                let g = o.into_mut();
                *g = version;
            },
            Entry::Vacant(v) => { v.insert(version); },
        };
    }
}

#[derive(Debug)]
pub struct Products {
    version: u32, 
    pub product: HashMap<u32, Product>,
    pub code: HashMap<String, u32>,
}
    
impl Products {
    pub fn new(cap: usize) -> Products {
        Products{
            version: 0, 
            product: HashMap::with_capacity(cap),
            code: HashMap::with_capacity(cap),
        }
    }

    pub fn add_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }

    pub fn get_product_id(&self, code: String) -> Option<u32> {
        match self.code.get(&code) {
            Some(product_id) => Some(*product_id),
            None => None,
        }
    }

    pub fn update(&mut self, product_id: u32, bonus: f32, vendor_id: u32, group_id: u32, class_id: u32, weight: f32, volume: f32, overall: i32, category_id: u32, warranty: u32, ddp: bool, country_id: u32) {
        match self.product.entry(product_id) {
            Entry::Occupied(o) => {
                let p = o.into_mut();
                p.version_1 = self.version;
                p.bonus = bonus;
                p.vendor_id = vendor_id;
                p.group_id = group_id;
                p.class_id = class_id;
                p.weight = weight;
                p.volume = volume;
                p.overall = overall;
                p.category_id = category_id;
                p.warranty = warranty;
                p.ddp = ddp;
                p.country_id = country_id;
            },
            Entry::Vacant(v) => {
                let p = Product::new( self.version, 0, 0,
                    "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), 
                    "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), 
                    "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), 0, 
                    bonus, vendor_id, group_id, class_id, weight, volume, overall, category_id, ddp, country_id, "0".to_owned()
                );
                v.insert(p);
            },
        };
    }

    pub fn update_lang(&mut self, product_id: u32, group_ua: String, group_ru: String, desc_ua: String, desc_ru: String, category_ua: String, category_ru: String, url_ua: String, url_ru: String, class_ua: String, class_ru: String) {
        match self.product.entry(product_id) {
            Entry::Occupied(o) => {
                let p = o.into_mut();
                p.version_2 = self.version;
                p.group_ua = group_ua;
                p.group_ru = group_ru;
                p.desc_ua = desc_ua;
                p.desc_ru = desc_ru;
                p.category_ua = category_ua;
                p.category_ru = category_ru;
                p.url_ua = url_ua;
                p.url_ru = url_ru;
                p.class_ua = class_ua;
                p.class_ru = class_ru;

            },
            Entry::Vacant(v) => {
                let p = Product::new(0, self.version, 0, 
                    "".to_owned(), "".to_owned(), "".to_owned(), group_ua, group_ru, desc_ua, desc_ru, category_ua, category_ru, url_ua, url_ru, class_ua, class_ru, 
                    "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), 0, 
                    0.0, 0, 0, 0, 0.0, 0.0, 0, 0, false, 0, "0".to_owned()
                );
                v.insert(p);
            },
        };
    }

    pub fn update_str(&mut self, product_id: u32, code: String, bg: String, ean: String, seller: String, article: String, vendor: String, model: String, ua: String, ru: String, uktved: String, exclusive: String) {
        if let Entry::Vacant(v) = self.code.entry(code.clone()) {
            v.insert(product_id);
        }
        match self.product.entry(product_id) {
            Entry::Occupied(o) => {
                let p = o.into_mut();
                p.version_3 = self.version;
                p.code = code;
                p.bg = bg;
                p.ean = ean;
                p.seller = seller;
                p.article = article;
                p.vendor = vendor;
                p.model = model;
                p.ua = ua;
                p.ru = ru;
                p.uktved = uktved;
                p.exclusive = exclusive;
            },
            Entry::Vacant(v) => {
                let p = Product::new(0, 0, self.version, 
                    code, ua, ru, "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), 
                    "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), 
                    bg, ean, seller, article, vendor, model, uktved, 0, 
                    0.0, 0, 0, 0, 0.0, 0.0, 0, 0, false, 0,
                    exclusive
                );
                v.insert(p);
            },
        };
    }

    pub fn clear_old(&mut self) {
        self.product.retain(|_, p| {
            if self.version != p.version_1 || self.version != p.version_2 || self.version != p.version_3 {
                return false;
            }
            true
        })
    }
}

#[derive(Debug, Clone)]
pub struct Product {
    version_1: u32, 
    version_2: u32, 
    version_3: u32, 

    pub code: String, 
    pub ua: String, 
    pub ru: String, 
    pub group_ua: String, 
    pub group_ru: String, 
    pub desc_ua: String, 
    pub desc_ru: String, 
    pub category_ua: String, 
    pub category_ru: String, 
    pub url_ua: String, 
    pub url_ru: String, 
    pub class_ua: String, 
    pub class_ru: String, 
    pub bg: String, 
    pub ean: String, 
    pub seller: String, 
    pub article: String, 
    pub vendor: String, 
    pub model: String, 
    pub uktved: String, 
    pub warranty: u32, 
    pub bonus: f32,
    pub vendor_id: u32,
    pub group_id: u32,
    pub class_id: u32,
    pub weight: f32,
    pub volume: f32,
    pub overall: i32,
    pub category_id: u32,
    pub ddp: bool,
    pub country_id: u32,
    pub exclusive: String,
}
    
impl Product {
    pub fn new(version_1: u32, version_2: u32, version_3: u32, code: String, ua: String, ru: String, group_ua: String, group_ru: String, desc_ua: String, desc_ru: String, category_ua: String, category_ru: String, url_ua: String, url_ru: String, class_ua: String, class_ru: String, bg: String, ean: String, seller: String, article: String, vendor: String, model: String, uktved: String, warranty: u32, bonus: f32, vendor_id: u32, group_id: u32, class_id: u32, weight: f32, volume: f32, overall: i32, category_id: u32, ddp: bool, country_id: u32, exclusive: String,) -> Product {
        Product {
            version_1,
            version_2,
            version_3,
            code, 
            ua, 
            ru, 
            group_ua, 
            group_ru, 
            desc_ua, 
            desc_ru, 
            category_ua, 
            category_ru, 
            url_ua, 
            url_ru, 
            class_ua, 
            class_ru, 
            bg, 
            ean, 
            seller, 
            article, 
            vendor, 
            model, 
            uktved, 
            warranty, 
            bonus,
            vendor_id,
            group_id,
            class_id,
            weight,
            volume,
            overall,
            category_id,
            ddp,
            country_id,
            exclusive,
        }
    }
}

#[derive(Debug)]
pub struct Locks {
    pub lock: HashMap<u32, LockList>,
    capacity: usize,
    version: u32,
}

impl Locks {
    pub fn new(cap: usize, capacity: usize) -> Locks {
        Locks{
            lock: HashMap::with_capacity(cap),
            capacity,
            version: 0,
        }
    }

    pub fn add_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }

    pub fn update(&mut self, company_id: u32, vendor_id: u32, group_id: u32, class_id: u32, product_id: u32) {
        let l = match self.lock.entry(company_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(LockList::new(self.capacity)),
        };
        l.update(self.version, vendor_id, group_id, class_id, product_id);
    }

    pub fn clear_old(&mut self) {
        self.lock.retain(|_, l| {
            if l.version != self.version {
                return false;
            }
            l.clear_old();
            true
        });
    }
}

#[derive(Debug, Clone)]
pub struct LockList {
    pub list: HashMap<String, Lock>,
    version: u32,
}

impl LockList {
    pub fn new(cap: usize) -> LockList{
        LockList {
            list: HashMap::with_capacity(cap),
            version: 0,
        }
    }

    pub fn update(&mut self, version: u32, vendor_id: u32, group_id: u32, class_id: u32, product_id: u32) {
        self.version = version;
        let key = format!("{}:{}:{}:{}", vendor_id, group_id, class_id, product_id);
        match self.list.entry(key) {
            Entry::Occupied(o) => {
                let l = o.into_mut();
                l.version = self.version;
                l.vendor_id = vendor_id;
                l.group_id = group_id;
                l.class_id = class_id;
                l.product_id = product_id;
            },
            Entry::Vacant(v) => {
                let l = Lock::new(self.version, vendor_id, group_id, class_id, product_id);
                v.insert(l);
            },
        };
    }

    pub fn clear_old(&mut self) {
        self.list.retain(|_, l| {
            l.version == self.version
        });
    }
}

#[derive(Debug, Clone)]
pub struct Lock {
    version: u32, 
    vendor_id: u32, 
    group_id: u32, 
    class_id: u32, 
    product_id: u32, 
}

impl Lock {
    pub fn new(version: u32, vendor_id: u32, group_id: u32, class_id: u32, product_id: u32) -> Lock {
        Lock {
            version,
            vendor_id,
            group_id,
            class_id,
            product_id,
        }
    }
}

#[derive(Debug)]
pub struct Targets {
    pub target: HashMap<u32, Target>,
    version: u32,
}

impl Targets {
    pub fn new(cap: usize) -> Targets {
        Targets{
            target: HashMap::with_capacity(cap),
            version: 0,
        }
    }

    pub fn add_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }

    pub fn update(&mut self, target_id: u32, region_stock: bool, stock_id: u32, postage_compact: f32, postage_middle: f32, postage_big: f32, postage_large: f32) {
        match self.target.entry(target_id) {
            Entry::Occupied(o) => {
                let t = o.into_mut();
                t.version = self.version;
                t.region_stock = region_stock;
                t.stock_id = stock_id;
                t.postage_compact = postage_compact;
                t.postage_middle = postage_middle;
                t.postage_big = postage_big;
                t.postage_large = postage_large;
            },
            Entry::Vacant(v) => {
                let t = Target::new(self.version, region_stock, stock_id, postage_compact, postage_middle, postage_big, postage_large);
                v.insert(t);
            },
        };
    }

    pub fn clear_old(&mut self) {
        self.target.retain(|_, u| {
            self.version == u.version
        });
    }
}

#[derive(Debug, Clone)]
pub struct Target {
    version: u32, 
    pub region_stock: bool, 
    pub stock_id: u32, 
    pub postage_compact: f32, 
    pub postage_middle: f32, 
    pub postage_big: f32, 
    pub postage_large: f32,
}

impl Target {
    pub fn new(version: u32, region_stock: bool, stock_id: u32, postage_compact: f32, postage_middle: f32, postage_big: f32, postage_large: f32) -> Target {
        Target {
            version,
            region_stock,
            stock_id,
            postage_compact,
            postage_middle,
            postage_big,
            postage_large,
        }
    }
}

#[derive(Debug)]
pub struct World {
    pub countries: HashMap<u32, Country>,
}

impl World {
    pub fn new(cap: usize) -> World {
        World {
            countries: HashMap::with_capacity(cap),
        }
    }

    pub fn update(&mut self, country_id: u32, ua: String, ru: String) {
        match self.countries.entry(country_id) {
            Entry::Occupied(o) => {
                let c = o.into_mut();
                c.ua = ua;
                c.ru = ru;
            },
            Entry::Vacant(v) => {
                let c = Country::new(ua, ru);
                v.insert(c);
            },
        };
    }
}

#[derive(Debug, Clone)]
pub struct Country {
    pub ua: String,
    pub ru: String,
}

impl Country {
    pub fn new(ua: String, ru: String) -> Country {
        Country {
            ua,
            ru,
        }
    }
}

#[derive(Debug)]
pub struct Auth {
    pub company: HashMap<u32, Company>,
    capacity: usize,
    version: u32,
}

impl Auth {
    pub fn new(cap: usize, capacity: usize) -> Auth {
        Auth {
            company: HashMap::with_capacity(cap),
            capacity,
            version: 0,
        }
    }

    pub fn update(&mut self, company_id: u32, user_id: u32, profiles_id: u32, corp: bool, rozn: bool, r3: bool) {
        let c = match self.company.entry(company_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let c = Company::new(self.capacity);
                v.insert(c)
            },
        };
        c.update(self.version, user_id, profiles_id, corp, rozn, r3);
    }

    pub fn add_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }

    pub fn clear_old(&mut self) {
        self.company.retain(|_, c| {
            if self.version != c.version {
                return false;
            }
            c.clear_old();
            true
        });
    }
}

#[derive(Debug)]
pub struct Company {
    pub users: HashMap<u32, User>,
    version: u32,
}

impl Company {
    pub fn new(cap: usize) -> Company {
        Company {
            users: HashMap::with_capacity(cap),
            version: 0,
        }
    }

    pub fn update(&mut self, version: u32, user_id: u32, profiles_id: u32, corp: bool, rozn: bool, r3: bool) {
        self.version = version;
        match self.users.entry(user_id) {
            Entry::Occupied(o) => {
                let u = o.into_mut();
                u.update(version, profiles_id, corp, rozn, r3);
            },
            Entry::Vacant(v) => {
                let c = User::new(self.version, profiles_id, corp, rozn, r3);
                v.insert(c);
            },
        };
    }

    pub fn clear_old(&mut self) {
        self.users.retain(|_, u| {
            self.version == u.version
        });
    }
}

#[derive(Debug)]
pub struct User {
    version: u32,
    pub profiles_id: u32, 
    pub corp: bool, 
    pub rozn: bool, 
    pub r3: bool,
}

impl User {
    pub fn new(version: u32, profiles_id: u32, corp: bool, rozn: bool, r3: bool) -> User {
        User {
            version,
            profiles_id,
            corp,
            rozn,
            r3,
        }
    }

    pub fn update(&mut self, version: u32, profiles_id: u32, corp: bool, rozn: bool, r3: bool) {
        self.version = version;
        self.profiles_id = profiles_id;
        self.corp = corp;
        self.rozn = rozn;
        self.r3 = r3;
    }
}
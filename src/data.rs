use std::{collections::{HashMap, hash_map::Entry}};

use crate::{STOCK_CAPACITY, PRODUCT_CAPACITY, BONUS_COMPANY_CAPACITY, BONUS_GROUP_CAPACITY, LOCK_CAPACITY, TARGET_CAPACITY, COUNTRY_CAPACITY, AUTH_COMPANY_CAPACITY, AUTH_USER_CAPACITY};

#[derive(Debug)]
pub struct Store {
    version: u32,
    stock: HashMap<u32, ProductStock>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            version: 0,
            stock: HashMap::with_capacity(STOCK_CAPACITY),
        }
    }

    pub fn add_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }

    pub fn update_available(&mut self, stock_id: u32, product_id: u32, available: u32) {
        let stock = match self.stock.entry(stock_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let p = ProductStock::new();
                v.insert(p)
            },
        };
        stock.update_available(self.version, product_id, available);
    }
    
    pub fn update_day(&mut self, stock_id: u32, product_id: u32, day: u32) {
        let stock = match self.stock.entry(stock_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let p = ProductStock::new();
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

#[derive(Debug)]
pub struct ProductStock {
    version: u32,
    product: HashMap<u32, Stock>,
}

impl ProductStock {
    pub fn new() -> ProductStock {
        ProductStock {
            version: 0,
            product: HashMap::with_capacity(PRODUCT_CAPACITY),
        }
    }

    pub fn update_available(&mut self, version: u32, product_id: u32, available: u32) {
        self.version = version;
        match self.product.entry(product_id) {
            Entry::Occupied(o) => {
                let s = o.into_mut();
                s.version = version;
                s.available = available;
            },
            Entry::Vacant(v) => {
                let s = Stock::new(version, available, 0);
                v.insert(s);
            },
        };
    }
    
    pub fn update_day(&mut self, version: u32, product_id: u32, day: u32) {
        self.version = version;
        match self.product.entry(product_id) {
            Entry::Occupied(o) => {
                let s = o.into_mut();
                s.version = version;
                s.day = day;
            },
            Entry::Vacant(v) => {
                let s = Stock::new(version, 0, day);
                v.insert(s);
            },
        };
    }

    pub fn clear_old(&mut self) {
        self.product.retain(|_, s| {
            if s.day == 0 && s.available == 0 {
                return false;
            }
            s.version == self.version
        });
    }
}

#[derive(Debug)]
pub struct Stock {
    version: u32,
    available: u32,
    day: u32,
}

impl Stock {
    pub fn new(version: u32, available: u32, day: u32) -> Stock {
        Stock {
            version,
            available,
            day,
        }
    }
}

#[derive(Debug)]
pub struct Bg {
    bg: HashMap<u32, BonusGroup>,
    version: u32,
}

impl Bg {
    pub fn new() -> Bg {
        Bg {
            bg: HashMap::with_capacity(BONUS_COMPANY_CAPACITY),
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
                let g = BonusGroup::new();
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

#[derive(Debug)]
pub struct BonusGroup {
    version: u32, 
    groups: HashMap<String, u32>,
}

impl BonusGroup {
    pub fn new() -> BonusGroup {
        BonusGroup {
            version: 0,
            groups: HashMap::with_capacity(BONUS_GROUP_CAPACITY),
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
    product: HashMap<u32, Product>,
    code: HashMap<String, u32>,
}
    
impl Products {
    pub fn new() -> Products {
        Products{
            version: 0, 
            product: HashMap::with_capacity(PRODUCT_CAPACITY),
            code: HashMap::with_capacity(PRODUCT_CAPACITY),
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

    pub fn update(&mut self, product_id: u32, bonus: f32, vendor_id: u32, group_id: u32, class_id: u32, weight: f32, volume: f32, overall: u32, category_id: u32, warranty: String, ddp: bool, country_id: u32) {
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
                    "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), 
                    bonus, vendor_id, group_id, class_id, weight, volume, overall, category_id, ddp, country_id
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
                    "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), 
                    0.0, 0, 0, 0, 0.0, 0.0, 0, 0, false, 0
                );
                v.insert(p);
            },
        };
    }

    pub fn update_str(&mut self, product_id: u32, code: String, bg: String, ean: String, seller: String, article: String, vendor: String, model: String, ua: String, ru: String, uktved: String) {
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
            },
            Entry::Vacant(v) => {
                let p = Product::new(0, 0, self.version, 
                    code, ua, ru, "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), 
                    "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), 
                    bg, ean, seller, article, vendor, model, uktved, "".to_owned(), 
                    0.0, 0, 0, 0, 0.0, 0.0, 0, 0, false, 0
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

#[derive(Debug)]
pub struct Product {
    version_1: u32, 
    version_2: u32, 
    version_3: u32, 

    code: String, 
    ua: String, 
    ru: String, 
    group_ua: String, 
    group_ru: String, 
    desc_ua: String, 
    desc_ru: String, 
    category_ua: String, 
    category_ru: String, 
    url_ua: String, 
    url_ru: String, 
    class_ua: String, 
    class_ru: String, 
    bg: String, 
    ean: String, 
    seller: String, 
    article: String, 
    vendor: String, 
    model: String, 
    uktved: String, 
    warranty: String, 
    bonus: f32,
    vendor_id: u32,
    group_id: u32,
    class_id: u32,
    weight: f32,
    volume: f32,
    overall: u32,
    category_id: u32,
    ddp: bool,
    country_id: u32,
}
    
impl Product {
    pub fn new(version_1: u32, version_2: u32, version_3: u32, code: String, ua: String, ru: String, group_ua: String, group_ru: String, desc_ua: String, desc_ru: String, category_ua: String, category_ru: String, url_ua: String, url_ru: String, class_ua: String, class_ru: String, bg: String, ean: String, seller: String, article: String, vendor: String, model: String, uktved: String, warranty: String, bonus: f32, vendor_id: u32, group_id: u32, class_id: u32, weight: f32, volume: f32, overall: u32, category_id: u32, ddp: bool, country_id: u32,) -> Product {
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
        }
    }
}

#[derive(Debug)]
pub struct Locks {
    lock: HashMap<u32, Lock>,
    version: u32,
}

impl Locks {
    pub fn new() -> Locks {
        Locks{
            lock: HashMap::with_capacity(LOCK_CAPACITY),
            version: 0,
        }
    }

    pub fn add_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }

    pub fn update(&mut self, company_id: u32, vendor_id: u32, group_id: u32, class_id: u32, product_id: u32) {
        match self.lock.entry(company_id) {
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
        self.lock.retain(|_, l| {
            self.version == l.version
        });
    }
}

#[derive(Debug)]
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
    target: HashMap<u32, Target>,
    version: u32,
}

impl Targets {
    pub fn new() -> Targets {
        Targets{
            target: HashMap::with_capacity(TARGET_CAPACITY),
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

#[derive(Debug)]
pub struct Target {
    version: u32, 
    region_stock: bool, 
    stock_id: u32, 
    postage_compact: f32, 
    postage_middle: f32, 
    postage_big: f32, 
    postage_large: f32,
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
    countries: HashMap<u32, Country>,
}

impl World {
    pub fn new() -> World {
        World {
            countries: HashMap::with_capacity(COUNTRY_CAPACITY),
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

#[derive(Debug)]
pub struct Country {
    ua: String,
    ru: String,
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
    version: u32,
}

impl Auth {
    pub fn new() -> Auth {
        Auth {
            company: HashMap::with_capacity(AUTH_COMPANY_CAPACITY),
            version: 0,
        }
    }

    pub fn update(&mut self, company_id: u32, user_id: u32, profiles_id: u32, corp: bool, rozn: bool, r3: bool) {
        let c = match self.company.entry(company_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let c = Company::new();
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
    pub fn new() -> Company {
        Company {
            users: HashMap::with_capacity(AUTH_USER_CAPACITY),
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
mod log;
mod init;
mod help;
mod action;
mod go;
mod queue;
mod worker;
mod fastcgi;
mod price;
mod cache;
mod db;
mod data;
mod param;
mod format_xlsx;

use std::env;

use action::Action;
use go::Go;
use help::Help;
use init::AppInit;

use crate::{log::Log, init::Init};

pub const AUTH_COMPANY_CAPACITY: usize = 32768;
pub const AUTH_USER_CAPACITY: usize = 64;
pub const COUNTRY_CAPACITY: usize = 128;
pub const TARGET_CAPACITY: usize = 512;
pub const LOCK_CAPACITY: usize = 8192;
pub const LOCK_ITEM_CAPACITY: usize = 512;
pub const PRODUCT_CAPACITY: usize = 1048576;
pub const BONUS_COMPANY_CAPACITY: usize = 4096;
pub const BONUS_GROUP_CAPACITY: usize = 256;
pub const STOCK_CAPACITY: usize = 64;

fn main() {
    let dir = env::current_dir().unwrap().to_str().unwrap().to_owned();
    let log = Log::new(&dir);
    let init = Init::new(&log, &dir);

    match init.app {
        AppInit::Help => Help::show(),
        AppInit::Start => Action::start(init, log, &dir, false),
        AppInit::Check => Action::start(init, log, &dir, true),
        AppInit::Go => Go::run(init, log),
        AppInit::Stop => Action::stop(init, log),
    }
    
}

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
mod format_php;
mod format_xml;
mod format_json;

use std::env;

use action::Action;
use go::Go;
use help::Help;
use init::AppInit;

use crate::{log::Log, init::Init};

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
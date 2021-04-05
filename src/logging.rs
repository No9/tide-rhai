use std::fmt::Display;
use tide::log;

pub fn log<T: Display>(s: T) {
    println!("{}", s)
}

pub fn info<T: Display>(s: T) {
    log::info!("{}", s)
}

pub fn warn<T: Display>(s: T) {
    log::warn!("{}", s)
}

pub fn error<T: Display>(s: T) {
    log::error!("{}", s)
}

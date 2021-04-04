use rhai::{Dynamic, ImmutableString};
use tide::log;

pub fn log(s: ImmutableString) {
    println!("{:?}", s)
}

pub fn log_dynamic(s: Dynamic) {
    println!("{:?}", s)
}

pub fn info(s: ImmutableString) {
    log::info!("{:?}", s)
}

pub fn info_dynamic(s: Dynamic) {
    log::info!("{:?}", s)
}

pub fn warn(s: ImmutableString) {
    log::warn!("{:?}", s)
}

pub fn warn_dynamic(s: Dynamic) {
    log::warn!("{:?}", s)
}

pub fn error(s: ImmutableString) {
    log::error!("{:?}", s)
}

pub fn error_dynamic(s: Dynamic) {
    log::error!("{:?}", s)
}
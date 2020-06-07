#[macro_use] extern crate log;
use std::io::Write;
use env_logger::Builder;
use log::LevelFilter;

mod client;
use client::account_client;

fn main() {
    config_logger(true);

    let get_resp = account_client::get();
    let post_resp= account_client::post();
}

fn config_logger(debug: bool) {
    let level = match debug {
        true => LevelFilter::Debug,
        _ => LevelFilter::Info
    };

    Builder::new()
        .format(|buf, record| {
            writeln!(buf, "{}: {}", record.level(), record.args()) })
        .filter(None, level)
        .init();
}


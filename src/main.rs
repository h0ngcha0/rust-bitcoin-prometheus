#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate prometheus;
#[macro_use] extern crate lazy_static;

use std::time::Duration;

pub mod schedule;
pub mod prometheus_metrics;

fn main() {
    schedule::repeat(Duration::from_secs(120), prometheus_metrics::set_metrics);
    rocket::ignite().mount("/", routes![prometheus_metrics::endpoint]).launch();
}
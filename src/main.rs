#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate prometheus;
#[macro_use] extern crate lazy_static;

pub mod schedule;
pub mod metrics;

fn main() {
    metrics::collect();
    rocket::ignite().mount("/", routes![metrics::endpoint]).launch();
}
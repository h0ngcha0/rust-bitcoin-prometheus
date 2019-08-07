#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate reqwest;
extern crate serde;
extern crate chrono;
extern crate crossbeam;

#[macro_use] extern crate prometheus;
#[macro_use] extern crate lazy_static;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use prometheus::{IntGauge, TextEncoder, Encoder};
use log::info;
use std::time::Duration;

pub mod schedule;

lazy_static! {
    static ref UNCONFIRMED_COUNT_GAUGE: IntGauge = register_int_gauge!(
        "unconfirmed_count_gauge",
        "Number of unconfirmed transaction in memory pool"
    ).unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockCypherBitcoinChainResponse {
    name: String,
    height: u32,
    hash: String,
    time: DateTime<Utc>,
    latest_url: String,
    previous_hash: String,
    previous_url: String,
    unconfirmed_count: u32,
    high_fee_per_kb: u32,
    medium_fee_per_kb: u32,
    low_fee_per_kb: u32,
    last_fork_height: u32
}

#[get("/")]
fn index() -> String {
    // Gather the metrics.
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let result = String::from_utf8(buffer).unwrap();
    return result;
}

fn main() {
    schedule::repeat(Duration::from_secs(60), || {
        let body: BlockCypherBitcoinChainResponse = reqwest::get("https://api.blockcypher.com/v1/btc/main")
            .unwrap()
            .json()
            .unwrap();

        info!("{:?}", body);
        UNCONFIRMED_COUNT_GAUGE.set(body.unconfirmed_count as i64);
    });

    rocket::ignite().mount("/", routes![index]).launch();
}
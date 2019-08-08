use log::info;
use prometheus::{TextEncoder, Encoder, IntGauge};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

lazy_static! {
    pub static ref UNCONFIRMED_COUNT_GAUGE: IntGauge = register_int_gauge!(
        "unconfirmed_count_gauge",
        "Number of unconfirmed transaction in memory pool"
    ).unwrap();
}

#[get("/metrics")]
pub fn endpoint() -> String {
    // Gather metrics from the default registry.
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let result = String::from_utf8(buffer).unwrap();
    return result;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockCypherBitcoinChainResponse {
    pub name: String,
    pub height: u32,
    pub hash: String,
    pub time: DateTime<Utc>,
    pub latest_url: String,
    pub previous_hash: String,
    pub previous_url: String,
    pub unconfirmed_count: u32,
    pub high_fee_per_kb: u32,
    pub medium_fee_per_kb: u32,
    pub low_fee_per_kb: u32,
    pub last_fork_height: u32
}

pub fn set_metrics() -> () {
    let body: BlockCypherBitcoinChainResponse = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get("https://api.blockcypher.com/v1/btc/main")
        .send()
        .unwrap()
        .json()
        .unwrap();

    info!("{:?}", body);
    UNCONFIRMED_COUNT_GAUGE.set(body.unconfirmed_count as i64);
}
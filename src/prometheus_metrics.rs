use log::info;
use prometheus::{TextEncoder, Encoder, IntGauge};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

lazy_static! {
    pub static ref UNCONFIRMED_COUNT_GAUGE: IntGauge = register_int_gauge!(
        "unconfirmed_count_gauge",
        "Number of unconfirmed transaction in memory pool"
    ).unwrap();

    pub static ref LOW_FEE_PER_KB: IntGauge = register_int_gauge!(
        "bitcoin_low_fee_per_kb",
        concat!(
            "A rolling average of the fee (in satoshis) paid per kilobyte for ",
            "transactions to be confirmed in 7 or more blocks"
        )
    ).unwrap();

    pub static ref HEIGHT: IntGauge = register_int_gauge!(
        "bitcoin_block_height",
        "The current height of the blockchain; i.e., the number of blocks in the blockchain"
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
    let response: BlockCypherBitcoinChainResponse = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get("https://api.blockcypher.com/v1/btc/main")
        .send()
        .unwrap()
        .json()
        .unwrap();

    info!("{:?}", response);

    UNCONFIRMED_COUNT_GAUGE.set(response.unconfirmed_count as i64);
    LOW_FEE_PER_KB.set(response.low_fee_per_kb as i64);
    HEIGHT.set(response.height as i64);
}
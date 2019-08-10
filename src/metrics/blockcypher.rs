use prometheus::IntGauge;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use log::info;

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

// https://www.blockcypher.com/dev/bitcoin/#blockchain
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

// https://www.blockcypher.com/dev/bitcoin/#block
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockCypherBlockResponse {
    pub hash: String,
    pub height: u32,
    pub total: u64,    // total number of value transacted in this block (in satoshi)
    pub fee: u64,      // total number of fee in this block (in satoshi)
    pub size: u64,
    pub n_tx: u32,
    pub bits: u64
}

pub struct BitcoinChainGeneralInfo {}

impl super::PrometheusMetrics for BitcoinChainGeneralInfo {
    fn set_metrics(&self) -> () {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let chain_response: BlockCypherBitcoinChainResponse = client
            .get("https://api.blockcypher.com/v1/btc/main")
            .send()
            .unwrap()
            .json()
            .unwrap();

        info!("BlockCypher chain response {:?}", chain_response);

        UNCONFIRMED_COUNT_GAUGE.set(chain_response.unconfirmed_count as i64);
        LOW_FEE_PER_KB.set(chain_response.low_fee_per_kb as i64);
        HEIGHT.set(chain_response.height as i64);

        let niux: String = format!("https://api.blockcypher.com/v1/btc/main/blocks/{}", chain_response.hash);
        let block_response: BlockCypherBlockResponse = client
            .get(niux)
            .send()
            .unwrap()
            .json()
            .unwrap();
    }
}
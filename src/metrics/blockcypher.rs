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

    pub static ref BLOCK_SIZE: IntGauge = register_int_gauge!(
        "bitcoin_block_size",
        concat!(
            "Raw size of block (including header and all transactions) in bytes.",
            " Not returned for bitcoin blocks earlier than height 389104"
        )

    ).unwrap();

    pub static ref BLOCK_TOTAL_VALUE_TRANSACTED: IntGauge = register_int_gauge!(
        "bitcoin_block_total_value_transacted",
        "The total number of satoshis transacted in this block"
    ).unwrap();

    pub static ref BLOCK_TOTAL_FEE: IntGauge = register_int_gauge!(
        "bitcoin_block_total_fee",
        "The total number of fees—in satoshis—collected by miners in this block"
    ).unwrap();

    pub static ref BLOCK_NUMBER_TRANSACTIONS: IntGauge = register_int_gauge!(
        "bitcoin_block_number_of_transactions",
        "Number of transactions in this block"
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
    pub unconfirmed_count: u64,
    pub high_fee_per_kb: u64,
    pub medium_fee_per_kb: u64,
    pub low_fee_per_kb: u64,
    pub last_fork_height: u64
}

// https://www.blockcypher.com/dev/bitcoin/#block
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockCypherBlockResponse {
    pub hash: String,
    pub height: u64,
    pub total: u64,    // total number of value transacted in this block (in satoshi)
    pub fees: u64,     // total number of fee in this block (in satoshi)
    pub size: u64,
    pub n_tx: u64,
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

        let block_response: BlockCypherBlockResponse = client
            .get(&format!("https://api.blockcypher.com/v1/btc/main/blocks/{}", chain_response.hash))
            .send()
            .unwrap()
            .json()
            .unwrap();

        info!("BlockCypher block response {:?}", block_response);

        BLOCK_SIZE.set(block_response.size as i64);
        BLOCK_TOTAL_VALUE_TRANSACTED.set(block_response.total as i64);
        BLOCK_TOTAL_FEE.set(block_response.fees as i64);
        BLOCK_NUMBER_TRANSACTIONS.set(block_response.n_tx as i64)
    }
}
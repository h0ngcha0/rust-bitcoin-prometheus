use prometheus::Gauge;
use serde::{Serialize, Deserialize};
use log::info;

lazy_static! {
    pub static ref COINDESK_BITCOIN_USD_PRICE: Gauge = register_gauge!(
        "bitcoin_coindesk_usd_price",
        "CoinDesk Bitcoin price index (BPI) in USD"
    ).unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinDeskBitcoinPriceIndexInUSD {
    pub code: String,
    pub rate: String,
    pub rate_float: f64
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CoinDeskBitcoinPriceIndex {
    pub USD: CoinDeskBitcoinPriceIndexInUSD
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinDeskBitcionPriceResponse {
    bpi: CoinDeskBitcoinPriceIndex
}

pub struct BitcoinPrice {}

impl super::PrometheusMetrics for BitcoinPrice {
    fn set_metrics(&self) -> () {
        let response: CoinDeskBitcionPriceResponse = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get("https://api.coindesk.com/v1/bpi/currentprice/USD.json")
            .send()
            .unwrap()
            .json()
            .unwrap();

        info!("CoinDesk response {:?}", response);

        COINDESK_BITCOIN_USD_PRICE.set(response.bpi.USD.rate_float);
    }
}

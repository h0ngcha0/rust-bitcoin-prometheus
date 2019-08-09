use prometheus::{TextEncoder, Encoder};
use std::time::Duration;
pub mod coindesk;
pub mod blockcypher;
use super::schedule;

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

pub trait PrometheusMetrics {
    fn set_metrics(&self) -> ();
}

pub struct PrometheusMetricsCollector {
    pub metrics: Vec<Box<dyn PrometheusMetrics>>
}


impl PrometheusMetricsCollector {
    pub fn collect(&self) -> () {
        for metrics in &self.metrics {
            metrics.set_metrics();
        }
    }
}

pub fn collect() -> () {
    schedule::repeat(Duration::from_secs(120), || {
        let collector = PrometheusMetricsCollector {
            metrics: vec![
                Box::new(blockcypher::BitcoinChainGeneralInfo{}),
                Box::new(coindesk::BitcoinPrice{})
            ]
        };

        collector.collect();
    });
}
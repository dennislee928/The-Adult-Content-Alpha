//! 數據抓取與影像 pHash 運算，產出 metadata 至 Kafka。

pub mod kafka_sender;
pub mod rate_limiter;
pub mod scrapers;
pub mod vision;

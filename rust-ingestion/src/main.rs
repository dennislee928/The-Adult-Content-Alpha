//! Ingestion 二進位：依設定週期拉取 X / Reddit 並將事件打入 Kafka。

use adult_content_ingestion::kafka_sender::KafkaSender;
use adult_content_ingestion::rate_limiter::RateLimiter;
use adult_content_ingestion::scrapers::{reddit, x_api};
use std::sync::Arc;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("ingestion=info".parse().unwrap()))
        .init();

    let kafka_brokers = std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let kafka = match KafkaSender::new(&kafka_brokers) {
        Ok(k) => k,
        Err(e) => {
            tracing::error!(error = %e, "kafka connect failed");
            std::process::exit(1);
        }
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("http client");

    let x_limiter = Arc::new(RateLimiter::for_x_api());
    let reddit_limiter = Arc::new(RateLimiter::for_reddit_api());

    loop {
        if let Ok(token) = std::env::var("X_BEARER_TOKEN") {
            x_api::fetch_and_send(&client, Arc::clone(&x_limiter), &kafka, &token).await;
        }
        let user_agent = std::env::var("REDDIT_USER_AGENT")
            .unwrap_or_else(|_| "adult-content-ingestion/0.1".to_string());
        reddit::fetch_and_send(&client, Arc::clone(&reddit_limiter), &kafka, &user_agent).await;

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

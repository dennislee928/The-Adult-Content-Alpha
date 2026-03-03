//! 將社群 metadata 與 pHash 打入 Kafka。

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use std::time::Duration;
use tracing::warn;

/// 送往 Kafka 的社群提及事件（topic: social-mentions）。
#[derive(Debug, Clone, Serialize)]
pub struct SocialMentionEvent {
    pub platform: String,
    pub post_id: String,
    pub author_id: String,
    pub author_handle: String,
    pub content_text: String,
    pub sentiment_score: Option<f32>,
    pub created_at_utc: String,
}

/// 送往 Kafka 的媒體哈希事件（topic: media-hashes）。
#[derive(Debug, Clone, Serialize)]
pub struct MediaHashEvent {
    pub platform: String,
    pub post_id: String,
    pub media_url: String,
    pub phash_hex: String,
    pub created_at_utc: String,
}

pub const TOPIC_SOCIAL_MENTIONS: &str = "social-mentions";
pub const TOPIC_MEDIA_HASHES: &str = "media-hashes";

/// Kafka 發送器，負責將事件序列化後送到對應 topic。
pub struct KafkaSender {
    producer: FutureProducer,
}

impl KafkaSender {
    pub fn new(brokers: &str) -> Result<Self, rdkafka::error::KafkaError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;
        Ok(Self { producer })
    }

    /// 發送社群提及至 `social-mentions`。
    pub async fn send_social_mention(&self, event: &SocialMentionEvent) {
        let payload = match serde_json::to_string(event) {
            Ok(s) => s,
            Err(e) => {
                warn!(error = %e, "serialize SocialMentionEvent");
                return;
            }
        };
        let record = FutureRecord::to(TOPIC_SOCIAL_MENTIONS)
            .payload(&payload)
            .key(&event.post_id);
        if let Err((e, _)) = self.producer.send(record, Duration::from_secs(5)).await {
            warn!(error = %e, "kafka send social-mentions");
        }
    }

    /// 發送媒體 pHash 至 `media-hashes`。
    pub async fn send_media_hash(&self, event: &MediaHashEvent) {
        let payload = match serde_json::to_string(event) {
            Ok(s) => s,
            Err(e) => {
                warn!(error = %e, "serialize MediaHashEvent");
                return;
            }
        };
        let record = FutureRecord::to(TOPIC_MEDIA_HASHES)
            .payload(&payload)
            .key(&event.post_id);
        if let Err((e, _)) = self.producer.send(record, Duration::from_secs(5)).await {
            warn!(error = %e, "kafka send media-hashes");
        }
    }
}

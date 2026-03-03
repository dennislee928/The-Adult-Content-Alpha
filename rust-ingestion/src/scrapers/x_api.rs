//! X (Twitter) API 抓取模組。
//! 需設定環境變數 X_BEARER_TOKEN 或 OAuth 1.0a 憑證。

use crate::kafka_sender::{KafkaSender, SocialMentionEvent};
use crate::rate_limiter::RateLimiter;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Debug, Deserialize)]
pub struct XTweet {
    pub id: String,
    pub text: Option<String>,
    pub author_id: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct XResponse {
    pub data: Option<Vec<XTweet>>,
    pub errors: Option<Vec<XError>>,
}

#[derive(Debug, Deserialize)]
pub struct XError {
    pub detail: Option<String>,
}

const X_RATE_LIMIT_KEY: &str = "x_api";

/// 從 X API 拉取 NSFW 相關推文並送往 Kafka。
pub async fn fetch_and_send(
    client: &reqwest::Client,
    limiter: Arc<RateLimiter>,
    kafka: &KafkaSender,
    bearer_token: &str,
) {
    limiter.wait_until_ready(X_RATE_LIMIT_KEY).await;

    let url = "https://api.twitter.com/2/tweets/search/recent?query=lang:en&max_results=10&tweet.fields=created_at,author_id";
    let res = client
        .get(url)
        .bearer_auth(bearer_token)
        .send()
        .await;

    let res = match res {
        Ok(r) => r,
        Err(e) => {
            warn!(error = %e, "x api request failed");
            return;
        }
    };

    let body = match res.text().await {
        Ok(b) => b,
        Err(e) => {
            warn!(error = %e, "x api response body");
            return;
        }
    };

    let parsed: XResponse = match serde_json::from_str(&body) {
        Ok(p) => p,
        Err(e) => {
            warn!(error = %e, "x api parse json");
            return;
        }
    };

    if let Some(errors) = &parsed.errors {
        for e in errors {
            warn!(detail = ?e.detail, "x api error");
        }
        return;
    }

    let tweets = match &parsed.data {
        Some(t) => t,
        None => return,
    };

    for tweet in tweets {
        let event = SocialMentionEvent {
            platform: "x".to_string(),
            post_id: tweet.id.clone(),
            author_id: tweet.author_id.clone().unwrap_or_default(),
            author_handle: String::new(),
            content_text: tweet.text.clone().unwrap_or_default(),
            sentiment_score: None,
            created_at_utc: tweet.created_at.clone().unwrap_or_default(),
        };
        kafka.send_social_mention(&event).await;
        info!(post_id = %tweet.id, "sent social mention");
    }
}

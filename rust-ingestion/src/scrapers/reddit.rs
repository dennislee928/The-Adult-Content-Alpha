//! Reddit API 抓取模組。
//! 需設定環境變數 REDDIT_CLIENT_ID, REDDIT_CLIENT_SECRET（或 OAuth token）。

use crate::kafka_sender::{KafkaSender, SocialMentionEvent};
use crate::rate_limiter::RateLimiter;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Debug, Deserialize)]
pub struct RedditListing {
    pub data: RedditListingData,
}

#[derive(Debug, Deserialize)]
pub struct RedditListingData {
    pub children: Vec<RedditChild>,
}

#[derive(Debug, Deserialize)]
pub struct RedditChild {
    pub data: RedditPost,
}

#[derive(Debug, Deserialize)]
pub struct RedditPost {
    pub id: String,
    pub author: Option<String>,
    pub title: Option<String>,
    pub selftext: Option<String>,
    pub created_utc: Option<f64>,
    pub subreddit: Option<String>,
}

const REDDIT_RATE_LIMIT_KEY: &str = "reddit_api";

/// 從 Reddit API 拉取貼文並送往 Kafka。
pub async fn fetch_and_send(
    client: &reqwest::Client,
    limiter: Arc<RateLimiter>,
    kafka: &KafkaSender,
    user_agent: &str,
) {
    limiter.wait_until_ready(REDDIT_RATE_LIMIT_KEY).await;

    let url = "https://oauth.reddit.com/r/nsfw/new?limit=10";
    let res = client
        .get(url)
        .header("User-Agent", user_agent)
        .send()
        .await;

    let res = match res {
        Ok(r) => r,
        Err(e) => {
            warn!(error = %e, "reddit api request failed");
            return;
        }
    };

    let body = match res.text().await {
        Ok(b) => b,
        Err(e) => {
            warn!(error = %e, "reddit api response body");
            return;
        }
    };

    let listing: RedditListing = match serde_json::from_str(&body) {
        Ok(p) => p,
        Err(e) => {
            warn!(error = %e, "reddit api parse json");
            return;
        }
    };

    for child in listing.data.children {
        let post = &child.data;
        let created_at_utc = post
            .created_utc
            .map(|t| format!("{}", t))
            .unwrap_or_default();
        let content = post
            .title
            .clone()
            .unwrap_or_default()
            + &post.selftext.clone().unwrap_or_default();

        let event = SocialMentionEvent {
            platform: "reddit".to_string(),
            post_id: post.id.clone(),
            author_id: post.author.clone().unwrap_or_default(),
            author_handle: post.author.clone().unwrap_or_default(),
            content_text: content,
            sentiment_score: None,
            created_at_utc,
        };
        kafka.send_social_mention(&event).await;
        info!(post_id = %post.id, "sent reddit mention");
    }
}

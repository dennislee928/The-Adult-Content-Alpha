//! 嚴格的 API 請求頻率控制，避免觸發 X / Reddit 限流。

use governor::{
    quota::Quota,
    state::keyed::DashMapStateStore,
    RateLimiter as GovernorLimiter,
};
use std::num::NonZeroU32;
use std::time::Duration;

/// 依 key（例如 API 名稱）做限流的速率限制器。
pub struct RateLimiter {
    limiter: GovernorLimiter<String, DashMapStateStore<String>, governor::clock::QuantaInstant>,
}

impl RateLimiter {
    /// 建立限流器：每 `refill_interval` 內最多 `max_requests` 次。
    pub fn new(max_requests: u32, refill_interval: Duration) -> Self {
        let quota = Quota::per_interval(
            NonZeroU32::new(max_requests).expect("max_requests must be > 0"),
            refill_interval,
        );
        let limiter = GovernorLimiter::keyed(quota);
        Self { limiter }
    }

    /// 針對 X API 的建議限流（例如每 15 分鐘 180 次）。
    pub fn for_x_api() -> Self {
        Self::new(180, Duration::from_secs(15 * 60))
    }

    /// 針對 Reddit API 的建議限流（例如每分鐘 60 次）。
    pub fn for_reddit_api() -> Self {
        Self::new(60, Duration::from_secs(60))
    }

    /// 等待直到可發送請求為止（會 block 當前 task）。
    pub async fn wait_until_ready(&self, key: &str) {
        while self.limiter.check_key(key).is_err() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// 若可發送則消耗一個額度並回傳 true，否則回傳 false。
    pub fn try_acquire(&self, key: &str) -> bool {
        self.limiter.check_key(key).is_ok()
    }
}

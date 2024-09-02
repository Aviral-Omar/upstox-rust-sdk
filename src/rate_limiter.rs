use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum RateLimitExceeded {
    PerSecond { next_allowed_at: Instant },
    PerMinute { next_allowed_at: Instant },
    PerThirtyMinutes { next_allowed_at: Instant },
}

#[derive(Debug)]
pub struct RateLimiter {
    per_second: usize,
    per_minute: usize,
    per_thirty_minutes: usize,
    requests: Arc<Mutex<VecDeque<Instant>>>,
}

impl RateLimiter {
    fn new(per_second: usize, per_minute: usize, per_thirty_minutes: usize) -> Self {
        Self {
            per_second,
            per_minute,
            per_thirty_minutes,
            requests: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    async fn check_rate_limit(&self) -> Option<RateLimitExceeded> {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();

        // Remove outdated requests
        requests.retain(|&time| now.duration_since(time) <= Duration::from_secs(1800)); // 30 minutes

        // Check per-second limit
        let per_second_count = requests
            .iter()
            .filter(|&&time| now.duration_since(time) < Duration::from_secs(1))
            .count();
        if per_second_count >= self.per_second {
            let next_allowed_at = requests
                .iter()
                .find(|&&time| now.duration_since(time) < Duration::from_secs(1))
                .map(|&time| time + Duration::from_secs(1))
                .unwrap_or(now + Duration::from_secs(1));
            return Some(RateLimitExceeded::PerSecond { next_allowed_at });
        }

        // Check per-minute limit
        let per_minute_count = requests
            .iter()
            .filter(|&&time| now.duration_since(time) < Duration::from_secs(60))
            .count();
        if per_minute_count >= self.per_minute {
            let next_allowed_at = requests
                .iter()
                .find(|&&time| now.duration_since(time) < Duration::from_secs(60))
                .map(|&time| time + Duration::from_secs(60))
                .unwrap_or(now + Duration::from_secs(60));
            return Some(RateLimitExceeded::PerMinute { next_allowed_at });
        }

        // Check per-thirty-minutes limit
        let per_thirty_minutes_count = requests.len();
        if per_thirty_minutes_count >= self.per_thirty_minutes {
            let next_allowed_at = requests
                .iter()
                .find(|&&time| now.duration_since(time) < Duration::from_secs(1800))
                .map(|&time| time + Duration::from_secs(1800))
                .unwrap_or(now + Duration::from_secs(1800));
            return Some(RateLimitExceeded::PerThirtyMinutes { next_allowed_at });
        }

        // Record the new request
        requests.push_back(now);
        None
    }
}

#[derive(Debug)]
pub struct ApiRateLimiter {
    rate_limiters: Mutex<HashMap<String, RateLimiter>>,
    per_second: usize,
    per_minute: usize,
    per_thirty_minutes: usize,
}

impl ApiRateLimiter {
    pub(super) fn new(per_second: usize, per_minute: usize, per_thirty_minutes: usize) -> Self {
        Self {
            rate_limiters: Mutex::new(HashMap::new()),
            per_second,
            per_minute,
            per_thirty_minutes,
        }
    }

    pub(super) async fn check_rate_limit(&self, endpoint: &str) -> Option<RateLimitExceeded> {
        let mut rate_limiters = self.rate_limiters.lock().await;

        // Insert a new rate limiter for the endpoint if it doesn't exist
        let rate_limiter: &mut RateLimiter = rate_limiters
            .entry(endpoint.to_string())
            .or_insert_with(|| {
                RateLimiter::new(self.per_second, self.per_minute, self.per_thirty_minutes)
            });

        rate_limiter.check_rate_limit().await
    }
}

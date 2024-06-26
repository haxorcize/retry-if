//! This example tests a backoff configuration using a function that returns false for if it should
//! retry, thus resulting in no retries at all.
use retry_if::{retry, ExponentialBackoffConfig};
use std::time::Duration;
use tokio::time::Instant;

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(1),
    backoff: 2.0,
    t_wait_max: None,
    backoff_max: None,
};

#[tokio::test]
async fn test_retry_if() {
    fn retry_if(_: &()) -> bool {
        false
    }

    #[retry(BACKOFF_CONFIG, retry_if)]
    async fn method() {}

    let start = Instant::now();
    method().await;
    let end = Instant::now();
    let duration = end - start;

    assert!(duration > Duration::from_millis(0));
    assert!(duration < Duration::from_millis(100));
}

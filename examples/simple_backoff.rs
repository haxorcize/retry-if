use retry_if::{retry, ExponentialBackoffConfig};
use std::num::TryFromIntError;
use std::time::Duration;
use tokio::time::{pause, Instant};

// A backoff that will retry a failing function up to 5 times, waiting 1s the first retry and
//  doubling the wait with each retry.
// It specifies no overall limit for retries (t_wait_max), nor a maximum for individual retry waits
//  (backoff_max).
const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(1),
    backoff: 2.0,
    t_wait_max: None,
    backoff_max: None,
};

// flavor = "current_thread" is only needed for Tokio's `pause()` functionality to work
#[tokio::main(flavor = "current_thread")]
async fn main() {
    pause();
    let start = Instant::now();

    let _ = fallible_call().await;

    let end = Instant::now();

    let elapsed = end - start;

    // expected waits are 1s, 2s, 4s, 8s, 16s = 31s
    assert!(elapsed > Duration::from_secs(31));
    assert!(elapsed < Duration::from_millis(31100));

    println!("Total test time: {elapsed:?}")
}

// this takes any argument of the same type as the output of the decorated function
//  it returns a boolean specifying if the function should be retried based on the result
fn retry_if(result: &Result<i64, TryFromIntError>) -> bool {
    result.is_err()
}

#[retry(BACKOFF_CONFIG, retry_if)]
async fn fallible_call() -> Result<i64, TryFromIntError> {
    // this will always produce a TryFromIntError, triggering a retry
    i64::try_from(i128::MAX)
}

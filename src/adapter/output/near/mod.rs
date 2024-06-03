pub mod error;
pub mod rpc_client;

use self::error::{Error, Result};
use std::time::Duration;
use tokio::time::sleep;

pub const MAX_RETRY_COUNT: usize = 10;
pub const RETRY_DELAY: Duration = Duration::from_secs(1);

pub async fn retry_async<F, T>(mut task: F, max_attempts: usize, retry_delay: Duration) -> Result<T>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>,
{
    let mut attempt = 0;

    loop {
        attempt += 1;
        match task().await {
            Ok(result) => return Ok(result),
            Err(err) if attempt < max_attempts => {
                match err {
                    Error::CustomInvalidNonce 
                    | Error::CustomInvalidSignature => {
                        tracing::warn!("Attempt {}/{} failed with error: {:?}. Retrying in {:?}...", attempt, max_attempts, err, retry_delay);
                        sleep(retry_delay).await;
                        continue;
                    },
                    _ => return Err(err),
                }
            },
            Err(err) => return Err(err),
        }
    }
}
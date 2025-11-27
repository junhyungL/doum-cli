use crate::system::error::{DoumError, DoumResult};
use std::future::Future;

/// Retry LLM call with parsing when either the call or parsing fails
pub async fn retry_with_parse<T, F, Fut, P>(
    llm_call: F,
    parser: P,
    max_retries: u32,
) -> DoumResult<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = DoumResult<String>>,
    P: Fn(&str) -> DoumResult<T>,
{
    let mut last_error = None;

    for attempt in 1..=max_retries {
        // Call LLM
        let response = match llm_call().await {
            Ok(resp) => resp,
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    tracing::warn!(
                        "LLM call failed (attempt {}/{}): Retrying...",
                        attempt,
                        max_retries
                    );
                    continue;
                } else {
                    tracing::error!("All retry attempts exhausted.");
                    break;
                }
            }
        };

        // Parse response
        match parser(&response) {
            Ok(parsed) => return Ok(parsed),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    tracing::warn!(
                        "Parse failed (attempt {}/{}): Retrying...",
                        attempt,
                        max_retries
                    );
                    continue;
                } else {
                    tracing::error!("All retry attempts exhausted.");
                    break;
                }
            }
        }
    }

    // All retries exhausted
    Err(last_error.unwrap_or_else(|| DoumError::LLM("Unknown error after retries".to_string())))
}

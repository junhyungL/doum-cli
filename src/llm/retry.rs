use crate::system::error::{DoumError, Result};
use std::future::Future;

/// LLM 호출 및 파싱 재시도
///
/// LLM이 잘못된 형식으로 응답하는 경우를 대비하여 재시도 로직을 제공합니다.
pub async fn retry_with_parse<T, F, Fut, P>(llm_call: F, parser: P, max_retries: u32) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<String>>,
    P: Fn(&str) -> Result<T>,
{
    let mut last_error = None;

    for attempt in 1..=max_retries {
        // LLM 호출
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

        // 파싱 시도
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

    // 모든 재시도 실패
    Err(last_error.unwrap_or_else(|| DoumError::LLM("Unknown error after retries".to_string())))
}

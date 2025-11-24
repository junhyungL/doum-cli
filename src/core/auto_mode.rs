use crate::llm::Message;
use crate::llm::client::LLMRequest;
use crate::system::error::Result;
use crate::system::Config;
use crate::system::SystemInfo;
use crate::llm::{LLMClient, PromptBuilder, parse_mode_select};
use crate::core::{handle_ask, handle_suggest};
use crate::llm::retry_with_parse;
use crate::cli::ui::{create_spinner, finish_spinner};

/// 자동 모드 선택 및 실행
/// 
/// 사용자 입력을 분석하여 적절한 모드(ask/suggest/execute)를 선택하고 실행합니다.
pub async fn select_and_execute(
    input: &str,
    client: &dyn LLMClient,
    system_info: &SystemInfo,
    config: &Config,
) -> Result<()> {
    // 프롬프트 빌더 생성
    let builder = PromptBuilder::new(system_info.clone());
    
    // 스피너 시작
    let spinner = create_spinner("Analyzing input...");
    
    // LLM에게 모드 선택 요청
    let mode_response = retry_with_parse(
        || {
            let request = LLMRequest {
                system: builder.build_mode_select(),
                messages: vec![Message::user(input)],
                use_websearch: false,
            };
            Box::pin(client.generate(request))
        },
        |content| parse_mode_select(content),
        config.llm.max_retries,
    )
    .await?;
    
    // 스피너 완료 및 선택된 모드 출력
    finish_spinner(spinner, None);
    println!("📌 Selected mode: {} mode\n", mode_response.mode);
    
    // 해당 모드 실행
    match mode_response.mode.as_str() {
        "ask" => {
            handle_ask(input, client, system_info, config).await
        }
        "suggest" => {
            handle_suggest(input, client, system_info, config).await?;
            Ok(())
        }
        "execute" => {
            // Execute 모드는 suggest로 통합됨 (suggest에서 실행 선택 가능)
            handle_suggest(input, client, system_info, config).await?;
            Ok(())
        }
        unknown => {
            println!("⚠️  Unknown mode: {}", unknown);
            println!("💡 Falling back to Ask mode.\n");
            handle_ask(input, client, system_info, config).await
        }
    }
}
use crate::cli::ui::{create_spinner, finish_spinner};
use crate::llm::Message;
use crate::llm::client::LLMRequest;
use crate::llm::{LLMClient, PromptBuilder};
use crate::system::Config;
use crate::system::SystemInfo;
use crate::system::error::Result;

/// Ask 모드 핵심 로직
///
/// 사용자의 질문에 대해 LLM이 답변을 제공합니다.
pub async fn handle_ask(
    question: &str,
    client: &dyn LLMClient,
    system_info: &SystemInfo,
    _config: &Config,
) -> Result<()> {
    // 프롬프트 빌더 생성
    let builder = PromptBuilder::new(system_info.clone());

    // Ask 모드용 메시지 생성
    let request = LLMRequest {
        system: builder.build_ask(),
        messages: vec![Message::user(question)],
        use_websearch: true,
    };

    // 스피너 시작
    let spinner = create_spinner("AI is generating an answer...");

    // LLM 호출
    let response = client.generate(request).await?;

    // 스피너 완료
    finish_spinner(spinner, None);

    // 응답 출력
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{}\n", response);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    Ok(())
}

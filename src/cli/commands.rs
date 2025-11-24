use crate::system::error::Result;
use crate::system::{load_config, save_config, get_config_path, load_default_config, get_system_info};
use crate::llm::create_client;
use crate::core::{handle_ask, handle_suggest, select_and_execute};
use super::args::ConfigAction;

/// config 커맨드 처리
pub fn handle_config_command(action: Option<ConfigAction>) -> Result<()> {
    // action이 None이면 interactive 모드로
    let action = action.unwrap_or(ConfigAction::Interactive);
    
    match action {
        ConfigAction::Interactive => {
            crate::cli::config::run_config_interactive()
        }
        ConfigAction::Show => show_config(),
        ConfigAction::Reset => reset_config(),
    }
}

/// 전체 설정 표시
fn show_config() -> Result<()> {
    let config = load_config()?;
    let config_path = get_config_path()?;
    
    println!("Config file location: {}\n", config_path.display());
    
    let toml_str = toml::to_string_pretty(&config)?;
    
    println!("{}", toml_str);
    
    Ok(())
}

/// 설정을 기본값으로 리셋
fn reset_config() -> Result<()> {
    let default_config = load_default_config()?;
    save_config(&default_config)?;
    
    println!("✅ Configuration reset to default");
    
    Ok(())
}

/// ask 커맨드 처리
pub async fn handle_ask_command(question: &str) -> Result<()> {
    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();
    handle_ask(question, client.as_ref(), &system_info, &config).await
}

/// suggest 커맨드 처리
pub async fn handle_suggest_command(request: &str) -> Result<()> {
    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();
    handle_suggest(request, client.as_ref(), &system_info, &config).await?;
    Ok(())
}

/// auto 커맨드 처리 (모드 자동 선택)
pub async fn handle_auto_command(input: &str) -> Result<()> {
    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();
    select_and_execute(input, client.as_ref(), &system_info, &config).await
}

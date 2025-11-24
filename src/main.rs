use doum_cli::system::error::Result;
use doum_cli::cli::{Cli, Commands};
use doum_cli::system::{load_config, load_default_config, init_logging};
use clap::Parser;

#[tokio::main]
async fn main() {
    let exit_code = match run().await {
        Ok(_) => 0,
        Err(e) => {
            tracing::error!("에러 발생: {}", e);
            eprintln!("\n{}\n", e.user_message());
            1
        }
    };
    
    std::process::exit(exit_code);
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    
    // 설정 로드
    let config = load_config().unwrap_or_else(|e| {
        eprintln!("⚠️  Failed to load config: {}. Using default config.", e);
        load_default_config().expect("Failed to load default config")
    });
    
    // 로깅 초기화
    if let Err(e) = init_logging(&config) {
        eprintln!("⚠️  Failed to initialize logging: {}", e);
    }
    
    tracing::info!("starting doum-cli");
    
    let result = match cli.command {
        Some(Commands::Config { action }) => {
            tracing::info!("Exec Config command");
            doum_cli::cli::handle_config_command(action)?;
            Ok(())
        },
        Some(Commands::Ask { question }) => {
            tracing::info!("Exec Ask mode: {}", question);
            doum_cli::cli::handle_ask_command(&question).await
        },
        Some(Commands::Suggest { request }) => {
            tracing::info!("Exec Suggest mode: {}", request);
            doum_cli::cli::handle_suggest_command(&request).await
        },
        None => {
            if let Some(input) = cli.input {
                tracing::info!("Exec Auto mode: {}", input);
                doum_cli::cli::handle_auto_command(&input).await
            } else {
                // 인자가 없다면 커맨드 설명
                tracing::info!("명령어 인자 없음 - 도움말 출력");
                Cli::parse_from(["doum-cli", "--help"]);
                Ok(())
            }
        }
    };
    
    tracing::info!("doum-cli 정상 종료");
    result
}

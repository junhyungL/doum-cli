use crate::system::config::Config;
use crate::system::error::{DoumError, DoumResult};
use crate::system::paths::get_log_dir;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// 로깅 시스템 초기화
pub fn init_logging(config: &Config) -> DoumResult<()> {
    if !config.logging.enabled {
        return Ok(());
    }

    // 로그 레벨 설정
    let level = match config.logging.level.as_str() {
        "debug" => "debug",
        "info" => "info",
        "warn" => "warn",
        "error" => "error",
        _ => "info",
    };

    let filter = EnvFilter::try_new(format!("doum_cli={}", level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // 로그 디렉터리 경로 가져오기
    let log_dir = get_log_dir()?;

    // 로그 디렉터리 생성
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir)
            .map_err(|e| DoumError::Config(format!("로그 디렉터리 생성 실패: {}", e)))?;
    }

    // 파일 appender 설정 (일별 로테이션, {날짜}.log 형식)
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix("log")
        .build(&log_dir)
        .map_err(|e| DoumError::Config(format!("로그 파일 생성 실패: {}", e)))?;

    // Subscriber 설정
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(file_appender)
                .with_ansi(false)
                .with_target(false),
        )
        .try_init()
        .map_err(|e| DoumError::Config(format!("로깅 초기화 실패: {}", e)))?;

    tracing::info!("로깅 시스템 초기화 완료 (레벨: {})", level);
    Ok(())
}

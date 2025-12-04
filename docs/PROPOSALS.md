# Command Options & Feature Proposals

## 📋 1. 커맨드 옵션 제안

### A. Ask 명령어 옵션
```rust
Ask {
    /// Question to ask
    question: String,
    
    /// Stream response (live output)
    #[arg(short = 's', long = "stream")]
    stream: bool,
}
```

**사용 예시:**
```bash
doum ask "Explain Rust" --stream  # 실시간 스트리밍
```

---

### B. Suggest 명령어 옵션
```rust
Suggest {
    /// Request description
    request: String,

    /// Number of suggestions to generate
    #[arg(short = 'n', long = "num", default_value = "3")]
    num_suggestions: usize,
}
```

**사용 예시:**
```bash
doum suggest "monitor CPU" --num 5            # 5개 제안 생성
```

---

### D. Secret 명령어 옵션
```rust
Secret {
    /// List all configured secrets (masked)
    #[arg(short = 'l', long = "list")]
    list: bool,
    
    /// Delete/remove secret
    #[arg(short = 'd', long = "delete")]
    delete: bool,
    
    /// Verify secret (test API call)
    #[arg(short = 'v', long = "verify")]
    verify: bool,
}
```

**사용 예시:**
```bash
doum secret --list                    # 모든 secret 상태 확인
doum secret openai --verify           # OpenAI API 키 검증
doum secret anthropic --delete        # Anthropic secret 삭제
```

---

### E. Switch 명령어 옵션
```rust
Switch {
    /// Show current selection
    #[arg(short = 's', long = "show")]
    show: bool,
}
```

**사용 예시:**
```bash
doum switch --show                    # 현재 provider/model 표시
```

---

### F. Global 옵션
```rust
#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Auto mode input
    #[arg(value_name = "INPUT")]
    pub input: Option<String>,
    
    /// Verbose output (debug info)
    #[arg(short = 'v', long = "verbose", global = true)]
    pub verbose: bool,
    
    /// Quiet mode (minimal output)
    #[arg(short = 'q', long = "quiet", global = true)]
    pub quiet: bool,
    
    /// Use specific provider for this command
    #[arg(long = "provider", global = true)]
    pub provider_override: Option<String>,
    
    /// Use specific model for this command
    #[arg(long = "model", global = true)]
    pub model_override: Option<String>,
    
    /// Disable web search for this command
    #[arg(long = "no-web-search", global = true)]
    pub no_web_search: bool,
}
```

**사용 예시:**
```bash
doum ask "What is Rust?" --verbose                    # 디버그 정보 표시
doum suggest "find files" --quiet                     # 최소 출력
doum ask "latest news" --provider anthropic           # 일회성 provider 변경
doum suggest "list processes" --model gpt-5-mini      # 일회성 model 변경
doum ask "Define AI" --no-web-search                  # 웹 검색 비활성화
```

---

## 🚀 2. 추가 명령어 제안

### A. `history` - 명령 이력 관리
```bash
doum history [OPTIONS]
```

**옵션:**
```rust
History {
    /// Show last N commands
    #[arg(short = 'n', default_value = "10")]
    num: usize,
    
    /// Search history
    #[arg(short = 's', long = "search")]
    search: Option<String>,
    
    /// Clear history
    #[arg(long = "clear")]
    clear: bool,
    
    /// Re-run command by ID
    #[arg(short = 'r', long = "rerun")]
    rerun: Option<usize>,
}
```

**사용 예시:**
```bash
doum history                      # 최근 10개 표시
doum history -n 50                # 최근 50개
doum history --search "docker"    # "docker" 포함 검색
doum history --clear              # 이력 삭제
```

**저장 형식** (`~/.config/doum-cli/history.json`):
```json
[
  {
    "id": 1,
    "timestamp": "2025-11-26T14:30:22Z",
    "command": "ask",
    "input": "What is Docker?",
    "provider": "openai",
    "model": "gpt-5",
    "success": true
  }
]
```

---

### B. `upgrade` - 자동 업데이트
```bash
doum upgrade [OPTIONS]
```

**옵션:**
```rust
Upgrade {
    /// Check for updates without installing
    #[arg(short = 'c', long = "check")]
    check: bool,
    
    /// Upgrade to specific version
    #[arg(short = 'v', long = "version")]
    version: Option<String>,
}
```

**동작:**
1. GitHub Releases API 확인
2. 현재 버전과 비교
3. 새 버전이 있으면 다운로드 및 설치

**사용 예시:**
```bash
doum upgrade                      # 최신 버전으로 업데이트
doum upgrade --check              # 업데이트 가능 여부만 확인
doum upgrade --version 0.2.5      # 특정 버전으로
```

**구현 아이디어:**
```rust
// src/cli/upgrade.rs
use reqwest;
use semver::Version;

pub async fn check_for_updates() -> Result<Option<String>> {
    let current = env!("CARGO_PKG_VERSION");
    let url = "https://api.github.com/repos/junhyungL/doum-cli/releases/latest";
    
    let response: serde_json::Value = reqwest::get(url).await?.json().await?;
    let latest = response["tag_name"].as_str().unwrap().trim_start_matches('v');
    
    if Version::parse(latest)? > Version::parse(current)? {
        Ok(Some(latest.to_string()))
    } else {
        Ok(None)
    }
}
```

---

### C. `context` - 컨텍스트 관리
```bash
doum context [OPTIONS]
```

**기능:**
- 현재 디렉토리의 파일/구조를 LLM 컨텍스트로 포함
- 프로젝트별 컨텍스트 저장

**옵션:**
```rust
Context {
    /// Add files to context
    #[arg(short = 'a', long = "add")]
    add: Vec<PathBuf>,
    
    /// Show current context
    #[arg(short = 's', long = "show")]
    show: bool,
}
```

**사용 예시:**
```bash
doum context --add src/**/*.rs           # Rust 파일 추가
doum context --show                      # 현재 컨텍스트 표시
doum ask "Refactor this code" --with-context  # 컨텍스트 포함 질문
```

---

## 📌 3. 버전 관리 방법 제안

### A. Semantic Versioning (semver)
```toml
[package]
version = "0.1.0"  # MAJOR.MINOR.PATCH
```

**규칙:**
- **MAJOR**: Breaking changes (config 형식 변경 등)
- **MINOR**: 새 기능 추가 (새 명령어, 옵션)
- **PATCH**: 버그 수정

---

### B. Upgrade 명령어 구현

**Cargo.toml**:
```toml
[dependencies]
self_update = "0.39"  # 자동 업데이트 라이브러리
```

**src/cli/upgrade.rs**:
```rust
use self_update::backends::github::{ReleaseList, Update};
use self_update::cargo_crate_version;

pub fn handle_upgrade_command(check_only: bool, force: bool) -> Result<()> {
    let current_version = cargo_crate_version!();
    
    println!("🔍 Checking for updates...");
    println!("Current version: {}", current_version);
    
    let releases = ReleaseList::configure()
        .repo_owner("junhyungL")
        .repo_name("doum-cli")
        .build()?
        .fetch()?;
    
    if let Some(latest) = releases.first() {
        let latest_version = latest.version.trim_start_matches('v');
        
        if latest_version > current_version || force {
            if check_only {
                println!("✨ New version available: {}", latest_version);
                return Ok(());
            }
            
            println!("📥 Downloading version {}...", latest_version);
            
            Update::configure()
                .repo_owner("junhyungL")
                .repo_name("doum-cli")
                .bin_name("doum")
                .current_version(current_version)
                .build()?
                .update()?;
            
            println!("✅ Successfully upgraded to {}", latest_version);
        } else {
            println!("✅ Already up-to-date!");
        }
    }
    
    Ok(())
}
```

---

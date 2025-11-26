# Command Options & Feature Proposals

## 📋 1. 커맨드 옵션 제안

### A. Ask 명령어 옵션
```rust
Ask {
    /// Question to ask
    question: String,
    
    /// Output format (text, json, markdown)
    #[arg(short = 'f', long = "format", default_value = "text")]
    format: String,
    
    /// Save response to file
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,
    
    /// Stream response (live output)
    #[arg(short = 's', long = "stream")]
    stream: bool,
}
```

**사용 예시:**
```bash
doum ask "What is Docker?" --format markdown --output docker.md
doum ask "Explain Rust" --stream  # 실시간 스트리밍
doum ask "Compare SQL vs NoSQL" --format json
```

---

### B. Suggest 명령어 옵션
```rust
Suggest {
    /// Request description
    request: String,
    
    /// Auto-execute first command (no interaction)
    #[arg(short = 'y', long = "yes")]
    auto_execute: bool,
    
    /// Copy to clipboard instead of executing
    #[arg(short = 'c', long = "copy")]
    copy_only: bool,
    
    /// Number of suggestions to generate
    #[arg(short = 'n', long = "num", default_value = "3")]
    num_suggestions: usize,
    
    /// Dry run (show commands but don't execute)
    #[arg(long = "dry-run")]
    dry_run: bool,
}
```

**사용 예시:**
```bash
doum suggest "find large files" --yes         # 첫 번째 명령 자동 실행
doum suggest "compress folder" --copy         # 클립보드에만 복사
doum suggest "monitor CPU" --num 5            # 5개 제안 생성
doum suggest "backup database" --dry-run      # 실행 없이 미리보기
```

---

### C. Config 명령어 옵션
```rust
Config {
    #[command(subcommand)]
    action: Option<ConfigAction>,
    
    /// Use specific config profile
    #[arg(short = 'p', long = "profile")]
    profile: Option<String>,
    
    /// Export config to file
    #[arg(short = 'e', long = "export")]
    export: Option<PathBuf>,
    
    /// Import config from file
    #[arg(short = 'i', long = "import")]
    import: Option<PathBuf>,
}
```

**사용 예시:**
```bash
doum config --profile production
doum config --export config-backup.toml
doum config --import team-config.toml
```

---

### D. Secret 명령어 옵션
```rust
Secret {
    /// Provider name (openai, anthropic)
    provider: Option<String>,
    
    /// List all configured secrets (masked)
    #[arg(short = 'l', long = "list")]
    list: bool,
    
    /// Delete/remove secret
    #[arg(short = 'd', long = "delete")]
    delete: bool,
    
    /// Verify secret (test API call)
    #[arg(short = 'v', long = "verify")]
    verify: bool,
    
    /// Export secrets (encrypted)
    #[arg(long = "export")]
    export: Option<PathBuf>,
}
```

**사용 예시:**
```bash
doum secret --list                    # 모든 secret 상태 확인
doum secret openai --verify           # OpenAI API 키 검증
doum secret anthropic --delete        # Anthropic secret 삭제
doum secret --export secrets.enc      # 암호화된 백업
```

---

### E. Switch 명령어 옵션
```rust
Switch {
    /// Provider name (optional)
    provider: Option<String>,
    /// Model name (optional)
    model: Option<String>,
    
    /// List available providers and models
    #[arg(short = 'l', long = "list")]
    list: bool,
    
    /// Show current selection
    #[arg(short = 's', long = "show")]
    show: bool,
}
```

**사용 예시:**
```bash
doum switch --list                    # 사용 가능한 모든 조합 표시
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

### A. `doctor` - 시스템 진단
```bash
doum doctor
```

**기능:**
- Config 파일 검증
- Secret 상태 확인 (각 provider별 API 키 존재 여부)
- API 연결 테스트
- 권장 설정 제안

**출력 예시:**
```
🔍 Diagnosing doum-cli...

✅ Config: OK
   Location: ~/.config/doum-cli/config.toml
   Provider: openai
   Model: gpt-5

✅ OpenAI Secret: OK
   API Key: sk-proj-...abc123 (verified)
   
❌ Anthropic Secret: Not Found
   💡 Run: doum secret anthropic

⚠️  Recommendations:
   - Consider increasing timeout to 60s for complex queries
   - Enable logging for debugging: doum config set logging.enabled true

Overall Status: 1 warning, 1 error
```

---

### B. `history` - 명령 이력 관리
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
doum history --rerun 5            # 5번 명령 재실행
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

### C. `upgrade` - 자동 업데이트
```bash
doum upgrade [OPTIONS]
```

**옵션:**
```rust
Upgrade {
    /// Check for updates without installing
    #[arg(short = 'c', long = "check")]
    check_only: bool,
    
    /// Force upgrade even if up-to-date
    #[arg(short = 'f', long = "force")]
    force: bool,
    
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

### D. `alias` - 명령 별칭
```bash
doum alias [NAME] [COMMAND]
```

**사용 예시:**
```bash
# 별칭 생성
doum alias ports "suggest check if port 8080 is open"
doum alias docker-help "ask explain docker commands"

# 사용
doum ports          # → doum suggest "check if port 8080 is open"
doum docker-help    # → doum ask "explain docker commands"

# 관리
doum alias --list              # 모든 별칭 표시
doum alias --delete ports      # ports 별칭 삭제
```

**저장** (`~/.config/doum-cli/aliases.toml`):
```toml
[aliases]
ports = "suggest check if port 8080 is open"
docker-help = "ask explain docker commands"
backup = "suggest backup database to /tmp"
```

---

### E. `template` - 프롬프트 템플릿
```bash
doum template [OPTIONS]
```

**옵션:**
```rust
Template {
    /// Template name
    name: Option<String>,
    
    /// List all templates
    #[arg(short = 'l', long = "list")]
    list: bool,
    
    /// Create new template
    #[arg(short = 'c', long = "create")]
    create: bool,
    
    /// Delete template
    #[arg(short = 'd', long = "delete")]
    delete: Option<String>,
}
```

**사용 예시:**
```bash
# 템플릿 생성
doum template --create code-review
# → 입력: "Review this {{language}} code for best practices: {{code}}"

# 템플릿 사용
doum template code-review --language rust --code "fn main() { ... }"

# 목록 표시
doum template --list
```

---

### F. `benchmark` - 성능 비교
```bash
doum benchmark [QUERY]
```

**기능:**
- 동일한 질문을 여러 provider/model로 실행
- 응답 시간, 토큰 사용량, 품질 비교

**사용 예시:**
```bash
doum benchmark "Explain async/await in Rust"
```

**출력:**
```
🏁 Benchmarking: "Explain async/await in Rust"

┌──────────────┬──────────┬────────┬──────────┐
│ Provider     │ Model    │ Time   │ Tokens   │
├──────────────┼──────────┼────────┼──────────┤
│ openai       │ gpt-5    │ 2.3s   │ 450      │
│ openai       │ gpt-4.1  │ 3.1s   │ 380      │
│ anthropic    │ claude   │ 1.8s   │ 520      │
└──────────────┴──────────┴────────┴──────────┘

🏆 Fastest: anthropic/claude (1.8s)
💰 Most Efficient: openai/gpt-4.1 (380 tokens)
```

---

### G. `context` - 컨텍스트 관리
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
    
    /// Clear context
    #[arg(short = 'c', long = "clear")]
    clear: bool,
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

## 🎯 우선순위

1. **High Priority** (즉시 구현):
   - Global options (`--verbose`, `--quiet`, `--provider`, `--model`)
   - `doctor` 명령어 (진단 도구)
   - `upgrade` 명령어 (자동 업데이트)
   - Suggest/Ask 옵션 (`--yes`, `--copy`, `--output`)

2. **Medium Priority** (다음 릴리스):
   - `history` 명령어
   - `alias` 명령어
   - Secret 옵션 (`--list`, `--verify`, `--delete`)

3. **Low Priority** (향후 고려):
   - `template` 명령어
   - `benchmark` 명령어
   - `context` 관리
   - Config profiles

---

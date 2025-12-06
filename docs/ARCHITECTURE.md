# Architecture

## Overview

doum-cli is an LLM-based CLI tool that generates appropriate OS commands or provides answers based on natural language queries.

```
User Input → CLI Parser → Commands Handler → LLM Client → Response → UI
```

## Module Structure

```
src/
├── main.rs             # Entry point & routing
├── cli/                # CLI interface
│   ├── args.rs         # Command line arguments
│   ├── ask.rs          # Ask command handler
│   ├── suggest.rs      # Suggest command handler
│   ├── auto_mode.rs    # Auto mode handler
│   ├── secret.rs       # Secret command handler
│   ├── switch.rs       # Switch command handler
│   └── config.rs       # Config command handler
├── llm/                # LLM integration
│   ├── client.rs       # LLM client trait & verify_config
│   ├── provider.rs     # Provider enum
│   ├── prompt.rs       # Prompt templates
│   ├── parser.rs       # Response parsing
│   ├── presets.rs      # Provider/Model presets
│   ├── openai/         # OpenAI implementation
│   └── anthropic/      # Anthropic implementation
├── system/             # System utilities
│   ├── config.rs       # Configuration management
│   ├── env.rs          # OS/Shell detection
│   ├── paths.rs        # Path utilities
│   ├── secret.rs       # Secret storage (OS keyring)
│   └── logging.rs      # Logging setup
└── tools/              # Tool execution
    └── executor.rs     # Command executor
```

## Key Components

### 1. CLI Layer (`cli/`)
- **Each command is a self-contained module** with UI and business logic using `cliclack`
- **ask.rs**: Question answering with spinner feedback (includes LLM request logic)
- **suggest.rs**: Command suggestions with clipboard copy (includes parsing and retry logic)
- **auto_mode.rs**: Automatic mode selection (includes LLM-based mode detection)
- **secret.rs**: API key configuration with verification (includes secret management)
- **switch.rs**: Provider/Model switching with 2-step selection (includes config update)
- **config.rs**: Configuration operations (set/get/unset/show/reset with value validation)

### 2. LLM Integration (`llm/`)
- **Client enum**: Concrete client type supporting OpenAI and Anthropic
- **generate_with_parser**: Built-in retry logic for parsing failures (3 attempts)
- **provider.rs**: Type-safe Provider enum with FromStr/Display traits
- Provider-specific implementations (OpenAI, Anthropic)
- Secure secret management (keyring + environment variables)

### 3. System Layer (`system/`)
- Auto-detect OS/Shell (Windows/Linux/macOS, cmd/powershell/bash/zsh)
  - Parent process-based shell detection (primary)
  - Environment variable fallback (secondary)
- TOML-based configuration file management

## Data Flow

### Suggest Mode
```
User: "find large files"
  ↓
LLM: Generate commands
  ↓
UI: Display options (cliclack)
  ↓
User: Select command
  ↓
Clipboard: Copy to clipboard automatically
  ↓
Output: Success message
```

## Configuration

**Config file:** `~/.config/doum-cli/config.toml` (Linux/macOS) or `%APPDATA%\doum-cli\config.toml` (Windows)

```toml
[llm]
provider = "openai"
model = "gpt-4"
timeout = 30
use_thinking = false
use_web_search = true

[context]
max_lines = 100
max_size_kb = 50

[logging]
enabled = true
level = "info"
```

**Secrets:** Stored separately in OS keyring or environment variables
- Windows: Credential Manager (`openai.doum-cli`)
- macOS: Keychain
- Linux: Secret Service

# Architecture

## Overview

doum-cli is an LLM-based CLI tool that generates appropriate OS commands or provides answers based on natural language queries.

```
User Input → CLI Parser → Commands Handler → LLM Client → Response → UI
```

## Module Structure

```
src/
├── main.rs              # Entry point & routing
├── cli/                 # CLI interface (each command is a module)
│   ├── args.rs         # Command line arguments
│   ├── ask.rs          # Ask command handler
│   ├── suggest.rs      # Suggest command handler
│   ├── auto_mode.rs    # Auto mode handler
│   ├── secret.rs       # Secret command handler
│   ├── switch.rs       # Switch command handler
│   └── config.rs       # Config command handler
├── core/               # Business logic
│   ├── ask.rs          # Q&A mode logic
│   ├── suggest.rs      # Command suggestion logic
│   ├── auto_mode.rs    # Auto mode selection
│   ├── secret.rs       # Secret management service
│   ├── switch.rs       # Provider/Model switching service
│   └── config.rs       # Config manager
├── llm/                # LLM integration
│   ├── client.rs       # LLM client trait & verify_config
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
- **Each command is a separate module** with its own UI logic using `cliclack`
- **ask.rs**: Question answering with spinner feedback
- **suggest.rs**: Command suggestions with clipboard copy
- **auto_mode.rs**: Automatic mode selection
- **secret.rs**: API key configuration with verification
- **switch.rs**: Provider/Model switching with 2-step selection
- **config.rs**: Configuration operations (set/get/unset/show/reset)

### 2. Core Logic (`core/`)
- **ask.rs**: Provides answers to questions
- **suggest.rs**: Command suggestions with copy/execute options
- **auto_mode.rs**: LLM analyzes input and automatically selects ask/suggest mode

### 3. LLM Integration (`llm/`)
- Provider-specific implementations (OpenAI, Anthropic)
- Secure secret management (keyring + environment variables)
- Streaming response support
- Retry logic with exponential backoff

### 4. System Layer (`system/`)
- Auto-detect OS/Shell (Windows/Linux/macOS, cmd/powershell/bash/zsh)
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
max_retries = 3

[llm.context]
max_lines = 100
max_size_kb = 50
```

**Secrets:** Stored separately in OS keyring or environment variables
- Windows: Credential Manager (`openai/doum-cli`)
- macOS: Keychain
- Linux: Secret Service

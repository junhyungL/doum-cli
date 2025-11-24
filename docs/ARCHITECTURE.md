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
├── cli/                 # CLI interface
│   ├── args.rs         # Command line arguments
│   ├── commands.rs     # Command handlers
│   ├── ui.rs           # User interaction (dialoguer)
│   └── config/         # Interactive config TUI
├── core/               # Business logic
│   ├── ask.rs          # Q&A mode
│   ├── suggest.rs      # Command suggestion mode
│   └── auto_mode.rs    # Auto mode selection
├── llm/                # LLM integration
│   ├── client.rs       # LLM client trait
│   ├── prompt.rs       # Prompt templates
│   ├── parser.rs       # Response parsing
│   ├── openai/         # OpenAI implementation
│   └── anthropic/      # Anthropic implementation
├── system/             # System utilities
│   ├── config.rs       # Configuration management
│   ├── env.rs          # OS/Shell detection
│   └── error.rs        # Error handling
└── tools/              # Tool execution
    └── executor.rs     # Command executor
```

## Key Components

### 1. CLI Layer (`cli/`)
- **commands.rs**: Mode-specific handlers (`handle_ask_command`, `handle_suggest_command`, etc.)
- **ui.rs**: User interaction based on dialoguer
- **config/**: Configuration menu implemented with ratatui TUI

### 2. Core Logic (`core/`)
- **ask.rs**: Provides answers to questions
- **suggest.rs**: Command suggestions with copy/execute options
- **auto_mode.rs**: LLM analyzes input and automatically selects ask/suggest mode

### 3. LLM Integration (`llm/`)
- Provider-specific implementations (OpenAI, Anthropic)
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
UI: Display options (dialoguer)
  ↓
User: Select & Choose action (Copy/Execute/Cancel)
  ↓
Executor: Run command with UTF-8 encoding
  ↓
Output: Display result
```

## Configuration

Config file: `~/.config/doum-cli/config.toml` (Linux/macOS) or `%APPDATA%\doum-cli\config.toml` (Windows)

```toml
[llm]
provider = "openai"
timeout = 30
max_retries = 3

[llm.context]
max_lines = 100
max_size_kb = 50

[llm.providers.openai]
api_key = "sk-..."
model = "gpt-4"
```

# Commands Reference

## Basic Usage

```bash
doum [OPTIONS] [COMMAND]
```

## Commands

### `ask` - Ask Questions
Get answers to technical questions.

```bash
doum ask "What is Docker?"
doum ask "How to use grep?"
doum ask "Explain Rust ownership"
```

### `suggest` - Command Suggestions
Get command suggestions for specific tasks.

```bash
doum suggest "find large files"
doum suggest "compress a folder"
doum suggest "monitor system resources"
```

**Interactive Selection:**
1. Select from suggested commands
2. Choose: 📋 Copy, ▶️ Execute, or ❌ Cancel

### `config` - Configuration Management

```bash
# Launch interactive TUI
doum config

# Show current configuration
doum config --show

# Reset to default
doum config --reset
```

**Configuration Menu (TUI):**
- LLM Settings (timeout, retries, thinking mode, web search)
- Provider Settings (OpenAI, Anthropic)
- Context Settings (max lines, max size)

### Auto Mode (Default)
LLM automatically selects the appropriate mode when no command is specified.

```bash
doum "What is Kubernetes?"           # → ask mode
doum "show disk usage"                # → suggest mode
```

## Options

```bash
doum --help              # Show help
doum --version           # Show version
```

## Examples

### File Management
```bash
doum suggest "find all .log files older than 7 days"
doum suggest "count files in each subdirectory"
```

### System Monitoring
```bash
doum suggest "check CPU usage"
doum suggest "list top 10 processes by memory"
```

### Networking
```bash
doum suggest "check if port 8080 is open"
doum suggest "show network connections"
```

### Git Operations
```bash
doum suggest "show commits from last week"
doum suggest "undo last commit but keep changes"
```

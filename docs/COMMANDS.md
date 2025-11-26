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

### `secret` - API Key Management
Configure API keys securely using OS keyring.

```bash
# Configure OpenAI
doum secret openai

# Configure Anthropic
doum secret anthropic
```

**Storage Priority:**
1. OS Keyring (Windows Credential Manager, macOS Keychain, Linux Secret Service)
2. Environment Variable (`OPENAI_SECRET`, `ANTHROPIC_SECRET`)

**Note:** Due to Windows keyring limitations, environment variables are recommended:
```powershell
# The secret command will output the exact command to use
doum secret openai
# Copy the displayed PowerShell command
```

### `switch` - Provider/Model Selection
Interactive menu to switch LLM provider or model.

```bash
doum switch
```

**Options:**
- Provider: OpenAI / Anthropic
- Model: gpt-4, gpt-3.5-turbo, claude-3-5-sonnet, etc.

### `config` - Configuration Management

```bash
# Show all configuration
doum config show

# Set a value
doum config set <key> <value>

# Get a value
doum config get <key>

# Unset (remove) a value
doum config unset <key>

# Reset to defaults
doum config reset
```

**Common Keys:**
- `llm.provider` - LLM provider (openai/anthropic)
- `llm.model` - Model name
- `llm.timeout` - Request timeout in seconds
- `llm.max_retries` - Maximum retry attempts

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

# Quick Start Guide

## Installation

### 1. Install
```bash
cargo install --path .
```

### 2. Add to PATH
- Ensure `~/.cargo/bin` is in your system PATH.

## Setup

### API Key Setup

```bash
# Interactive setup (saves to OS keyring)
doum secret
```

### Switch Provider/Model

```bash
# Interactive provider/model selection
doum switch
```

## First Steps

### 1. Ask a Question
```bash
doum ask "What is the difference between grep and awk?"
```

### 2. Get Command Suggestions
```bash
doum suggest "find files larger than 100MB"
```

→ Select command → Automatically copied to clipboard!

### 3. Auto Mode
```bash
doum "how to check disk space"
```

LLM automatically selects the appropriate mode.

## Common Use Cases

### File Searching
```bash
doum suggest "find all Python files modified today"
doum suggest "search for files containing 'TODO'"
```

### System Monitoring
```bash
doum suggest "check memory usage"
doum suggest "list running docker containers"
```

### File Operations
```bash
doum suggest "count lines in all .txt files"
doum suggest "remove duplicates from file"
```

### Git Operations
```bash
doum suggest "show files changed in last commit"
doum suggest "create new branch from main"
```

## Configuration Tips

### View Current Configuration
```bash
doum config show
```

### Set Configuration Values
```bash
doum config set llm.timeout 60
doum config set llm.max_retries 5
```

### Get Configuration Value
```bash
doum config get llm.provider
doum config get llm.model
```

### Unset (Remove) Configuration
```bash
doum config unset llm.timeout  # Revert to default
```

## Troubleshooting
**Linux users:**
- **GLIBC 2.38+ is required** - This means Ubuntu 24.04 or later, Fedora 39+, etc.
- Older distributions (Ubuntu 22.04, Debian 11, CentOS 9, etc.) are **not supported**
- If you encounter GLIBC version errors, you need to upgrade your system or use a newer distribution

**macOS/Windows users:**
- Ensure your OS keyring service is running (Keychain for macOS, Credential Manager for Windows)
- Check for any error messages during `doum secret` execution

### Check Logs
Log file location:
- Linux: `~/.local/doum-cli/logs/`
- macOS: `~/Library/Application Support/doum-cli/logs/`
- Windows: `%APPDATA%\doum-cli\logs\`

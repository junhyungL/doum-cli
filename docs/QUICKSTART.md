# Quick Start Guide

## Installation

### 1. Build
```bash
git clone https://github.com/yourusername/doum-cli.git
cd doum-cli
cargo build --release
```

### 2. Install
```bash
cargo install --path .
```

### 3. Add to PATH
```bash
# Linux/macOS
cp target/release/doum-cli ~/.local/bin/doum

# Windows
copy target\release\doum-cli.exe C:\Users\YourName\.cargo\bin\doum.exe
```

## Setup

### API Key Setup

**OpenAI:**
```bash
# Interactive setup (saves to OS keyring)
doum secret openai

# Or use environment variable (Windows)
$env:OPENAI_SECRET='{"api_key":"sk-..."}'

# Or use environment variable (Linux/macOS)
export OPENAI_SECRET='{"api_key":"sk-..."}'
```

**Anthropic (Claude):**
```bash
# Interactive setup
doum secret anthropic

# Switch to Anthropic provider
doum switch
# Select: Provider → Anthropic

# Or use environment variable
export ANTHROPIC_SECRET='{"api_key":"sk-ant-..."}'
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

→ Select option → Choose Copy/Execute

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

### Change LLM Provider or Model
```bash
doum switch
# Interactive menu for provider/model selection
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

### API Key Issues
```bash
# Reconfigure API key
doum secret openai

# Or use environment variable as fallback
$env:OPENAI_SECRET='{"api_key":"sk-..."}'
```

### View Current Configuration
```bash
doum config show
```

### Reset Configuration to Defaults
```bash
doum config reset
```

### Keyring Not Working
If secrets don't persist, use environment variables:
```powershell
# PowerShell (current session)
$env:OPENAI_SECRET='{"api_key":"sk-..."}'

# PowerShell (permanent - add to profile)
echo "$env:OPENAI_SECRET='{\"api_key\":\"sk-...\'}}"` >> $PROFILE
```

### Check Logs
Log file location:
- Linux/macOS: `~/.local/share/doum-cli/logs/`
- Windows: `%APPDATA%\doum-cli\logs\`

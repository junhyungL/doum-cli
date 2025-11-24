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
doum config
# LLM Settings → Providers → OpenAI → Edit API Key
```

**Anthropic (Claude):**
```bash
doum config
# LLM Settings → Providers → Anthropic → Edit API Key
# LLM Settings → Provider Selection → Anthropic
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

### Response Speed Adjustment
```bash
doum config
# LLM Settings → Timeout → 조정 (default: 30 seconds)
```

### Context Size Setting
```bash
doum config
# LLM Settings → Context Settings → Max Lines/Size
```

### Provider Selection
```bash
doum config
# LLM Settings → Provider Selection → OpenAI/Anthropic
```

## Troubleshooting

### API Key Update
```bash
doum config --show  # 현재 설정 확인
doum config         # API 키 재설정
```

### Reset Configuration
```bash
doum config --reset
```

### Check Logs
Log file location:
- Linux/macOS: `~/.local/share/doum-cli/logs/`
- Windows: `%APPDATA%\doum-cli\logs\`

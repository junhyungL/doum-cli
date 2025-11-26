# doum-cli

🤖 **AI-Powered Terminal Assistant** - Natural language interface for OS commands

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- 💬 **Ask Mode**: Get answers to technical questions
- 🔍 **Suggest Mode**: Command suggestions with copy/execute options
- 🎯 **Auto Mode**: LLM automatically selects the appropriate mode
- 🔐 **Secret Management**: Secure API key storage with OS keyring
- ⚙️ **Config Management**: Simple config commands (set/get/unset/show/reset)
- 🌍 **Multi-Provider**: Support for OpenAI (GPT) and Anthropic (Claude)

## Quick Start

### Installation

#### Using Installation Script (Recommended)

**Linux / macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/junhyungL/doum-cli/main/scripts/install.sh | sh
```

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/junhyungL/doum-cli/main/scripts/install.ps1 | iex
```

#### From GitHub Releases

Download the latest binary for your platform from [Releases](https://github.com/junhyungL/doum-cli/releases):

- **Linux (x86_64)**: `doum-linux-x86_64.tar.gz`
- **macOS (Intel)**: `doum-macos-x86_64.tar.gz`
- **macOS (Apple Silicon)**: `doum-macos-aarch64.tar.gz`
- **Windows (x86_64)**: `doum-windows-x86_64.zip`

Extract and add to your PATH.

#### Using Cargo

```bash
cargo install doum-cli
```

### Setup API Key

```bash
# Configure OpenAI secret (interactive)
doum secret openai

# Or set via environment variable (if keyring doesn't work)
# PowerShell:
$env:OPENAI_SECRET='{"api_key":"sk-..."}'

# Bash/Zsh:
export OPENAI_SECRET='{"api_key":"sk-..."}'
```

### Usage Examples

```bash
# Ask questions
doum ask "What is Docker?"

# Get command suggestions
doum suggest "find large files"
→ Select option → Choose Copy/Execute

# Auto mode
doum "check disk usage"
```

## Commands

| Command | Description |
|---------|-------------|
| `doum ask <question>` | Ask questions and get answers |
| `doum suggest <task>` | Get command suggestions and execute |
| `doum secret <provider>` | Configure API keys (openai/anthropic) |
| `doum switch` | Switch LLM provider or model |
| `doum config <subcommand>` | Manage configuration (show/set/get/unset/reset) |
| `doum <input>` | Auto mode (LLM selects mode) |

## Documentation

- [Architecture](docs/ARCHITECTURE.md) - Architecture and module structure
- [Commands](docs/COMMANDS.md) - Detailed command reference
- [Quick Start](docs/QUICKSTART.md) - Installation and getting started
- [TODO](docs/TODO.md) - Development roadmap

## Tech Stack

- **Language**: Rust 2024
- **Terminal UI**: dialoguer
- **Secret Storage**: keyring (OS-level credential store)
- **LLM**: OpenAI GPT, Anthropic Claude

## Development

```bash
# Build
cargo build --release

# Run tests
cargo test

# Run
./target/release/doum-cli
```

## License

MIT License - see [LICENSE](LICENSE) for details

## Acknowledgments

- Powered by [OpenAI](https://openai.com/) & [Anthropic](https://www.anthropic.com/)
- Built with [Rust](https://www.rust-lang.org/) 🦀

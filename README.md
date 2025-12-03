# doum-cli

🤖 **AI-Powered Terminal Assistant** 
Terminal command helper powered by Large Language Models (LLMs) like OpenAI GPT and Anthropic Claude.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- 💬 **Ask Mode**: Get answers to technical questions
- 🔍 **Suggest Mode**: Command suggestions with instant clipboard copy
- 🎯 **Auto Mode**: LLM automatically selects the appropriate mode
- 🔐 **Secret Management**: Secure API key storage with automatic verification
- ⚙️ **Config Management**: Simple config commands (set/get/unset/show/reset)
- 🌍 **Multi-Provider**: Support for OpenAI (GPT) and Anthropic (Claude)
- ✨ **Modern UI**: Clean, interactive CLI powered by cliclack

## Quick Start

### Installation

#### Using Installation Script (Recommended)

- **Linux / macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/junhyungL/doum-cli/main/scripts/install.sh | sh
```

- **Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/junhyungL/doum-cli/main/scripts/install.ps1 | iex
```

#### Using Cargo

if you have Rust and Cargo installed, you can install via Cargo:
```bash
cargo install doum-cli
```

#### From GitHub Releases

- Download the latest binary for your platform from [Releases](https://github.com/junhyungL/doum-cli/releases)
- Choose the appropriate version for your OS.
- Extract and add to your PATH.

### Setup Secret

Before using doum-cli, you need to configure your LLM API keys.

```bash
# Configure OpenAI secret
doum secret openai
```

#### Storage Location by Platform

- **Linux/FreeBSD/OpenBSD**: DBus Secret Service (GNOME Keyring, KWallet)
- **macOS/iOS**: Local Keychain
- **Windows**: Windows Credential Manager

#### Troubleshooting

**Linux users**: If you encounter issues with Secret Service:
- Ensure you have a desktop environment with keyring support (GNOME Keyring or KWallet)
- The Secret Service uses synchronous IPC calls which may take 10-100ms per operation
- For async runtime users: Use a separate thread for keyring operations to avoid deadlocks

**macOS/iOS users**: Service and user names cannot be empty (treated as wildcards)

**Windows users**: Multi-threaded access may not be serialized - access credentials from one thread at a time

### Switch Provider/Model

```bash
# Interactive provider/model selection
doum switch
```

### Usage Examples

```bash
# Ask questions
doum ask "What is Docker?"

# Get command suggestions (auto-copied to clipboard)
doum suggest "find large files"

# Auto mode (LLM decides ask/suggest)
doum "check disk usage"

# Configure API keys (with verification)
doum secret

# Switch provider/model
doum switch
```

## Commands

| Command | Description |
|---------|-------------|
| `doum secret <provider>` | Configure API keys (openai/anthropic) |
| `doum switch` | Switch LLM provider or model |
| `doum config <subcommand>` | Manage configuration (show/set/get/unset/reset) |
| `doum ask <question>` | Ask questions and get answers |
| `doum suggest <task>` | Get command suggestions and execute |
| `doum <input>` | Auto mode (LLM selects mode) |

## Documentation

- [Quick Start](docs/QUICKSTART.md) - Installation and getting started
- [Architecture](docs/ARCHITECTURE.md) - Architecture and module structure
- [Commands](docs/COMMANDS.md) - Detailed command reference

## License

MIT License - see [LICENSE](LICENSE) for details

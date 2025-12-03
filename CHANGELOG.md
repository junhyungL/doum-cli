# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-12-04

### Changed
- **UI Library Migration**: Switched from `dialoguer` to `cliclack` for better UX
  - Cleaner, more modern interactive prompts
  - Better visual feedback with intro/outro messages
  - Improved spinner animations
- **Module Architecture**: Reorganized CLI command handlers
  - Each command is now a separate module (ask.rs, suggest.rs, etc.)
  - Removed central `commands.rs` router
  - Better code organization and maintainability
- **Suggest Command**: Simplified workflow
  - Removed execute option
  - Commands are automatically copied to clipboard after selection
  - Uses `arboard` for reliable cross-platform clipboard operations
- **Switch Command**: Improved selection flow
  - Changed to 2-step process: Provider → Model
  - More intuitive than previous combined selection
  - Removed unnecessary verification step
- **Secret Command**: Enhanced user experience
  - Optional fields (OpenAI org/project) can be skipped with Enter
  - API key verification happens automatically after input
  - Better error messages and feedback

### Removed
- Removed dependencies: `dialoguer`, `console`, `indicatif`
- Removed unused UI utilities and helper functions
- Removed execute option from suggest command

### Added
- Added `cliclack` for modern CLI interactions
- Added `arboard` for cross-platform clipboard support (replacing cli-clipboard)
- Added `verify_config()` function for testing API configurations

## [0.2.0] - 2025-11-27

### Added
- **Secret Management**: Secure API key storage using OS-level keyring
  - `doum secret openai` - Configure OpenAI API keys
  - `doum secret anthropic` - Configure Anthropic API keys
  - Support for Windows Credential Manager, macOS Keychain, Linux Secret Service
  - Environment variable fallback (`OPENAI_SECRET`, `ANTHROPIC_SECRET`)
- **Provider Switching**: Interactive menu to switch LLM provider and model
  - `doum switch` - Quick provider/model selection
- **Config Subcommands**: Simplified configuration management
  - `doum config show` - Display all settings
  - `doum config set <key> <value>` - Set configuration value
  - `doum config get <key>` - Get configuration value
  - `doum config unset <key>` - Remove configuration value
  - `doum config reset` - Reset to defaults

### Changed
- **Configuration Structure**: Simplified config format
  - Removed nested `providers` section
  - Moved `model` to top-level `llm.model`
  - API keys stored separately in OS keyring instead of config file
- **Default Config**: Embedded default configuration in binary
  - Removed `rust-embed` dependency for config loading
  - Faster initial setup with sensible defaults

### Removed
- **Interactive TUI**: Removed ratatui-based configuration menu
  - Replaced with simple command-line subcommands
  - Removed dependencies: `ratatui`, `crossterm`
  - Simpler, more maintainable CLI interface

### Security
- API keys no longer stored in plain text config files
- Secure storage using OS-native credential managers

## [0.1.0] - 2025-11-25
- Initial release

---

**Legend**
- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` in case of vulnerabilities
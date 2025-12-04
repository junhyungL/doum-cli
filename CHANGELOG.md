# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Type Safety Enhancement**: Updated `LLMConfig` to use `Provider` enum instead of `String`
  - Direct serialization/deserialization with TOML configuration
  - Removed unnecessary string parsing throughout the codebase
  - Improved type safety across all configuration operations
- **Code Refactoring**: Simplified provider handling
  - Updated `verify_config()` to accept `Provider` type directly
  - Removed redundant `.as_str()` and `.parse()` calls
  - Cleaner API with better compile-time guarantees

## [0.3.1] - 2025-12-05

### Added
- **Provider Enum**: Introduced `Provider` enum in `llm/provider.rs` for better type safety
  - Centralized provider management (OpenAI, Anthropic)
  - Implemented `FromStr`, `Display`, and serialization traits
  - Added helper methods: `as_str()`, `all()`, `all_names()`
  - Comprehensive test coverage

### Changed
- **Improved Shell Detection**: Enhanced shell type detection using parent process analysis
  - Added `sysinfo` dependency for cross-platform process inspection
  - More accurate detection compared to environment variable fallback
  - 2-tier detection: parent process (primary) → environment variables (fallback)
- **Code Refactoring**: Replaced hardcoded provider strings with `Provider` enum
  - Updated all modules: `llm/`, `core/`, `system/`, `cli/`
  - Better compile-time safety and IDE support
  - Easier maintenance and extension for new providers
- **Prompt Builder**: Unified prompt concatenation logic
  - Added `concat_prompts()` helper method
  - Consistent formatting across all prompt types (ask, suggest, mode_select)

### Fixed
- Removed unused imports across multiple modules
- Fixed prompt builder return values

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
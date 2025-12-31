# Agent Development Guide

This guide provides coding standards and development practices for AI agents working on the local_shelf project.

<!-- OPENSPEC:START -->
## OpenSpec Integration

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines
<!-- OPENSPEC:END -->

## Build & Test Commands

### Basic Commands
```bash
# Build the project
cargo build

# Run the application
cargo run

# Run all tests
cargo test

# Run tests quietly (minimal output)
cargo test --quiet

# Run a specific test
cargo test test_config_validation_empty_path

# Run tests in a specific file/module
cargo test config::tests::
cargo test file_discovery::tests::
cargo test integration_test

# List all available tests
cargo test -- --list

# Run with environment variable override
KNOWLEDGE_BASE="/custom/path" cargo run
```

### Code Quality
```bash
# Format code (REQUIRED before commits)
cargo fmt

# Check linting (REQUIRED - must pass)
cargo clippy

# Check without building
cargo check

# Run with specific features
cargo test --features "feature_name"
```

### Development Workflow
```bash
# Create new jujutsu changeset before starting work
jj new

# Describe your change after completion
jj desc -m "Brief title\n\nDetailed description"
```

## Code Style Guidelines

### Import Organization
```rust
// 1. External crates (alphabetical)
use serde::{Deserialize, Serialize};
use tempfile::tempdir;

// 2. Standard library (alphabetical)
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// 3. Local modules (alphabetical)
use crate::config::{Config, ConfigError};
```

### Naming Conventions
- **Functions**: `snake_case` - `load_config()`, `validate_path()`
- **Structs/Enums**: `PascalCase` - `Config`, `ConfigError`
- **Constants**: `SCREAMING_SNAKE_CASE` - `DEFAULT_CONFIG_PATH`
- **Variables**: `snake_case` - `knowledge_base_path`, `config_dir`
- **Modules**: `snake_case` - `config.rs`, `file_manager.rs`

### Type Usage
```rust
// Prefer explicit types for public APIs
pub fn load_config() -> Result<Config, ConfigError> { ... }

// Use descriptive error types, not generic errors
pub enum ConfigError {
    IoError(std::io::Error),
    YamlError(serde_yaml::Error), 
    ValidationError(String),
}

// Implement proper error traits
impl std::fmt::Display for ConfigError { ... }
impl std::error::Error for ConfigError {}
impl From<std::io::Error> for ConfigError { ... }
```

### Error Handling Patterns
```rust
// Use ? operator for propagating errors
let config = Config::load()?;

// Provide descriptive error messages
return Err(ConfigError::ValidationError(
    format!("Parent directory does not exist: {}", parent.display())
));

// Use Result<T, E> for fallible operations
pub fn initialize() -> Result<(), ConfigError> { ... }

// Chain error conversions with From trait
impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        ConfigError::YamlError(error)
    }
}
```

### Documentation Standards
```rust
/// Get the configuration directory path
/// 
/// Returns the platform-specific config directory with "local_shelf" appended.
/// On macOS: ~/Library/Application Support/local_shelf
/// On Linux: ~/.config/local_shelf
pub fn config_dir() -> Result<PathBuf, ConfigError> { ... }

/// Expand tilde (~) in path
/// 
/// # Examples
/// ```
/// assert_eq!(Config::expand_path("~/Documents"), "/home/user/Documents");
/// ```
pub fn expand_path(path: &str) -> String { ... }
```

## Testing Guidelines

### Test Organization
- **Unit tests**: In the same file with `#[cfg(test)]` module
- **Integration tests**: In `tests/` directory
- **Test naming**: `test_function_scenario` - `test_config_validation_empty_path`

### Test Patterns
```rust
#[test]
fn test_config_validation_empty_path() {
    let config = Config {
        knowledge_base_path: "".to_string(),
    };
    assert!(config.validate().is_err());
}

// Use tempfile for filesystem tests
#[test]
fn test_file_operations() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.yaml");
    // ... test implementation
}

// Test environment variable behavior
#[test] 
fn test_environment_override() {
    unsafe {
        env::set_var("KNOWLEDGE_BASE", "/test/path");
    }
    // ... test implementation
    unsafe {
        env::remove_var("KNOWLEDGE_BASE");
    }
}
```

### Test Coverage Requirements
- All public functions MUST have tests
- Error conditions MUST be tested
- Configuration scenarios MUST be covered:
  - Default values
  - File loading
  - Environment overrides
  - Path expansion
  - Validation failures

## Project-Specific Guidelines

### Configuration Management
- Always validate configuration after loading
- Support environment variable overrides
- Create default config files on first run
- Use hierarchical loading: defaults < file < env vars

### File Operations
- Use `dirs` crate for platform-specific paths
- Handle missing directories gracefully
- Never overwrite files without user consent
- Expand `~/` in user-provided paths

### Development Principles
- **TDD Approach**: Write tests before implementation
- **Error Safety**: All operations should handle failure cases
- **Path Safety**: Validate and expand all file paths
- **Configuration First**: Load config before any file operations

### Version Control (Jujutsu)
```bash
# Before starting new feature
jj new

# Describe completed work
jj desc -m "Add feature name

Brief description of what was implemented.
Include any breaking changes or important details."
```

## Required Checks Before Commit
1. `cargo test` - All tests pass
2. `cargo fmt` - Code is formatted
3. `cargo clippy` - No linting warnings
4. Configuration scenarios tested
5. Error handling implemented
6. Documentation updated if needed

This project emphasizes robust configuration management, comprehensive testing, and clear error handling. Always prioritize user safety and data integrity in file operations.
# Change: Add Configuration Management System

## Why
The project requires storing the Knowledge Base location parameter in a config file (`~/.config/local-shelf/config.yaml`), but this functionality doesn't exist yet. Without proper configuration management, users cannot customize their Knowledge Base location or other settings.

## What Changes
- Add YAML-based configuration system with default and user-specific config files
- Implement configuration loading with environment variable overrides
- Add validation for required configuration parameters
- Create configuration initialization on first run
- **BREAKING**: Hard-coded paths will be replaced with configurable ones

## Impact
- Affected specs: [new capability: configuration]
- Affected code: src/main.rs (configuration loading), new config module
- Users will need to set up initial configuration or use defaults
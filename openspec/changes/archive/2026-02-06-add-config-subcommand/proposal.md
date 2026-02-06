# Change: Add Config Subcommand for Knowledge Base Path Management

## Why
Users currently need to manually edit the YAML configuration file or use environment variables to set the knowledge base path. This is inconvenient for users who want to change their knowledge base location programmatically or through a simple CLI command. A dedicated `config` subcommand would provide a user-friendly interface for managing the knowledge base path configuration.

## What Changes
- Add a new `config` subcommand to the CLI interface
- Implement `set` sub-subcommand to update the knowledge base path: `local_shelf config set knowledge_base_path /new/path`
- Validate the provided path immediately when setting (ensure it exists or can be created)
- Save configuration changes to the YAML config file

## Impact
- Affected specs: cli, configuration
- Affected code: src/main.rs (CLI structure), src/config.rs (configuration management)
- Improves user experience by providing a simple CLI interface for configuration management
- Maintains backward compatibility with existing configuration methods (file editing, environment variables)
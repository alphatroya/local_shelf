# Change: Add Config Subcommand

## Why
Users need visibility into their configuration setup to understand where the config file is located and what the expected format looks like. Currently, there's no easy way to discover the config directory path or see a configuration example without manually navigating the filesystem or reading documentation.

## What Changes
- Add new `config` subcommand to the CLI interface
- Print current configuration directory path
- Display configuration file example with comments
- Provide clear output that helps users understand their setup

## Impact
- Affected specs: `cli` - adding new subcommand to existing CLI structure
- Affected code: `src/main.rs` - new subcommand enum variant and handler function
- User experience: Improved discoverability of configuration options
- Backwards compatible: No breaking changes to existing functionality
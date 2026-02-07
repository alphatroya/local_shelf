## ADDED Requirements

### Requirement: Config Subcommand
The system SHALL provide a `config` subcommand that displays configuration information to help users understand their setup.

#### Scenario: Config subcommand displays directory location
- **WHEN** user runs `local_shelf config`
- **THEN** display the current configuration directory path (e.g., "~/.config/local_shelf" or "~/Library/Application Support/local_shelf")

#### Scenario: Config subcommand shows configuration example
- **WHEN** user runs `local_shelf config`
- **THEN** display a properly formatted YAML configuration example with comments explaining each option

#### Scenario: Config subcommand help display
- **WHEN** user runs `local_shelf config --help`
- **THEN** display help text explaining the config subcommand functionality and its purpose

#### Scenario: Config subcommand with non-existent config directory
- **WHEN** user runs `local_shelf config` and no configuration directory exists yet
- **THEN** display the directory path where configuration would be created and show the example configuration
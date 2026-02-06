## ADDED Requirements
### Requirement: Config Subcommand
The system SHALL provide a `config` subcommand for managing application configuration through the command-line interface.

#### Scenario: Config help display
- **WHEN** user runs `local_shelf config --help`
- **THEN** display help text explaining configuration management options and available sub-subcommands

#### Scenario: Config without sub-subcommand
- **WHEN** user runs `local_shelf config` with no sub-subcommands
- **THEN** display error message indicating a sub-subcommand is required and show available options

#### Scenario: Invalid config sub-subcommand
- **WHEN** user runs `local_shelf config invalid-command`
- **THEN** display error message indicating invalid sub-subcommand and suggest valid options

### Requirement: Config Set Knowledge Base Path
The system SHALL provide a `set` sub-subcommand under `config` to update the knowledge base path configuration.

#### Scenario: Set knowledge base path successfully
- **WHEN** user runs `local_shelf config set knowledge_base_path /valid/path`
- **THEN** validate the path exists or can be created, save the new path to config.yaml, and display confirmation message

#### Scenario: Set knowledge base path with tilde expansion
- **WHEN** user runs `local_shelf config set knowledge_base_path ~/Documents/KB`
- **THEN** expand the tilde to the user's home directory, validate the path, and save the expanded path

#### Scenario: Set knowledge base path to invalid location
- **WHEN** user runs `local_shelf config set knowledge_base_path /nonexistent/readonly/path`
- **THEN** display error message indicating the path cannot be created or accessed and exit with error code

#### Scenario: Set knowledge base path with empty value
- **WHEN** user runs `local_shelf config set knowledge_base_path ""`
- **THEN** display error message indicating path cannot be empty and exit with error code

#### Scenario: Set help for config set
- **WHEN** user runs `local_shelf config set --help`
- **THEN** display help text explaining how to set configuration values and available parameters
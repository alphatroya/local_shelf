# cli Specification

## Purpose
TBD - created by archiving change split-app-stow-convert. Update Purpose after archive.
## Requirements
### Requirement: Command Line Interface Structure
The system SHALL provide a command-line interface with subcommands for different operations.

#### Scenario: Display help when no subcommand provided
- **WHEN** user runs `local_shelf` with no arguments
- **THEN** display help text showing available subcommands (stow, convert) and require explicit subcommand selection

#### Scenario: Display version information
- **WHEN** user runs `local_shelf --version` or `local_shelf -V`
- **THEN** display the application version number "0.2.0"

#### Scenario: Invalid subcommand handling
- **WHEN** user runs `local_shelf invalid-command`
- **THEN** display error message and suggest valid subcommands

### Requirement: Stow Subcommand
The system SHALL provide a `stow` subcommand that moves markdown files from a specified directory to the Knowledge Base pages directory.

#### Scenario: Stow execution with default directory
- **WHEN** user runs `local_shelf stow` with no directory parameter
- **THEN** scan the current directory for markdown files and execute the file discovery, move, and journal entry logic

#### Scenario: Stow execution with specified directory
- **WHEN** user runs `local_shelf stow /path/to/directory` 
- **THEN** scan the specified directory for markdown files and execute the file discovery, move, and journal entry logic

#### Scenario: Stow with Downloads directory (backward compatibility)
- **WHEN** user runs `local_shelf stow ~/Downloads`
- **THEN** scan the Downloads directory for markdown files (preserving previous default behavior when explicitly specified)

#### Scenario: Stow help display
- **WHEN** user runs `local_shelf stow --help`
- **THEN** display help text explaining the stow functionality and directory parameter usage

#### Scenario: Stow with invalid directory
- **WHEN** user runs `local_shelf stow /nonexistent/path`
- **THEN** display error message indicating directory does not exist and exit with error code

### Requirement: Convert Subcommand  
The system SHALL provide a `convert` subcommand for markdown to EPUB conversion that accepts an optional directory parameter.

#### Scenario: Convert with default directory
- **WHEN** user runs `local_shelf convert`
- **THEN** convert all markdown files in the current directory to EPUB format

#### Scenario: Convert with specified directory
- **WHEN** user runs `local_shelf convert /path/to/directory`
- **THEN** convert all markdown files in the specified directory to EPUB format

#### Scenario: Convert help display
- **WHEN** user runs `local_shelf convert --help`
- **THEN** display help text explaining conversion options, directory parameter, and pandoc requirements

#### Scenario: Missing pandoc dependency
- **WHEN** user runs `local_shelf convert` and pandoc is not installed
- **THEN** display error message with pandoc installation instructions and exit with error code

#### Scenario: Invalid directory for conversion
- **WHEN** user runs `local_shelf convert /nonexistent/path`
- **THEN** display error message indicating directory does not exist and exit with error code

### Requirement: Unified Error Handling
The system SHALL provide consistent error handling and exit codes across all subcommands.

#### Scenario: Configuration error in subcommand
- **WHEN** any subcommand encounters a configuration error
- **THEN** display descriptive error message and exit with appropriate error code

#### Scenario: File operation error in subcommand  
- **WHEN** any subcommand encounters a file operation error
- **THEN** display descriptive error message and exit with appropriate error code

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


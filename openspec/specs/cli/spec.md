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
- **THEN** display the application version number "0.3.0"

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


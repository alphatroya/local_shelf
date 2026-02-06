## ADDED Requirements

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
The system SHALL provide a `stow` subcommand that preserves all existing file organization functionality.

#### Scenario: Stow execution with current behavior
- **WHEN** user runs `local_shelf stow`
- **THEN** execute the existing file discovery, move, and journal entry logic exactly as the current implementation

#### Scenario: Stow help display
- **WHEN** user runs `local_shelf stow --help`
- **THEN** display help text explaining the stow functionality

### Requirement: Convert Subcommand
The system SHALL provide a `convert` subcommand for markdown to EPUB conversion.

#### Scenario: Convert with default directory
- **WHEN** user runs `local_shelf convert`
- **THEN** convert all markdown files in the current directory to EPUB format

#### Scenario: Convert with specified directory
- **WHEN** user runs `local_shelf convert /path/to/directory`
- **THEN** convert all markdown files in the specified directory to EPUB format

#### Scenario: Convert help display
- **WHEN** user runs `local_shelf convert --help`
- **THEN** display help text explaining conversion options and requirements

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
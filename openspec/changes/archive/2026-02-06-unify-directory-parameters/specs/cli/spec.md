## MODIFIED Requirements

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
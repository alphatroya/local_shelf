## ADDED Requirements

### Requirement: Downloads Directory Discovery
The system SHALL scan the ~/Downloads directory to identify its contents for processing.

#### Scenario: Downloads directory exists and is accessible
- **WHEN** the system attempts to scan ~/Downloads directory
- **THEN** it SHALL successfully read the directory contents

#### Scenario: Downloads directory does not exist
- **WHEN** the system attempts to scan ~/Downloads directory that doesn't exist
- **THEN** it SHALL return an empty result and log the condition

#### Scenario: Downloads directory access denied
- **WHEN** the system lacks permissions to read ~/Downloads directory
- **THEN** it SHALL return an appropriate error indicating permission issues

### Requirement: Markdown File Filtering
The system SHALL identify and filter files with .md extension from the Downloads directory contents.

#### Scenario: Markdown files present
- **WHEN** scanning a directory containing .md files and other file types
- **THEN** it SHALL return only files with .md extension

#### Scenario: No markdown files present
- **WHEN** scanning a directory with no .md files
- **THEN** it SHALL return an empty list

#### Scenario: Mixed case extensions
- **WHEN** scanning a directory with .md, .MD, and .Md files
- **THEN** it SHALL recognize all markdown files regardless of case

### Requirement: Path Expansion
The system SHALL expand tilde (~) notation to the user's home directory path.

#### Scenario: Tilde path expansion
- **WHEN** given a path starting with ~/Downloads
- **THEN** it SHALL expand ~ to the current user's home directory path

#### Scenario: Non-tilde paths
- **WHEN** given an absolute path not starting with ~
- **THEN** it SHALL use the path as provided without modification

### Requirement: File Discovery Interface
The system SHALL provide a clean interface for discovering markdown files in Downloads directory.

#### Scenario: Successful file discovery
- **WHEN** calling the file discovery function
- **THEN** it SHALL return a list of full paths to discovered markdown files

#### Scenario: Error handling in discovery
- **WHEN** file system errors occur during discovery
- **THEN** it SHALL propagate errors with descriptive messages
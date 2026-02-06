## ADDED Requirements

### Requirement: Markdown to EPUB Conversion
The system SHALL convert markdown files to EPUB format using pandoc.

#### Scenario: Single file conversion success
- **WHEN** a markdown file exists in the target directory
- **THEN** create corresponding EPUB file in the same directory with same basename

#### Scenario: Multiple file conversion
- **WHEN** multiple markdown files exist in the target directory  
- **THEN** convert each file individually and report conversion statistics

#### Scenario: File without markdown extension skipped
- **WHEN** non-markdown files exist in the target directory
- **THEN** skip those files and only process .md files

#### Scenario: Conversion failure handling
- **WHEN** pandoc fails to convert a specific file
- **THEN** report the failure, continue with remaining files, and include failed count in final statistics

### Requirement: Pandoc Dependency Management
The system SHALL validate pandoc availability before attempting conversions.

#### Scenario: Pandoc availability check
- **WHEN** convert command starts execution
- **THEN** verify pandoc is installed and accessible in PATH

#### Scenario: Pandoc missing error
- **WHEN** pandoc is not found in PATH
- **THEN** display installation instructions with link to https://pandoc.org/installing.html and exit with error

### Requirement: Conversion Progress Reporting
The system SHALL provide clear feedback during the conversion process.

#### Scenario: Conversion start notification
- **WHEN** conversion begins
- **THEN** display target directory and begin processing notification

#### Scenario: Individual file conversion reporting
- **WHEN** each file is processed
- **THEN** display source filename, target filename, and success/failure status

#### Scenario: Final conversion statistics
- **WHEN** all files have been processed
- **THEN** display total files converted and total files that failed conversion

### Requirement: Directory Processing
The system SHALL process all markdown files in a specified directory.

#### Scenario: Target directory validation
- **WHEN** convert command receives directory argument
- **THEN** verify directory exists and is accessible

#### Scenario: Empty directory handling
- **WHEN** target directory contains no markdown files
- **THEN** display message indicating no markdown files found and exit successfully

#### Scenario: Single-level directory processing
- **WHEN** processing a directory
- **THEN** process only files directly in the target directory (non-recursive by default)
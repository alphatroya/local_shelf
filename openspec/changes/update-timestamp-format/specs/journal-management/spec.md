# journal-management Specification Delta

## MODIFIED Requirements

### Requirement: Journal Entry Creation
The system SHALL append timestamped entries linking to moved files using the specified format.

#### Scenario: Single file journal entry
- **WHEN** creating a journal entry for a moved file
- **THEN** the entry SHALL follow the format `- **HH:mm** [[Name of the file]]`
- **AND** use the current time for the timestamp in HH:mm format with colon separator
- **AND** use the actual filename without extension for the link

#### Scenario: Multiple file journal entries
- **WHEN** creating journal entries for multiple moved files
- **THEN** each file SHALL have a separate entry line
- **AND** all entries SHALL use the same timestamp when processed together in HH:mm format
- **AND** entries SHALL be appended in the order files were processed

#### Scenario: Timestamp format consistency
- **WHEN** generating timestamps for journal entries
- **THEN** the timestamp SHALL use 24-hour format with colon separator (HH:mm)
- **AND** maintain zero-padding for single-digit hours and minutes
- **EXAMPLES**: 09:45, 14:30, 23:15, 00:05
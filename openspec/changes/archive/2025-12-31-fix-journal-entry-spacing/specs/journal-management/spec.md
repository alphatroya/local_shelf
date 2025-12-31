# journal-management Specification Delta

## MODIFIED Requirements

### Requirement: Journal Entry Creation
The system SHALL append timestamped entries linking to moved files using the specified format.

#### Scenario: Multiple file journal entries
- **WHEN** creating journal entries for multiple moved files
- **THEN** each file SHALL have a separate entry line
- **AND** all entries SHALL use the same timestamp when processed together
- **AND** entries SHALL be appended in the order files were processed
- **AND** entries SHALL appear consecutively without blank lines between them

#### Scenario: Journal entry batch formatting
- **WHEN** appending multiple entries in a single batch operation
- **THEN** entries SHALL be written consecutively with no blank lines separating them
- **AND** maintain proper separation from existing content in the journal file
- **AND** the final entry SHALL end with a newline character
- **EXAMPLES**: 
  ```
  - **16:36** [[First Article]]
  - **16:36** [[Second Article]]  
  - **16:36** [[Third Article]]
  ```

#### Scenario: Journal entry spacing consistency
- **WHEN** entries are added to an existing journal file
- **THEN** new entries SHALL be separated from existing content by a single newline
- **AND** entries within the same batch SHALL have no blank lines between them
- **AND** preserve all existing journal content without modification
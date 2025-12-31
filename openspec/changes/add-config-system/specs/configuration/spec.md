## ADDED Requirements

### Requirement: Configuration File Management
The system SHALL load configuration from YAML files with a hierarchical priority system.

#### Scenario: Default configuration loading
- **WHEN** no user configuration exists
- **THEN** system uses built-in defaults for all parameters

#### Scenario: User configuration override
- **WHEN** user has ~/.config/local-shelf/config.yaml
- **THEN** user values override defaults while preserving unspecified defaults

#### Scenario: Environment variable override
- **WHEN** KNOWLEDGE_BASE environment variable is set
- **THEN** environment value takes precedence over config file values

### Requirement: Configuration Validation
The system SHALL validate all configuration parameters before use.

#### Scenario: Required path validation
- **WHEN** knowledge_base_path is specified in config
- **THEN** system SHALL verify the directory exists or can be created

#### Scenario: Invalid configuration handling
- **WHEN** configuration file contains invalid YAML or missing required fields
- **THEN** system SHALL report clear error messages and exit gracefully

### Requirement: Configuration Initialization
The system SHALL create default configuration on first run.

#### Scenario: First-time setup
- **WHEN** no configuration directory exists
- **THEN** system SHALL create ~/.config/local-shelf/ and default config.yaml

#### Scenario: Configuration template
- **WHEN** generating default configuration
- **THEN** config file SHALL include commented examples and documentation
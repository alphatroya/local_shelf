# configuration Specification Delta

## MODIFIED Requirements

### Requirement: Configuration File Management
The system SHALL load configuration from YAML files with a hierarchical priority system.

#### Scenario: Default configuration loading
- **WHEN** no user configuration exists
- **THEN** system uses built-in defaults for all parameters

#### Scenario: User configuration override
- **WHEN** user has ~/.config/local_shelf/config.yaml
- **THEN** user values override defaults while preserving unspecified defaults

#### Scenario: Environment variable override
- **WHEN** KNOWLEDGE_BASE environment variable is set
- **THEN** environment value takes precedence over config file values

### Requirement: Configuration Initialization
The system SHALL create default configuration on first run.

#### Scenario: First-time setup
- **WHEN** no configuration directory exists
- **THEN** system SHALL create ~/.config/local_shelf/ and default config.yaml

#### Scenario: Configuration template
- **WHEN** generating default configuration
- **THEN** config file SHALL include commented examples and documentation

## ADDED Requirements

### Requirement: Configuration Migration Support
The system SHALL support migration from previous configuration locations.

#### Scenario: Legacy config directory detection
- **WHEN** ~/.config/local-shelf/ exists but ~/.config/local_shelf/ does not exist
- **THEN** system SHALL offer to migrate configuration to new location

#### Scenario: Automatic migration
- **WHEN** user approves migration during startup
- **THEN** system SHALL copy config.yaml and remove old directory after successful migration
## 1. Implementation
- [x] 1.1 Add Config enum variant to Commands in src/main.rs
- [x] 1.2 Create ConfigCommands enum with Set sub-subcommand structure
- [x] 1.3 Add config::save() method to Config struct for writing configuration to file
- [x] 1.4 Add config::update_knowledge_base_path() method with validation
- [x] 1.5 Implement handle_config_command() function in main.rs
- [x] 1.6 Add config set command parsing and validation logic
- [x] 1.7 Update CLI help text and documentation for new config subcommand

## 2. Testing
- [x] 2.1 Write unit tests for Config::save() method
- [x] 2.2 Write unit tests for Config::update_knowledge_base_path() method
- [x] 2.3 Write integration tests for config set command with valid paths
- [x] 2.4 Write integration tests for config set command with invalid paths
- [x] 2.5 Write tests for tilde expansion in config set command
- [x] 2.6 Write tests for error handling (empty path, readonly location)
- [x] 2.7 Write tests for CLI help text and invalid sub-subcommands

## 3. Documentation
- [x] 3.1 Update CLI help text to include config subcommand
- [x] 3.2 Add usage examples for config set command
- [x] 3.3 Update error messages to be clear and actionable

## 4. Quality Assurance
- [x] 4.1 Run cargo fmt on all modified files
- [x] 4.2 Run cargo clippy and fix any warnings
- [x] 4.3 Ensure all tests pass with cargo test
- [x] 4.4 Test config command with existing stow/convert workflows
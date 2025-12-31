# Tasks: Rename Project to Snake Case

## Implementation Tasks

### 1. Update Package Configuration
- [x] 1.1 Update `name` field in Cargo.toml from "local-shelf" to "local_shelf"
- [x] 1.2 Verify cargo build succeeds with new name
- [x] 1.3 Test binary generation with correct name

### 2. Update Configuration System
- [x] 2.1 Update config directory path in src/config.rs from "local-shelf" to "local_shelf"
- [x] 2.2 Add migration logic to handle existing config directories
- [x] 2.3 Update configuration tests to use new path
- [x] 2.4 Test config directory creation with new naming

### 3. Update Documentation and Specifications
- [x] 3.1 Update all references in openspec/project.md
- [x] 3.2 Update configuration spec references  
- [x] 3.3 Update AGENTS.md examples and paths
- [x] 3.4 Update code comments in src/ files
- [x] 3.5 Update integration test paths

### 4. Testing and Validation
- [x] 4.1 Run full test suite to ensure no regressions
- [x] 4.2 Test config directory migration functionality
- [x] 4.3 Verify cargo fmt and clippy pass
- [x] 4.4 Test binary execution with new name

### 5. Migration Support
- [x] 5.1 Document migration process for existing users
- [x] 5.2 Consider adding automatic config migration on first run
- [x] 5.3 Update any installation or setup instructions

## Validation Criteria
- All tests pass with `cargo test`
- No clippy warnings with `cargo clippy`  
- Code properly formatted with `cargo fmt`
- Binary builds and executes correctly
- Configuration system works with new directory structure
- All documentation accurately reflects new naming

## Dependencies
- Tasks 1.* can be done in parallel
- Tasks 2.* depend on 1.1 completion
- Tasks 3.* can be done in parallel with 2.*
- Task 4.* should be done after all implementation tasks
- Task 5.* can be done in parallel with documentation updates
# Rename Project from "local-shelf" to "local_shelf"

## Problem
The current project name "local-shelf" uses a hyphenated naming convention, while Rust naming conventions (RFC #430) specify that crates should use `snake_case` and prefer single words. Although hyphenated names are common in the Rust ecosystem, using `local_shelf` would better align with official Rust guidelines.

## Solution
Rename the project from "local-shelf" to "local_shelf" across all occurrences including:
- Cargo.toml package name
- Configuration directory paths  
- Documentation references
- Code comments and examples

## Impact Analysis
### Affected Components
- **Package Name**: Changes Cargo.toml package name field
- **Configuration Paths**: Updates config directory from `~/.config/local-shelf/` to `~/.config/local_shelf/`
- **Documentation**: Updates all references in README, specs, and code comments
- **Migration**: Existing users would need to migrate their config directories

### Breaking Changes
- **Configuration Directory**: Users with existing configurations in `~/.config/local-shelf/` would need to migrate
- **Binary Name**: The compiled binary name would change from `local-shelf` to `local_shelf`

## Constraints
- Must maintain backward compatibility during migration
- Should provide clear migration instructions for existing users
- All tests must continue to pass after rename

## Success Criteria
- All occurrences of "local-shelf" replaced with "local_shelf"
- Configuration system correctly uses new directory path
- All tests pass
- Documentation accurately reflects new naming
- Migration path documented for existing users
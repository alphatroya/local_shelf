# Change: Unify Directory Parameter Handling Across Commands

## Why
Currently, the `stow` and `convert` commands have inconsistent directory parameter handling:
- `stow` always operates on `~/Downloads` with no way to specify a different directory
- `convert` accepts an optional directory parameter, defaulting to the current directory

This inconsistency creates a poor user experience and limits flexibility. Users should be able to specify which directory to operate on for both commands, with consistent default behavior.

## What Changes
- **BREAKING**: Add optional directory parameter to `stow` command
- **BREAKING**: Change `stow` default from `~/Downloads` to current directory (`.`)
- **BREAKING**: Change `convert` default from current directory (`.`) to current directory (no change needed)
- Unify CLI parameter structure so both commands accept `[DIRECTORY]` as optional argument
- Update help text to reflect the unified behavior
- Maintain all other existing functionality for both commands

## Impact
- Affected specs: cli
- Affected code: src/main.rs (CLI argument parsing and command handlers)
- **BREAKING**: `stow` command behavior changes - users must specify `~/Downloads` explicitly if they want the old behavior
- **BREAKING**: `stow` without arguments will now scan current directory instead of `~/Downloads`
- Non-breaking: `convert` behavior remains the same (current directory default)
- Users gain flexibility to run `stow` on any directory, not just `~/Downloads`
## Context
The CLI currently has inconsistent directory parameter handling between `stow` and `convert` commands. This inconsistency makes the interface confusing and limits user flexibility. The `stow` command is hardcoded to always scan `~/Downloads`, while `convert` accepts an optional directory parameter.

## Goals / Non-Goals

### Goals
- Unify CLI parameter structure for consistency  
- Provide users flexibility to specify any directory for both operations
- Maintain clean, intuitive command-line interface
- Preserve all core functionality of both commands

### Non-Goals
- Changing the core file operations logic beyond directory source
- Adding multiple directory support or recursive directory traversal
- Modifying configuration system or Knowledge Base path handling
- Adding complex directory filtering or selection options

## Decisions

### Decision: Make both commands accept optional directory parameter
**Rationale**: Consistent CLI design principles suggest similar operations should have similar interfaces. Both commands operate on directories containing markdown files, so they should accept directory parameters in the same way.

**Alternatives considered**:
- Keep current inconsistent behavior (rejected - poor UX)
- Make directory parameter required for both (rejected - breaks ease of use)
- Add directory parameter only to stow but keep Downloads default (rejected - still inconsistent)

### Decision: Default both commands to current directory
**Rationale**: Current directory is the most common and intuitive default for command-line tools. It matches standard Unix tool behavior and makes the commands more flexible.

**Alternatives considered**:
- Keep stow defaulting to Downloads (rejected - inconsistent with convert)
- Make Downloads the default for both (rejected - unexpected for convert users)
- Keep current defaults (rejected - maintains inconsistency)

### Decision: Accept breaking change for stow command
**Rationale**: The benefit of consistent CLI design outweighs the cost of a breaking change. The application recently moved from v0.1.0 to v0.2.0, indicating breaking changes are acceptable during this phase.

## Risks / Trade-offs

### Risk: Breaking change impacts existing users
**Mitigation**: Clear documentation of the change. Users can easily adapt by specifying `~/Downloads` explicitly when needed.

### Risk: Current directory might not contain markdown files
**Mitigation**: Both commands already handle empty directories gracefully with "No markdown files found" messages.

## Migration Plan

### For Users
- Users who relied on `stow` scanning `~/Downloads` automatically must now run `stow ~/Downloads` 
- Users who run `stow` in directories containing markdown files will get the new behavior immediately
- No changes needed for `convert` command usage

### Implementation
1. Update CLI structure to accept directory parameters consistently
2. Modify stow logic to use parameterized directory instead of hardcoded Downloads
3. Update tests to cover new parameter behavior
4. Document breaking changes in help text

## Open Questions

None - the approach is straightforward with clear user feedback guiding the design decisions.
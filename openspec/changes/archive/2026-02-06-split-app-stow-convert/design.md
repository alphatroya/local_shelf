## Context
The local_shelf application currently provides a single function: organizing markdown files from Downloads into a Knowledge Base. Users have requested conversion capabilities (markdown to EPUB) that exist as a separate shell script. Combining these into a unified CLI improves user experience and maintains the codebase in a single location.

## Goals / Non-Goals

### Goals
- Unified CLI interface with clear subcommands
- Preserve all existing file organization functionality exactly
- Add robust markdown-to-EPUB conversion based on pandoc
- Maintain current configuration system compatibility  
- Comprehensive test coverage for new functionality

### Non-Goals
- Changing the underlying file organization logic
- Adding conversion formats beyond EPUB initially
- Web interface or GUI components
- Advanced pandoc configuration options in first iteration

## Decisions

### Decision: Use clap for CLI parsing
**Rationale**: clap is the de facto standard for Rust CLI applications, provides excellent help generation, argument validation, and subcommand support.

**Alternatives considered**: 
- structopt (deprecated in favor of clap derive)
- Manual argument parsing (too complex for subcommands)

### Decision: Preserve exact stow behavior
**Rationale**: Existing users depend on current functionality. Any behavioral changes should be opt-in features, not breaking changes to core workflow.

### Decision: Direct pandoc integration via process execution
**Rationale**: Pandoc is the gold standard for document conversion and the existing shell script proves this approach works well. A native Rust implementation would be significantly more complex.

**Alternatives considered**:
- Native Rust markdown parsing and EPUB generation (complex, reinventing wheel)
- Other conversion libraries (none as robust as pandoc)

### Decision: Keep conversion functionality simple initially
**Rationale**: The shell script demonstrates that basic pandoc conversion meets user needs. Advanced options can be added later based on user feedback.

## Risks / Trade-offs

### Risk: Pandoc dependency requirement
**Mitigation**: Clear error messages when pandoc is missing, with installation instructions. Consider optional pandoc bundling in future.

### Risk: CLI breaking change
**Mitigation**: Document the change clearly. Consider adding a deprecation warning for direct `local_shelf` execution that suggests `local_shelf stow`.

### Risk: Increased binary size
**Mitigation**: clap adds minimal overhead. Monitor binary size and consider feature flags if needed.

## Migration Plan

### Phase 1: Implementation
1. Add clap dependency and basic CLI structure
2. Move existing main.rs logic to stow subcommand
3. Implement convert subcommand with basic functionality
4. Update all tests

### Phase 2: Compatibility
1. Test thoroughly with existing configurations
2. Document CLI changes
3. Provide migration guidance for users

### Rollback
If issues arise, the CLI structure can be simplified back to single-command mode by reverting main.rs changes and keeping core modules intact.

## Open Questions

**Resolved Questions:**

1. **Should we provide a backward compatibility shim that runs `stow` when no subcommand is provided?**
   - **Decision: No** - Clean break with explicit subcommands provides clearer UX
   
2. **Should conversion support recursive directory traversal beyond single directory depth?** 
   - **Decision: No** - Keep initial implementation simple, matching shell script behavior
   
3. **Do we need configuration options for pandoc executable path or command-line options?**
   - **Decision: No** - Use pandoc from PATH with default options for simplicity
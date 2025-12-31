# Fix Journal Entry Spacing

## Summary
Remove unwanted blank lines between consecutive journal entries when multiple files are processed together, ensuring entries appear consecutively without whitespace gaps.

## Motivation
Currently, when adding multiple journal entries in a single batch operation, blank lines appear between entries:

```
- **16:36** [[The rise of industrial software | Chris Loy]]

- **16:47** [[A SOLID Load of Bull]]
```

This creates visual separation that makes the journal harder to read and is inconsistent with the expected compact format where entries should appear consecutively:

```
- **16:36** [[The rise of industrial software | Chris Loy]]
- **16:47** [[A SOLID Load of Bull]]
```

## Current State
- The `append_entries_to_journal` function adds a newline before new entries if the file has content
- Each entry line gets a newline appended after it
- This results in extra blank lines between entries when multiple entries are processed together
- Existing tests expect consecutive entries without blank lines (test verifies exactly 3 lines for 3 entries)

## Problem Analysis
The issue is in the journal entry appending logic in `src/journal_management.rs` lines 161-170:
1. A newline is added before new entries if the file has content (line 163)
2. Each entry gets a newline after it (line 169)
3. This creates gaps between entries in the same batch

## Proposed Changes
- Modify the entry appending logic to ensure consecutive entries have no blank lines between them
- Maintain proper separation between different batch operations
- Ensure the last entry in a file ends with a newline
- Preserve existing behavior for single entries and file boundaries

## Impact Analysis
- **Breaking Change**: No - improves formatting without changing functionality
- **User Experience**: Improved - cleaner, more readable journal format
- **Implementation**: Minimal - adjust newline handling in append logic
- **Testing**: Existing tests already expect this behavior

## Success Criteria
- Multiple entries in the same batch appear consecutively without blank lines
- Proper separation is maintained between different batch operations
- All existing tests continue to pass
- Journal files maintain proper formatting
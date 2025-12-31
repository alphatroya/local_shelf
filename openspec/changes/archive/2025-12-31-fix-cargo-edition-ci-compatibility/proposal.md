# Fix Cargo Edition CI Compatibility

## Problem Statement
The CI build is failing with the error: "Caused by: this version of Cargo is older than the `2024` edition, and only supports `2015`, `2018`, and `2021` editions."

The project is configured to use Rust edition 2024 in `Cargo.toml`, but the CI workflow specifies a minimum supported Rust version (MSRV) of 1.70.0, which predates the 2024 edition.

## Root Cause
- `Cargo.toml` specifies `edition = "2024"`
- CI workflow uses Rust 1.70.0 as MSRV, which doesn't support the 2024 edition
- Rust edition 2024 requires a more recent version of Rust (1.85.0+)

## Proposed Solution
Simplify the CI configuration by removing the version matrix and using only the stable Rust toolchain, which supports the 2024 edition.

### Options Considered
1. **Downgrade to edition 2021** - More conservative, broader compatibility
2. **Remove version matrix and use stable only** - Simplified CI, uses modern Rust features

**Recommendation:** Remove version matrix and use stable only (Option 2) because:
- The project is new and can adopt modern tooling
- Edition 2024 provides latest language improvements
- Simplifies CI configuration by removing unnecessary complexity
- Stable Rust always supports the latest edition

## Scope
This change affects the continuous integration specification by simplifying the CI configuration to use only stable Rust without a version matrix.

## Dependencies
None - this is a standalone CI configuration fix.

## Validation
- CI builds pass on all specified Rust versions
- All existing tests continue to pass
- Build artifacts remain compatible
# Continuous Integration Specification

## ADDED Requirements

### Requirement: CI builds support current Rust edition
The continuous integration system MUST use Rust toolchain versions that are compatible with the project's specified Rust edition.

#### Scenario: Simplified CI workflow with stable Rust
- **Given** the project uses Rust edition 2024 in Cargo.toml
- **When** the CI workflow runs quality gates (format, clippy, test, build)
- **Then** all jobs complete successfully without edition compatibility errors
- **And** builds work using the stable Rust toolchain

#### Scenario: CI workflow without version matrix
- **Given** the project specifies a Rust edition in Cargo.toml
- **When** the CI workflow is configured
- **Then** it MUST use stable Rust toolchain without a version matrix
- **And** the toolchain version supports the specified edition
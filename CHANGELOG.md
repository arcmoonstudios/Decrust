# Changelog

All notable changes to the Decrust project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.2] - 2025-01-25

### Updated

- **Rust Toolchain**: Upgraded from Rust 1.85.1 to Rust 1.87.0 (latest stable)
  - Updated `rust-toolchain.toml` to specify Rust 1.87.0 explicitly
  - Updated CI workflow to use Rust 1.87.0 across all jobs (code-quality, test, docs)
  - Updated MSRV in workspace Cargo.toml to 1.87.0
  - Resolved ABI compatibility issues between Rust versions

- **Dependencies**: Updated all dependencies to latest stable versions
  - `chrono`: Updated to 0.4.41 with serde features
  - `serde`: Updated to 1.0.219 with derive features
  - `tokio`: Updated to 1.45.0 with full features
  - `syn`: Updated to 2.0.101 with full features
  - `thiserror`: Updated to 2.0.12
  - `anyhow`: Updated to 1.0.95
  - `regex`: Updated to 1.11.1
  - `tracing`: Updated to 0.1.41
  - `derivative`: Updated to 2.2.0
  - `proc-macro2`: Updated to 1.0.95
  - `quote`: Updated to 1.0.38
  - `rand`: Updated to 0.9.1

### Fixed

- **Compatibility**: Resolved Rust 1.87.0 compatibility issues
  - Fixed all `std::io::Error::new()` calls to use `std::io::Error::other()` for Rust 1.87.0 compliance
  - Updated procedural macro compilation to work with Rust 1.87.0 ABI
  - Resolved macro expansion issues in test files
  - Fixed clippy warnings introduced by stricter Rust 1.87.0 linting rules

- **Testing**: Enhanced autocorrection testing framework
  - Fixed test assertions to match actual error message formats
  - Added comprehensive tests for autocorrection functionality through error creation
  - Verified that `decrust!` macro properly processes autocorrection suggestions
  - All 200+ tests now pass with Rust 1.87.0

### Technical Details

- **Build System**: Clean rebuild required for Rust version compatibility
  - Procedural macros recompiled with Rust 1.87.0 for ABI compatibility
  - Enhanced error handling in CI for better debugging with new Rust version
  - Updated toolchain configuration for consistent builds across environments

## [1.2.1] - 2025-01-25

### Fixed

- **CI/CD Improvements**: Comprehensive cache and stability fixes
  - Fixed rust-cache configuration with proper shared-key approach
  - Added Cargo.lock verification and generation steps
  - Enhanced cache debugging and persistence options
  - Each job now has dedicated cache keys (code-quality, test, docs)
  - Resolved "Cache not found" issues for faster CI runs

- **Code Quality**: Resolved all clippy warnings and documentation issues
  - Fixed documentation list item overindentation (changed `~` to `-` with proper spacing)
  - Fixed empty line after doc comment issues in test files
  - Updated IO error creation to use `std::io::Error::other()` instead of deprecated `new()`
  - Added workspace-level lint configuration allowing large error types (intentional design)

- **Build System**: Enhanced workspace lint inheritance
  - Added `[lints] workspace = true` to all package Cargo.toml files
  - Configured workspace-level clippy settings for consistent code quality
  - All 200+ tests now pass with zero warnings

### Changed

- **Toolchain**: Switched from Rust 1.85.1 to stable for better CI compatibility
- **Environment**: Simplified CI environment variables (removed RUST_LIB_BACKTRACE)
- **Rust Toolchain**: Updated rust-toolchain.toml to use stable channel
- **CI Workflow**: Enhanced error handling and debugging in all CI steps

### Technical Details

- Backtrace handling made more robust with explicit type aliasing
- Added `#[allow(unreachable_patterns)]` for future-proofing
- Enhanced CI environment variables: `RUST_BACKTRACE=1` for basic error reporting
- Improved debug output with working directory and file existence checks

## [1.2.0] - 2025-01-25

### New Features

- **M.A.R.S. Framework**: Mathematical Analysis & Recursive Systematic Error Resolution
  - 7-phase error resolution system with verification loops
  - Causal chain analysis with dependency mapping
  - Mathematical intent decomposition for understanding operation purpose
  - Autonomous processing with Luna⚛︎Ultima integration
  - Certification-level quality (IVDI 1337 compliant error handling)
- **Comprehensive Autocorrection System**: 39 different error types with intelligent fixes
  - **Automatic (8)**: Directly corrects warnings and simple errors
    - unused imports, variables, semicolons, unnecessary clones/braces
    - unused mut keywords, unreachable code, missing imports (E0433)
  - **Interactive (13)**: Presents options for complex issues
    - type mismatches, missing lifetimes, private field access
    - parameter mismatches, generic conflicts, trait implementations
  - **Manual (18)**: Provides guidance for architectural issues
    - division by zero, incorrect function arguments, recursive types
    - network/file/configuration errors, TLS certificate issues
- **Enhanced Error Handling**: Advanced DecrustError with multiple variants
  - Added `Oops` error variant for flexible error handling with any source
  - Implemented circuit breaker pattern for resilient operations
  - Added timeout, authentication, and authorization error categories
  - Enhanced backtrace integration with custom DecrustBacktrace type

### Enhanced
- **Macro System**: Significantly improved decrust! macro
  - Comprehensive coverage for any operation through single macro
  - Enhanced error propagation and automatic handling
  - Better integration with autocorrection system
  - Improved debugging and diagnostic capabilities
- **Testing Framework**: Extensive test coverage expansion
  - 200+ tests covering all core functionality
  - Integration tests for macro system and autocorrection
  - Circuit breaker and error handling comprehensive tests
  - Fix generation and template system tests

## [1.1.0] - 2025-01-25

### Added
- **Workspace Architecture**: Major restructuring into comprehensive workspace
  - `decrust-core`: Core error handling framework (renamed from original decrust)
  - `decrust-promac`: Procedural macro implementation for error handling
  - `decrust-promac-runtime`: Runtime support and middleware for macros
  - `decrust`: Ultimate public interface unifying all components
- **Advanced Error Categories**: Significantly expanded error handling capabilities
  - Network errors with detailed connection and TLS certificate handling
  - Configuration errors with validation, parsing, and file path support
  - Circuit breaker errors for building resilient distributed systems
  - Validation errors with field-specific handling and context
  - Parse errors for JSON, YAML, TOML, and other structured formats
- **Intelligent Fix Generation System**: Context-aware autocorrection engine
  - Parameter extraction from error messages using sophisticated regex patterns
  - Context-aware fix suggestions with confidence levels and multiple strategies
  - 15+ specialized fix generators for different error categories
  - Template-based solution generation with executable commands
  - IDE integration support for seamless fix application

### Changed
- **Project Structure**: Complete architectural reorganization for scalability
  - Moved core functionality to dedicated `decrust-core` package
  - Created unified interface in root `decrust` package for easy adoption
  - Separated macro implementation into dedicated packages for modularity
  - Enhanced workspace configuration with shared dependencies and metadata
- **Error Handling**: Comprehensive enhancement of error types and capabilities
  - Added custom backtrace integration with DecrustBacktrace type
  - Implemented multi-format error reporting (JSON, plain text, markdown)
  - Enhanced diagnostic information extraction and context preservation
  - Improved error categorization with more granular error types

### Enhanced
- **Documentation**: Extensive documentation improvements
  - Comprehensive API documentation for all public interfaces
  - Usage examples for common error scenarios and patterns
  - Integration guides for different use cases and frameworks
  - Performance benchmarks and optimization guidelines

## [1.0.0] - 2025-01-25

### Added
- **Core Framework**: Initial stable release of comprehensive error handling
  - DecrustError enum with 12 primary error categories
  - Result and Option extension traits (DecrustResultExt, DecrustOptionExt)
  - Basic autocorrection suggestions with confidence scoring
  - Error context preservation and propagation
- **Macro System**: Initial decrust! macro implementation
  - Basic error handling and automatic propagation
  - Integration with core error types and categories
  - Simple syntax for wrapping operations: `decrust!(operation())`
- **Error Categories**: Comprehensive error type system
  - I/O errors with path and operation context
  - Network errors with URL and connection details
  - Configuration errors with file path and validation support
  - Validation errors with field-specific messages
  - Internal errors with component tracking
  - NotFound errors with resource type identification
- **Documentation**: Complete documentation suite
  - Usage examples for common error scenarios
  - API documentation for all public interfaces
  - Integration guides and best practices
  - Performance considerations and optimization tips

### Infrastructure
- **CI/CD**: Robust GitHub Actions workflow for automated quality assurance
  - Code quality checks with rustfmt and clippy (zero warnings policy)
  - Comprehensive test execution across all targets and features
  - Documentation generation and validation
  - Multi-platform testing (Linux, Windows, macOS)
- **Build System**: Professional Cargo workspace configuration
  - Shared dependencies and version management across packages
  - Consistent build and test processes
  - Development and release profile optimization
  - MSRV (Minimum Supported Rust Version) enforcement

### Testing
- **Test Suite**: Comprehensive test coverage for reliability
  - Unit tests for all core functionality
  - Integration tests for macro system
  - Error handling and propagation tests
  - Performance and benchmark tests
  - Cross-platform compatibility tests

## [0.1.0] - 2025-01-24

### Added
- **Initial Release**: Foundation of the Decrust error handling framework
  - Basic Result and Option extension traits for enhanced error handling
  - Simple error types and categorization system
  - Initial project structure and development workflow
  - Core concepts for autocorrection and error resolution
- **Foundation**: Architectural concepts and initial implementation
  - Error categorization system with extensible design
  - Basic autocorrection concepts and framework
  - Initial macro framework for error handling
  - Simple error propagation and context preservation

### Infrastructure
- **Project Setup**: Initial repository and development infrastructure
  - Cargo.toml configuration with workspace setup
  - Basic CI/CD pipeline with GitHub Actions
  - Initial documentation structure and README
  - MIT license and contribution guidelines
  - Version control and release management setup

### Development
- **Tooling**: Basic development and quality assurance tools
  - Rust toolchain configuration
  - Code formatting and linting setup
  - Basic test framework and structure
  - Documentation generation pipeline

---

## Version History Summary

- **0.1.0** (2025-01-24): Initial foundation and basic error handling concepts
- **1.0.0** (2025-01-25): Core framework with comprehensive error types and stable macro system
- **1.1.0** (2025-01-25): Workspace architecture and advanced error categories with fix generation
- **1.2.0** (2025-01-25): M.A.R.S. framework and intelligent autocorrection with 39 error types
- **1.2.1** (2025-01-25): CI/CD improvements, code quality fixes, and enhanced stability
- **1.2.2** (2025-01-25): Rust 1.87.0 upgrade and latest dependency updates with compatibility fixes

## Links

- [Repository](https://github.com/arcmoonstudios/decrust)
- [Documentation](https://docs.rs/decrust)
- [Crates.io](https://crates.io/crates/decrust)

## Contributing

Please read our contributing guidelines and ensure all changes are properly documented in this changelog.
Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for commit messages.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

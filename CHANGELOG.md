# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-XX

### Added
- Initial release of Decrust error handling framework
- Core error types and error handling infrastructure
- Comprehensive error categorization system
- Backtrace support for all error types
- Circuit breaker pattern implementation
- Error reporting and formatting utilities
- Syntax error detection and autocorrection suggestions
- Parameter extraction from error messages
- Fix generation for common Rust compilation errors
- Template-based error correction system
- Multi-crate workspace architecture:
  - `decrust` - Main error handling library
  - `decrust-promac` - Procedural macros for error handling
  - `decrust-promac-runtime` - Runtime support for macros

### Error Categories
- I/O errors with path and operation context
- Network errors with URL and connection details
- Configuration errors with file path support
- Validation errors with field-specific messages
- Parse errors for JSON, YAML, and other formats
- Circuit breaker and timeout handling
- Resource exhaustion tracking
- Concurrency and state conflict detection
- External service integration errors
- Style and formatting suggestions

### Features
- Zero-cost abstractions for performance-critical code
- Seamless integration with Rust's standard error handling
- Context-aware error propagation
- Rich error context with diagnostic information
- Automatic fix suggestions for common errors
- IDE integration support for error corrections
- Comprehensive test coverage (198+ tests)
- Cross-platform support (Linux, Windows, macOS)

### Development Tools
- Comprehensive CI/CD pipeline with GitHub Actions and Cirrus CI
- Automated dependency management with Dependabot
- Security auditing with cargo-audit
- Documentation generation and testing
- Multi-platform testing and benchmarking
- MSRV support for Rust 1.86+

[Unreleased]: https://github.com/arcmoonstudios/decrust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/arcmoonstudios/decrust/releases/tag/v0.1.0

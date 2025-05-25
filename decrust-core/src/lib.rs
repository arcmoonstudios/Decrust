/* src/lib.rs */
#![warn(missing_docs)]
#![allow(stable_features)]
#![allow(clippy::result_large_err)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::new_without_default)]
#![allow(clippy::useless_format)]
#![allow(clippy::unnecessary_to_owned)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::single_char_add_str)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::useless_vec)]
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::await_holding_lock)]
#![allow(clippy::unwrap_or_default)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::overly_complex_bool_expr)]
#![allow(clippy::len_zero)]
//! # Decrust: Advanced Error Handling Framework for Rust
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! Decrust is a comprehensive, production-ready error handling framework that provides
//! rich error context, automatic error recovery, circuit breaker patterns, and powerful
//! debugging capabilities. It's designed to make error handling in Rust applications
//! both robust and developer-friendly.
//!
//! ## 🚀 Quick Start
//!
//! ```rust
//! use decrust_core::{DecrustError, DecrustResultExt, DecrustOptionExt, oops, validation_error};
//!
//! // Basic error creation with rich context
//! fn process_user_data(data: Option<&str>) -> Result<String, DecrustError> {
//!     let user_data = data.decrust_ok_or_missing_value("user data")?;
//!
//!     if user_data.is_empty() {
//!         return Err(validation_error!("user_data", "Data cannot be empty"));
//!     }
//!
//!     // Simulate an IO operation that might fail
//!     std::fs::read_to_string("config.json")
//!         .map_err(|e| oops!("Failed to read configuration", e))
//!         .and_then(|_| Ok(format!("Processed: {}", user_data)))
//! }
//! ```
//!
//! ## 🎯 Core Features
//!
//! ### 1. **Rich Error Context** 📍
//! Every error includes comprehensive context with location tracking, severity levels,
//! and metadata for better debugging and monitoring.
//!
//! ```rust
//! use decrust_core::{error_context, types::ErrorSeverity, oops};
//!
//! // Create rich error context with metadata
//! let context = error_context!(
//!     "Database connection failed",
//!     severity: ErrorSeverity::Critical
//! ).with_component("database")
//!  .with_correlation_id("req-123")
//!  .with_recovery_suggestion("Check database connectivity");
//!
//! // Use in error creation
//! let io_error = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused");
//! let error = oops!("Database unavailable", io_error, severity: ErrorSeverity::Critical);
//! ```
//!
//! ### 2. **Circuit Breaker Pattern** ⚡
//! Built-in circuit breaker for handling external service failures gracefully.
//!
//! ```rust
//! use decrust_core::{circuit_breaker::{CircuitBreaker, CircuitBreakerConfig}, DecrustError, Backtrace};
//! use std::time::Duration;
//!
//! // Configure circuit breaker
//! let config = CircuitBreakerConfig {
//!     failure_threshold: 5,
//!     reset_timeout: Duration::from_secs(30),
//!     operation_timeout: Some(Duration::from_secs(5)),
//!     ..Default::default()
//! };
//!
//! let circuit_breaker = CircuitBreaker::new("external-api", config);
//!
//! // Execute operations through circuit breaker
//! # fn external_api_call() -> Result<String, std::io::Error> { Ok("success".to_string()) }
//! let result = circuit_breaker.execute(|| {
//!     // Your external service call here
//!     external_api_call().map_err(|e| DecrustError::Oops {
//!         message: "API call failed".to_string(),
//!         source: Box::new(e),
//!         backtrace: Backtrace::generate(),
//!     })
//! });
//! ```
//!
//! ### 3. **Automatic Error Recovery** 🔄
//! Smart error recovery with configurable retry strategies and fix suggestions.
//!
//! ```rust
//! use decrust_core::{DecrustError, decrust::{Decrust, AutocorrectableError}, Backtrace};
//!
//! let mut decrust = Decrust::new();
//!
//! // Register custom fix generators
//! // decrust.register_fix_generator(Box::new(CustomFixGenerator::new()));
//!
//! // Apply fixes automatically
//! # let error = DecrustError::Validation {
//! #     field: "test".to_string(),
//! #     message: "test".to_string(),
//! #     expected: None,
//! #     actual: None,
//! #     rule: None,
//! #     backtrace: Backtrace::generate()
//! # };
//! if let Some(fix) = decrust.suggest_autocorrection(&error, None) {
//!     println!("Suggested fix: {}", fix.description);
//! }
//! ```
//!
//! ### 4. **Powerful Macros** 🛠️
//! Ergonomic macros for common error handling patterns.
//!
//! ```rust
//! use decrust_core::{oops, validation_error, error_context, location, types::ErrorSeverity};
//!
//! // Quick error creation
//! # let source_error = std::io::Error::new(std::io::ErrorKind::Other, "test");
//! let error = oops!("Something went wrong", source_error);
//!
//! // Validation errors with suggestions
//! let validation_err = validation_error!(
//!     "email",
//!     "Invalid email format",
//!     suggestion: "Use format: user@domain.com"
//! );
//!
//! // Rich context with location tracking
//! let context = error_context!("Operation failed", severity: ErrorSeverity::Error);
//! let loc = location!(context: "user authentication", function: "login");
//! ```
//!
//! ### 5. **Comprehensive Error Types** 📋
//! Pre-built error variants for common scenarios with rich metadata.
//!
//! ```rust
//! use decrust_core::{DecrustError, Backtrace, OptionalError};
//! use std::time::Duration;
//!
//! // Network errors with retry information
//! let network_error = DecrustError::Network {
//!     source: Box::new(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "refused")),
//!     kind: "HTTP".to_string(),
//!     url: Some("https://api.example.com".to_string()),
//!     backtrace: Backtrace::generate(),
//! };
//!
//! // Configuration errors with suggestions
//! let config_error = DecrustError::Config {
//!     message: "Invalid database URL format".to_string(),
//!     path: Some("config.toml".into()),
//!     source: OptionalError(None),
//!     backtrace: Backtrace::generate(),
//! };
//! ```
//!
//! ## 🔧 Advanced Usage Patterns
//!
//! ### Creating Custom Error Types
//!
//! You can create domain-specific error types that integrate seamlessly with Decrust:
//!
//! ```rust
//! use decrust_core::{DecrustError, DecrustResultExt, types::ErrorSeverity, Backtrace, OptionalError};
//!
//! # struct User;
//! // Define your domain-specific error
//! #[derive(Debug)]
//! pub enum UserServiceError {
//!     NotFound { id: String },
//!     InvalidEmail { email: String },
//!     PermissionDenied { user_id: String },
//! }
//!
//! impl std::fmt::Display for UserServiceError {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         match self {
//!             UserServiceError::NotFound { id } => write!(f, "User not found: {}", id),
//!             UserServiceError::InvalidEmail { email } => write!(f, "Invalid email format: {}", email),
//!             UserServiceError::PermissionDenied { user_id } => write!(f, "Permission denied for user: {}", user_id),
//!         }
//!     }
//! }
//!
//! impl std::error::Error for UserServiceError {}
//!
//! // Convert to DecrustError with rich context
//! impl From<UserServiceError> for DecrustError {
//!     fn from(err: UserServiceError) -> Self {
//!         match err {
//!             UserServiceError::NotFound { id } => DecrustError::NotFound {
//!                 resource_type: "User".to_string(),
//!                 identifier: id,
//!                 backtrace: Backtrace::generate(),
//!             },
//!             UserServiceError::InvalidEmail { email } => DecrustError::Validation {
//!                 field: "email".to_string(),
//!                 message: format!("Invalid email format: {}", email),
//!                 expected: None,
//!                 actual: None,
//!                 rule: None,
//!                 backtrace: Backtrace::generate(),
//!             },
//!             UserServiceError::PermissionDenied { user_id } => {
//!                 DecrustError::ExternalService {
//!                     service_name: "UserService".to_string(),
//!                     message: format!("Permission denied for user: {}", user_id),
//!                     source: OptionalError(None),
//!                     backtrace: Backtrace::generate(),
//!                 }
//!             }
//!         }
//!     }
//! }
//!
//! // Usage in your application
//! fn get_user(id: &str) -> Result<User, DecrustError> {
//!     // Your business logic here
//!     if id.is_empty() {
//!         return Err(UserServiceError::NotFound { id: id.to_string() }.into());
//!     }
//!
//!     // Add rich context to any errors
//!     database_call()
//!         .map_err(|e| DecrustError::Oops {
//!             message: "Database query failed".to_string(),
//!             source: Box::new(e),
//!             backtrace: Backtrace::generate(),
//!         })
//!         .decrust_context_msg("Fetching user from database")?;
//!
//!     Ok(User)
//! }
//! # fn database_call() -> Result<(), std::io::Error> { Ok(()) }
//! ```
//!
//! ### Error Reporting and Monitoring
//!
//! ```rust
//! use decrust_core::{ErrorReporter, ErrorReportConfig, types::ErrorReportFormat, DecrustError, Backtrace};
//!
//! // Configure error reporting
//! let config = ErrorReportConfig {
//!     format: ErrorReportFormat::Json,
//!     include_backtrace: true,
//!     include_rich_context: true,
//!     ..Default::default()
//! };
//!
//! let reporter = ErrorReporter::new();
//!
//! // Report errors with rich context
//! # let error = DecrustError::Validation {
//! #     field: "test".to_string(),
//! #     message: "test".to_string(),
//! #     expected: None,
//! #     actual: None,
//! #     rule: None,
//! #     backtrace: Backtrace::generate()
//! # };
//! let report = reporter.report_to_string(&error, &config);
//! println!("Error Report: {}", report);
//! ```
//!
//! ### Circuit Breaker with Custom Policies
//!
//! ```rust
//! use decrust_core::{circuit_breaker::{CircuitBreaker, CircuitBreakerConfig}, DecrustError};
//! use std::time::Duration;
//!
//! // Advanced circuit breaker configuration
//! let config = CircuitBreakerConfig {
//!     failure_threshold: 3,                    // Open after 3 failures
//!     success_threshold_to_close: 2,           // Close after 2 successes in half-open
//!     reset_timeout: Duration::from_secs(60),  // Try half-open after 60 seconds
//!     operation_timeout: Some(Duration::from_secs(10)), // Individual operation timeout
//!     half_open_max_concurrent_operations: 1, // Only 1 operation in half-open
//!     ..Default::default()
//! };
//!
//! let circuit_breaker = CircuitBreaker::new("payment-service", config);
//!
//! // Use with async operations (when std-thread feature is enabled)
//! let result = circuit_breaker.execute(|| {
//!     // Your potentially failing operation
//!     call_payment_service()
//! });
//!
//! match result {
//!     Ok(response) => println!("Payment successful: {:?}", response),
//!     Err(DecrustError::CircuitBreakerOpen { retry_after, .. }) => {
//!         println!("Circuit breaker is open, retry after: {:?}", retry_after);
//!     }
//!     Err(e) => println!("Payment failed: {}", e),
//! }
//! # fn call_payment_service() -> Result<String, DecrustError> { Ok("success".to_string()) }
//! ```
//!
//! ### Object-Safe Extension Trait Usage
//!
//! The extension traits are object-safe and support dynamic dispatch:
//!
//! ```rust
//! use decrust_core::{DecrustResultExt, DecrustOptionExt, DecrustError};
//!
//! // Object-safe trait usage with dynamic dispatch
//! fn process_with_dyn_traits(
//!     result: &dyn DecrustResultExt<String, std::io::Error>,
//!     option: &dyn DecrustOptionExt<i32>
//! ) {
//!     // These work because the traits are object-safe
//! }
//!
//! // Regular usage for better error handling
//! fn process_data() -> Result<String, DecrustError> {
//!     let result: Result<String, std::io::Error> = Ok("test".to_string());
//!     let option: Option<i32> = Some(42);
//!
//!     // Add context to results (object-safe methods)
//!     let processed = result.decrust_context_msg("Processing data")?;
//!
//!     // Convert options to results with meaningful errors (object-safe methods)
//!     let value = option.decrust_ok_or_missing_value("required value")?;
//!
//!     Ok(format!("{} - {}", processed, value))
//! }
//! ```
//!
//! ## 📚 Feature Flags
//!
//! - `std-thread`: Enables threading support for circuit breaker timeouts
//! - `serde`: Enables serialization support for error types
//! - `tracing`: Enables integration with the tracing ecosystem
//!
//! ## 🎨 Best Practices
//!
//! 1. **Use specific error variants** for different error categories
//! 2. **Add rich context** with `decrust_context_msg()` for better debugging
//! 3. **Implement circuit breakers** for external service calls
//! 4. **Use macros** for common error patterns to reduce boilerplate
//! 5. **Configure error reporting** for production monitoring
//! 6. **Create domain-specific error types** that convert to `DecrustError`
//!
//! ## 🔗 Integration Examples
//!
//! ### With Tokio and Async
//! ```rust
//! use decrust_core::{DecrustError, DecrustResultExt, Backtrace};
//! use std::path::PathBuf;
//!
//! // Simulate async file reading without requiring tokio dependency
//! async fn read_config_async() -> Result<String, DecrustError> {
//!     // Simulate reading a config file
//!     let result = std::fs::read_to_string("Cargo.toml") // Use existing file
//!         .map_err(|e| DecrustError::Io {
//!             source: e,
//!             path: Some("Cargo.toml".into()),
//!             operation: "read config file".to_string(),
//!             backtrace: Backtrace::generate(),
//!         })
//!         .decrust_context_msg("Loading application configuration")?;
//!
//!     Ok(result)
//! }
//!
//! // Test the async function (without actually running it)
//! let _future = read_config_async();
//! ```
//!
//! ### With Configuration Parsing
//! ```rust
//! use decrust_core::{DecrustError, Backtrace, OptionalError};
//! use std::path::PathBuf;
//!
//! // Simple configuration struct (without serde dependency)
//! struct AppConfig {
//!     database_url: String,
//!     api_key: String,
//! }
//!
//! fn load_config() -> Result<AppConfig, DecrustError> {
//!     // Simulate reading configuration from Cargo.toml (which exists)
//!     let config_str = std::fs::read_to_string("Cargo.toml")
//!         .map_err(|e| DecrustError::Config {
//!             message: "Failed to read configuration file".to_string(),
//!             path: Some("Cargo.toml".into()),
//!             source: OptionalError::new(Some(Box::new(e))),
//!             backtrace: Backtrace::generate(),
//!         })?;
//!
//!     // Simulate parsing (just create a dummy config)
//!     let config = AppConfig {
//!         database_url: "postgresql://localhost/mydb".to_string(),
//!         api_key: "dummy_key".to_string(),
//!     };
//!
//!     // Validate configuration
//!     if config.database_url.is_empty() {
//!         return Err(DecrustError::Validation {
//!             field: "database_url".to_string(),
//!             message: "Database URL cannot be empty".to_string(),
//!             expected: None,
//!             actual: None,
//!             rule: None,
//!             backtrace: Backtrace::generate(),
//!         });
//!     }
//!
//!     Ok(config)
//! }
//!
//! // Test the function
//! let _config = load_config();
//! ```
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me

pub mod backtrace;
pub mod circuit_breaker;
pub mod decrust;
pub mod reporter;
pub mod syntax;
pub mod types;

use std::path::PathBuf;
use std::time::Duration;

pub use self::backtrace::{
    AsBacktrace,
    BacktraceCompat,
    BacktraceFrame,
    // FromString,                 // Will add back if `oops!` macro or FromString trait is used directly
    // ensure,                     // Will add back if used
    BacktraceProvider,
    BacktraceStatus,
    DecrustBacktrace as Backtrace, // For Backtrace::generate()
    GenerateImplicitData,
    Location,
    ThreadId,
    Timestamp,
};

// Macros are automatically exported at crate root due to #[macro_export]
// Available macros: implicit_data!, location!, error_context!, oops!, validation_error!

pub use self::circuit_breaker::{
    CircuitBreaker, CircuitBreakerConfig, CircuitBreakerObserver, CircuitBreakerState,
    CircuitMetrics, CircuitOperationType, CircuitTransitionEvent,
};

pub use self::decrust::{
    AstMissingImportFixGenerator, AstUnusedCodeFixGenerator, AutocorrectableError,
    ClosureCaptureLifetimeFixGenerator, ConfigMissingKeyFixGenerator, ConfigSyntaxFixGenerator,
    CrateUsageAnalysis, Decrust, DependencyAnalysisResult, DependencyAnalyzer,
    DivisionByZeroFixGenerator, InteractiveRecommendation, InvalidArgumentCountFixGenerator,
    IoMissingDirectoryFixGenerator, IoPermissionFixGenerator, JsonParseFixGenerator,
    MissingOkErrFixGenerator, NetworkConnectionFixGenerator, NetworkTlsFixGenerator,
    OptimizationImpact, QuestionMarkPropagationFixGenerator, RecommendationType,
    RecursiveTypeFixGenerator, ReturnLocalReferenceFixGenerator, RuntimePanicFixGenerator,
    SecurityImpact, UnnecessaryCloneFixGenerator, UnnecessaryParenthesesFixGenerator,
    UnsafeUnwrapFixGenerator, UnstableFeatureFixGenerator, UnusedMutFixGenerator,
    VersionCompatibility, YamlParseFixGenerator,
};

pub use self::reporter::{ErrorReportConfig, ErrorReporter};

pub use self::syntax::{FixTemplate, SyntaxGenerator, TemplateRegistry};

pub use self::types::{
    Autocorrection, ErrorCategory, ErrorContext, ErrorReportFormat, ErrorSeverity, ErrorSource,
    ExtractedParameters, FixDetails, FixType, ParameterExtractor, ParameterSource,
};

/// A Result type specialized for DecrustError
pub type Result<T, E = DecrustError> = std::result::Result<T, E>;

// Re-export key types from submodules
/// A Result type specialized for diagnostic operations that can return multiple errors
pub type DiagnosticResult<T> = std::result::Result<T, Vec<DecrustError>>;

/// Wrapper for `Option<Box<dyn Error>>` to make it compatible with backtrace
///
/// This struct provides a way to handle optional error sources in a way that's
/// compatible with the backtrace error handling framework. It wraps an optional boxed
/// error trait object and provides methods to work with it.
#[derive(Debug)]
pub struct OptionalError(pub Option<Box<dyn std::error::Error + Send + Sync + 'static>>);

impl Clone for OptionalError {
    fn clone(&self) -> Self {
        match &self.0 {
            Some(err) => {
                // Create a new error with the string representation of the original error
                let cloned_err = std::io::Error::other(format!("{}", err));
                OptionalError(Some(Box::new(cloned_err)))
            }
            None => OptionalError(None),
        }
    }
}

impl std::fmt::Display for OptionalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(err) => write!(f, "{}", err),
            None => write!(f, "No error"),
        }
    }
}

impl std::error::Error for OptionalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0 {
            Some(err) => Some(err.as_ref()),
            None => None,
        }
    }
}

// Implement BacktraceCompat for DecrustError to make backtrace() work
// Implement std::error::Error for DecrustError
impl std::error::Error for DecrustError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DecrustError::Io { source, .. } => Some(source),
            DecrustError::WithRichContext { source, .. } => Some(source.as_ref()),
            DecrustError::Oops { source, .. } => Some(source.as_ref()),
            DecrustError::Parse { source, .. } => Some(source.as_ref()),
            DecrustError::Network { source, .. } => Some(source.as_ref()),
            DecrustError::Config { source, .. } => source
                .0
                .as_ref()
                .map(|e| e.as_ref() as &(dyn std::error::Error + 'static)),
            DecrustError::Internal { source, .. } => source
                .0
                .as_ref()
                .map(|e| e.as_ref() as &(dyn std::error::Error + 'static)),
            DecrustError::Concurrency { source, .. } => source
                .0
                .as_ref()
                .map(|e| e.as_ref() as &(dyn std::error::Error + 'static)),
            DecrustError::ExternalService { source, .. } => source
                .0
                .as_ref()
                .map(|e| e.as_ref() as &(dyn std::error::Error + 'static)),
            DecrustError::MultipleErrors { errors, .. } => errors
                .first()
                .map(|e| e as &(dyn std::error::Error + 'static)),
            DecrustError::CircuitBreakerOpen { .. } => None,
            DecrustError::ResourceExhausted { .. } => None,
            DecrustError::StateConflict { .. } => None,
            DecrustError::MissingValue { .. } => None,
            DecrustError::Validation { .. } => None,
            DecrustError::NotFound { .. } => None,
            DecrustError::Timeout { .. } => None,
            DecrustError::Style { .. } => None,
        }
    }
}

// Implement PartialEq for DecrustError to support testing
impl PartialEq for DecrustError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DecrustError::Parse { kind: k1, .. }, DecrustError::Parse { kind: k2, .. }) => {
                k1 == k2
            }
            (DecrustError::Oops { message: m1, .. }, DecrustError::Oops { message: m2, .. }) => {
                m1 == m2
            }
            (DecrustError::Network { kind: k1, .. }, DecrustError::Network { kind: k2, .. }) => {
                k1 == k2
            }
            (DecrustError::Style { message: m1, .. }, DecrustError::Style { message: m2, .. }) => {
                m1 == m2
            }
            (
                DecrustError::Config { message: m1, .. },
                DecrustError::Config { message: m2, .. },
            ) => m1 == m2,
            (DecrustError::Io { operation: op1, .. }, DecrustError::Io { operation: op2, .. }) => {
                op1 == op2
            }
            (
                DecrustError::Internal { message: m1, .. },
                DecrustError::Internal { message: m2, .. },
            ) => m1 == m2,
            (
                DecrustError::Concurrency { message: m1, .. },
                DecrustError::Concurrency { message: m2, .. },
            ) => m1 == m2,
            (
                DecrustError::Timeout { operation: op1, .. },
                DecrustError::Timeout { operation: op2, .. },
            ) => op1 == op2,
            (
                DecrustError::StateConflict { message: m1, .. },
                DecrustError::StateConflict { message: m2, .. },
            ) => m1 == m2,
            (
                DecrustError::CircuitBreakerOpen { name: n1, .. },
                DecrustError::CircuitBreakerOpen { name: n2, .. },
            ) => n1 == n2,
            (
                DecrustError::ResourceExhausted { resource: r1, .. },
                DecrustError::ResourceExhausted { resource: r2, .. },
            ) => r1 == r2,
            (
                DecrustError::ExternalService {
                    service_name: s1, ..
                },
                DecrustError::ExternalService {
                    service_name: s2, ..
                },
            ) => s1 == s2,
            (
                DecrustError::MissingValue {
                    item_description: i1,
                    ..
                },
                DecrustError::MissingValue {
                    item_description: i2,
                    ..
                },
            ) => i1 == i2,
            (
                DecrustError::MultipleErrors { errors: e1, .. },
                DecrustError::MultipleErrors { errors: e2, .. },
            ) => e1 == e2,
            (
                DecrustError::Validation {
                    field: f1,
                    message: m1,
                    ..
                },
                DecrustError::Validation {
                    field: f2,
                    message: m2,
                    ..
                },
            ) => f1 == f2 && m1 == m2,
            (
                DecrustError::NotFound {
                    resource_type: r1,
                    identifier: i1,
                    ..
                },
                DecrustError::NotFound {
                    resource_type: r2,
                    identifier: i2,
                    ..
                },
            ) => r1 == r2 && i1 == i2,
            (
                DecrustError::WithRichContext {
                    context: c1,
                    source: s1,
                },
                DecrustError::WithRichContext {
                    context: c2,
                    source: s2,
                },
            ) => c1.message == c2.message && s1 == s2,
            _ => false,
        }
    }
}

impl backtrace::BacktraceCompat for DecrustError {
    fn backtrace(&self) -> Option<&backtrace::DecrustBacktrace> {
        match self {
            DecrustError::Io { backtrace, .. } => Some(backtrace),
            DecrustError::Oops { backtrace, .. } => Some(backtrace),
            DecrustError::Style { backtrace, .. } => Some(backtrace),
            DecrustError::Parse { backtrace, .. } => Some(backtrace),
            DecrustError::Config { backtrace, .. } => Some(backtrace),
            DecrustError::Timeout { backtrace, .. } => Some(backtrace),
            DecrustError::Network { backtrace, .. } => Some(backtrace),
            DecrustError::NotFound { backtrace, .. } => Some(backtrace),
            DecrustError::Internal { backtrace, .. } => Some(backtrace),
            DecrustError::Validation { backtrace, .. } => Some(backtrace),
            DecrustError::Concurrency { backtrace, .. } => Some(backtrace),
            DecrustError::MissingValue { backtrace, .. } => Some(backtrace),
            DecrustError::StateConflict { backtrace, .. } => Some(backtrace),
            DecrustError::MultipleErrors { backtrace, .. } => Some(backtrace),
            DecrustError::ExternalService { backtrace, .. } => Some(backtrace),
            DecrustError::ResourceExhausted { backtrace, .. } => Some(backtrace),
            DecrustError::CircuitBreakerOpen { backtrace, .. } => Some(backtrace),
            DecrustError::WithRichContext { source, .. } => source.backtrace(),
        }
    }
}

impl OptionalError {
    /// Creates a new OptionalError from an optional boxed error
    ///
    /// # Parameters
    /// * `opt` - An optional boxed error trait object
    pub fn new(opt: Option<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        OptionalError(opt)
    }

    /// Checks if this OptionalError contains an actual error
    ///
    /// # Returns
    /// `true` if there is an error, `false` otherwise
    pub fn has_error(&self) -> bool {
        self.0.is_some()
    }
}

impl From<Option<Box<dyn std::error::Error + Send + Sync + 'static>>> for OptionalError {
    fn from(opt: Option<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        OptionalError(opt)
    }
}

impl AsRef<Option<Box<dyn std::error::Error + Send + Sync + 'static>>> for OptionalError {
    fn as_ref(&self) -> &Option<Box<dyn std::error::Error + Send + Sync + 'static>> {
        &self.0
    }
}

/// Unified error type for Decrust.
#[derive(Debug)]
#[allow(clippy::result_large_err)]
pub enum DecrustError {
    /// I/O related errors
    Io {
        /// The underlying I/O error
        source: std::io::Error,
        /// Optional path to the file or resource that caused the error
        path: Option<PathBuf>,
        /// Description of the operation that failed
        operation: String,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Parsing errors (JSON, YAML, etc.)
    Parse {
        /// The underlying parsing error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        /// The type of data being parsed (e.g., "JSON", "YAML")
        kind: String,
        /// Additional context information about the parsing operation
        context_info: String,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Network related errors
    Network {
        /// The underlying network error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        /// Optional URL that was being accessed
        url: Option<String>,
        /// The type of network operation (e.g., "HTTP", "TCP")
        kind: String,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Configuration related errors
    Config {
        /// Error message describing the configuration issue
        message: String,
        /// Optional path to the configuration file
        path: Option<PathBuf>,
        /// Optional underlying error that caused the configuration issue
        source: OptionalError,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Validation errors
    Validation {
        /// Name of the field that failed validation
        field: String,
        /// Description of the validation error
        message: String,
        /// Expected value or format (for backwards compatibility, optional)
        #[doc(hidden)]
        expected: Option<String>,
        /// Actual value that was provided (for backwards compatibility, optional)
        #[doc(hidden)]
        actual: Option<String>,
        /// Validation rule that was violated (for backwards compatibility, optional)
        #[doc(hidden)]
        rule: Option<String>,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Internal errors
    Internal {
        /// Description of the internal error
        message: String,
        /// Optional underlying error
        source: OptionalError,
        /// Component that generated the error (for backwards compatibility, optional)
        #[doc(hidden)]
        component: Option<String>,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Circuit breaker is open
    CircuitBreakerOpen {
        /// Name of the circuit breaker
        name: String,
        /// Optional duration after which the circuit breaker might transition to half-open
        retry_after: Option<Duration>,
        /// Number of consecutive failures that caused the circuit to open (for backwards compatibility, optional)
        #[doc(hidden)]
        failure_count: Option<u32>,
        /// Last error message that contributed to opening the circuit (for backwards compatibility, optional)
        #[doc(hidden)]
        last_error: Option<String>,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Operation timed out
    Timeout {
        /// Name of the operation that timed out
        operation: String,
        /// Duration after which the operation timed out
        duration: Duration,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Resource exhaustion
    ResourceExhausted {
        /// Name of the resource that was exhausted
        resource: String,
        /// The limit of the resource
        limit: String,
        /// The current value that exceeded the limit
        current: String,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Resource not found
    NotFound {
        /// Type of resource that was not found (e.g., "User", "File")
        resource_type: String,
        /// Identifier of the resource that was not found
        identifier: String,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// State conflict
    StateConflict {
        /// Description of the state conflict
        message: String,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Concurrency related errors
    Concurrency {
        /// Description of the concurrency error
        message: String,
        /// Optional underlying error
        source: OptionalError,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// External service errors
    ExternalService {
        /// Name of the external service that caused the error
        service_name: String,
        /// Description of the error from the external service
        message: String,
        /// Optional underlying error from the external service
        source: OptionalError,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Missing value errors
    MissingValue {
        /// Description of the missing value or item
        item_description: String,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Multiple errors
    MultipleErrors {
        /// Collection of errors that occurred
        errors: Vec<DecrustError>,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// Error with rich context
    WithRichContext {
        /// Rich context information attached to the error
        context: types::ErrorContext,
        /// The original error that is being wrapped with context
        source: Box<DecrustError>,
    },

    /// Style and formatting errors
    Style {
        /// The style error message
        message: String,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },

    /// General purpose error wrapper
    Oops {
        /// Custom error message
        message: String,
        /// The underlying error being wrapped
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        /// Backtrace captured at the error site
        backtrace: Backtrace,
    },
}

impl Clone for DecrustError {
    fn clone(&self) -> Self {
        match self {
            Self::Io {
                source,
                path,
                operation,
                ..
            } => Self::Io {
                source: std::io::Error::new(source.kind(), format!("{}", source)),
                path: path.clone(),
                operation: operation.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::Parse {
                source,
                kind,
                context_info,
                ..
            } => Self::Parse {
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("{}", source),
                )),
                kind: kind.clone(),
                context_info: context_info.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::Network {
                source, url, kind, ..
            } => Self::Network {
                source: Box::new(std::io::Error::other(format!("{}", source))),
                url: url.clone(),
                kind: kind.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::Config {
                message,
                path,
                source,
                ..
            } => Self::Config {
                message: message.clone(),
                path: path.clone(),
                source: source.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::Validation {
                field,
                message,
                expected,
                actual,
                rule,
                ..
            } => Self::Validation {
                field: field.clone(),
                message: message.clone(),
                expected: expected.clone(),
                actual: actual.clone(),
                rule: rule.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::Internal {
                message,
                source,
                component,
                ..
            } => Self::Internal {
                message: message.clone(),
                source: source.clone(),
                component: component.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::CircuitBreakerOpen {
                name,
                retry_after,
                failure_count,
                last_error,
                ..
            } => Self::CircuitBreakerOpen {
                name: name.clone(),
                retry_after: *retry_after,
                failure_count: *failure_count,
                last_error: last_error.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::Timeout {
                operation,
                duration,
                ..
            } => Self::Timeout {
                operation: operation.clone(),
                duration: *duration,
                backtrace: Backtrace::generate(),
            },
            Self::ResourceExhausted {
                resource,
                limit,
                current,
                ..
            } => Self::ResourceExhausted {
                resource: resource.clone(),
                limit: limit.clone(),
                current: current.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::NotFound {
                resource_type,
                identifier,
                ..
            } => Self::NotFound {
                resource_type: resource_type.clone(),
                identifier: identifier.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::StateConflict { message, .. } => Self::StateConflict {
                message: message.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::Concurrency {
                message, source, ..
            } => Self::Concurrency {
                message: message.clone(),
                source: source.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::ExternalService {
                service_name,
                message,
                source,
                ..
            } => Self::ExternalService {
                service_name: service_name.clone(),
                message: message.clone(),
                source: source.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::MissingValue {
                item_description, ..
            } => Self::MissingValue {
                item_description: item_description.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::MultipleErrors { errors, .. } => Self::MultipleErrors {
                errors: errors.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::WithRichContext { context, source } => {
                // Explicitly list fields, no 'backtrace' field here
                Self::WithRichContext {
                    context: context.clone(),
                    source: Box::new((**source).clone()),
                }
            }
            Self::Style { message, .. } => Self::Style {
                message: message.clone(),
                backtrace: Backtrace::generate(),
            },
            Self::Oops {
                message, source, ..
            } => Self::Oops {
                message: message.clone(),
                source: Box::new(std::io::Error::other(format!("{}", source))),
                backtrace: Backtrace::generate(),
            },
        }
    }
}

impl std::fmt::Display for DecrustError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecrustError::Io {
                source,
                path,
                operation,
                ..
            } => {
                write!(
                    f,
                    "I/O error during operation '{}' on path '{}': {}",
                    operation,
                    path.as_ref()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                    source
                )
            }
            DecrustError::Parse {
                source,
                kind,
                context_info,
                ..
            } => {
                write!(f, "{} parsing error: {} ({})", kind, source, context_info)
            }
            DecrustError::Network {
                source, url, kind, ..
            } => {
                write!(
                    f,
                    "{} network error: {} (URL: {})",
                    kind,
                    source,
                    url.as_deref().unwrap_or("N/A")
                )
            }
            DecrustError::Config {
                message,
                path,
                source,
                ..
            } => {
                if let Some(p) = path {
                    if let Some(s) = &source.0 {
                        write!(
                            f,
                            "Configuration error in '{}': {} ({})",
                            p.display(),
                            message,
                            s
                        )
                    } else {
                        write!(f, "Configuration error in '{}': {}", p.display(), message)
                    }
                } else if let Some(s) = &source.0 {
                    write!(f, "Configuration error: {} ({})", message, s)
                } else {
                    write!(f, "Configuration error: {}", message)
                }
            }
            DecrustError::Validation { field, message, .. } => {
                write!(f, "Validation error for '{}': {}", field, message)
            }
            DecrustError::Internal {
                message, source, ..
            } => {
                if let Some(s) = &source.0 {
                    write!(f, "Internal error: {} ({})", message, s)
                } else {
                    write!(f, "Internal error: {}", message)
                }
            }
            DecrustError::CircuitBreakerOpen {
                name, retry_after, ..
            } => {
                if let Some(duration) = retry_after {
                    write!(
                        f,
                        "Circuit breaker '{}' is open. Retry after {:?}",
                        name, duration
                    )
                } else {
                    write!(f, "Circuit breaker '{}' is open", name)
                }
            }
            DecrustError::Timeout {
                operation,
                duration,
                ..
            } => {
                write!(
                    f,
                    "Operation '{}' timed out after {:?}",
                    operation, duration
                )
            }
            DecrustError::ResourceExhausted {
                resource,
                limit,
                current,
                ..
            } => {
                write!(
                    f,
                    "Resource '{}' exhausted: {} (limit: {})",
                    resource, current, limit
                )
            }
            DecrustError::NotFound {
                resource_type,
                identifier,
                ..
            } => {
                write!(f, "{} not found: {}", resource_type, identifier)
            }
            DecrustError::StateConflict { message, .. } => {
                write!(f, "State conflict: {}", message)
            }
            DecrustError::Concurrency {
                message, source, ..
            } => {
                if let Some(s) = &source.0 {
                    write!(f, "Concurrency error: {} ({})", message, s)
                } else {
                    write!(f, "Concurrency error: {}", message)
                }
            }
            DecrustError::ExternalService {
                service_name,
                message,
                source,
                ..
            } => {
                if let Some(s) = &source.0 {
                    write!(
                        f,
                        "External service '{}' error: {} ({})",
                        service_name, message, s
                    )
                } else {
                    write!(f, "External service '{}' error: {}", service_name, message)
                }
            }
            DecrustError::MissingValue {
                item_description, ..
            } => {
                write!(f, "Missing value: {}", item_description)
            }
            DecrustError::MultipleErrors { errors, .. } => {
                write!(f, "Multiple errors ({} total):", errors.len())?;
                for (i, err) in errors.iter().enumerate() {
                    write!(f, "\n  {}. {}", i + 1, err)?;
                }
                Ok(())
            }
            DecrustError::WithRichContext {
                context, source, ..
            } => {
                write!(f, "{}: {}", context.message, source)
            }
            DecrustError::Style { message, .. } => {
                write!(f, "Style issue: {}", message)
            }
            DecrustError::Oops {
                message, source, ..
            } => {
                write!(f, "{}: {}", message, source)
            }
        }
    }
}

impl DecrustError {
    /// Adds rich context to an error
    ///
    /// This wraps the error in a WithRichContext variant, which allows for additional
    /// information to be attached to the error.
    ///
    /// # Parameters
    /// * `context` - The error context to add
    ///
    /// # Returns
    /// A new error with the context attached
    pub fn add_context(self, context: types::ErrorContext) -> Self {
        // Create the WithRichContext variant directly
        DecrustError::WithRichContext {
            context,
            source: Box::new(self),
        }
    }

    /// Adds a simple message context to an error
    ///
    /// This is a convenience method that creates a simple ErrorContext with just a message
    /// and adds it to the error.
    ///
    /// # Parameters
    /// * `message` - The message to add as context
    ///
    /// # Returns
    /// A new error with the context attached
    pub fn add_context_msg(self, message: impl Into<String>) -> Self {
        let error_context = types::ErrorContext::new(message);
        self.add_context(error_context)
    }

    /// Gets the category of this error
    ///
    /// # Returns
    /// The ErrorCategory that best describes this error
    pub fn category(&self) -> types::ErrorCategory {
        match self {
            DecrustError::Io { .. } => types::ErrorCategory::Io,
            DecrustError::Parse { .. } => types::ErrorCategory::Parsing,
            DecrustError::Network { .. } => types::ErrorCategory::Network,
            DecrustError::Config { .. } => types::ErrorCategory::Configuration,
            DecrustError::Validation { .. } => types::ErrorCategory::Validation,
            DecrustError::Internal { .. } => types::ErrorCategory::Internal,
            DecrustError::CircuitBreakerOpen { .. } => types::ErrorCategory::CircuitBreaker,
            DecrustError::Timeout { .. } => types::ErrorCategory::Timeout,
            DecrustError::ResourceExhausted { .. } => types::ErrorCategory::ResourceExhaustion,
            DecrustError::NotFound { .. } => types::ErrorCategory::NotFound,
            DecrustError::StateConflict { .. } => types::ErrorCategory::StateConflict,
            DecrustError::Concurrency { .. } => types::ErrorCategory::Concurrency,
            DecrustError::ExternalService { .. } => types::ErrorCategory::ExternalService,
            DecrustError::MultipleErrors { .. } => types::ErrorCategory::Multiple,
            DecrustError::WithRichContext { source, .. } => source.category(),
            DecrustError::Style { .. } => types::ErrorCategory::Style,
            DecrustError::Oops { .. } => types::ErrorCategory::Unspecified,
            DecrustError::MissingValue { .. } => types::ErrorCategory::Validation,
        }
    }

    /// Gets the severity of this error
    ///
    /// # Returns
    /// The ErrorSeverity level of this error
    pub fn severity(&self) -> types::ErrorSeverity {
        if let DecrustError::WithRichContext { context, .. } = self {
            context.severity
        } else {
            types::ErrorSeverity::Error
        }
    }

    /// Gets the rich context attached to this error, if any
    ///
    /// # Returns
    /// Some(context) if this is a WithRichContext error, None otherwise
    pub fn get_rich_context(&self) -> Option<&types::ErrorContext> {
        match self {
            DecrustError::WithRichContext { context, .. } => Some(context),
            _ => None,
        }
    }
}

/// Extension trait for Result types to add context to errors
///
/// This trait provides methods to add context to errors in a Result,
/// making it easier to provide additional information about the error.
///
/// This trait is object-safe and can be used with dynamic dispatch.
pub trait DecrustResultExt<T, EOrig> {
    /// Adds a simple message context to an error in a Result
    ///
    /// # Parameters
    /// * `message` - The message to add as context
    ///
    /// # Returns
    /// A new Result with the error wrapped in a WithRichContext variant if it was an error
    fn decrust_context_msg(self, message: &str) -> Result<T, DecrustError>;

    /// Adds a simple message context to an error in a Result (owned string version)
    ///
    /// # Parameters
    /// * `message` - The message to add as context
    ///
    /// # Returns
    /// A new Result with the error wrapped in a WithRichContext variant if it was an error
    fn decrust_context_msg_owned(self, message: String) -> Result<T, DecrustError>;

    /// Adds rich context to an error in a Result
    ///
    /// # Parameters
    /// * `context` - The error context to add
    ///
    /// # Returns
    /// A new Result with the error wrapped in a WithRichContext variant if it was an error
    fn decrust_context_rich(self, context: types::ErrorContext) -> Result<T, DecrustError>;
}

impl<T, E> DecrustResultExt<T, E> for std::result::Result<T, E>
where
    E: Into<DecrustError>,
{
    #[track_caller]
    fn decrust_context_msg(self, message: &str) -> Result<T, DecrustError> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                let decrust_err: DecrustError = err.into();
                Err(DecrustError::WithRichContext {
                    context: types::ErrorContext::new(message),
                    source: Box::new(decrust_err),
                })
            }
        }
    }

    #[track_caller]
    fn decrust_context_msg_owned(self, message: String) -> Result<T, DecrustError> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                let decrust_err: DecrustError = err.into();
                Err(DecrustError::WithRichContext {
                    context: types::ErrorContext::new(message),
                    source: Box::new(decrust_err),
                })
            }
        }
    }

    #[track_caller]
    fn decrust_context_rich(self, context: types::ErrorContext) -> Result<T, DecrustError> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                let decrust_err: DecrustError = err.into();
                Err(DecrustError::WithRichContext {
                    context,
                    source: Box::new(decrust_err),
                })
            }
        }
    }
}

/// Extension trait for Option types to convert to Result with DecrustError
///
/// This trait provides methods to convert an Option to a Result, with a MissingValue
/// error if the Option is None.
///
/// This trait is object-safe and can be used with dynamic dispatch.
pub trait DecrustOptionExt<T> {
    /// Converts an Option to a Result, with a MissingValue error if None
    ///
    /// # Parameters
    /// * `item_description` - Description of the missing value for the error message
    ///
    /// # Returns
    /// Ok(value) if the Option is Some(value), Err(DecrustError::MissingValue) otherwise
    fn decrust_ok_or_missing_value(self, item_description: &str) -> Result<T, DecrustError>;

    /// Converts an Option to a Result, with a MissingValue error if None (owned string version)
    ///
    /// # Parameters
    /// * `item_description` - Description of the missing value for the error message
    ///
    /// # Returns
    /// Ok(value) if the Option is Some(value), Err(DecrustError::MissingValue) otherwise
    fn decrust_ok_or_missing_value_owned(self, item_description: String)
        -> Result<T, DecrustError>;
}

impl<T> DecrustOptionExt<T> for Option<T> {
    #[track_caller]
    fn decrust_ok_or_missing_value(self, item_description: &str) -> Result<T, DecrustError> {
        match self {
            Some(v) => Ok(v),
            None => Err(DecrustError::MissingValue {
                item_description: item_description.to_string(),
                backtrace: Backtrace::generate(),
            }),
        }
    }

    #[track_caller]
    fn decrust_ok_or_missing_value_owned(
        self,
        item_description: String,
    ) -> Result<T, DecrustError> {
        match self {
            Some(v) => Ok(v),
            None => Err(DecrustError::MissingValue {
                item_description,
                backtrace: Backtrace::generate(),
            }),
        }
    }
}

/// Extension trait for Results that are known to always be Err
pub trait InfallibleResultExt<E> {
    /// Extract the error value from a Result that is known to always be Err
    ///
    /// This is a stable alternative to the nightly-only `into_err()` method.
    /// Use this when you have a Result<T, E> where T is an uninhabited type
    /// or when you know the Result will always be Err.
    fn extract_err(self) -> E;
}

impl<E> InfallibleResultExt<E> for Result<std::convert::Infallible, E> {
    fn extract_err(self) -> E {
        match self {
            Ok(infallible) => match infallible {},
            Err(e) => e,
        }
    }
}

/// Convenience trait for backward compatibility with generic string types
///
/// **Note:** This trait is NOT object-safe due to the use of `impl Into<String>`.
/// Use `DecrustResultExt` for object-safe operations.
pub trait DecrustResultExtConvenience<T, EOrig> {
    /// Convenience method for adding context with any string-like type
    ///
    /// **Warning:** This method makes the trait NOT object-safe.
    fn decrust_context<S: Into<String>>(self, message: S) -> Result<T, DecrustError>;
}

impl<T, E> DecrustResultExtConvenience<T, E> for std::result::Result<T, E>
where
    E: Into<DecrustError>,
{
    fn decrust_context<S: Into<String>>(self, message: S) -> Result<T, DecrustError> {
        self.decrust_context_msg_owned(message.into())
    }
}

/// Convenience trait for backward compatibility with generic string types
///
/// **Note:** This trait is NOT object-safe due to the use of `impl Into<String>`.
/// Use `DecrustOptionExt` for object-safe operations.
pub trait DecrustOptionExtConvenience<T> {
    /// Convenience method for converting to Result with any string-like type
    ///
    /// **Warning:** This method makes the trait NOT object-safe.
    fn decrust_ok_or_missing<S: Into<String>>(self, item_description: S)
        -> Result<T, DecrustError>;
}

impl<T> DecrustOptionExtConvenience<T> for Option<T> {
    fn decrust_ok_or_missing<S: Into<String>>(
        self,
        item_description: S,
    ) -> Result<T, DecrustError> {
        self.decrust_ok_or_missing_value_owned(item_description.into())
    }
}

/// Implementation of From<std::io::Error> for DecrustError to support extension traits
impl From<std::io::Error> for DecrustError {
    fn from(err: std::io::Error) -> Self {
        DecrustError::Io {
            source: err,
            path: None,
            operation: "I/O operation".to_string(),
            backtrace: Backtrace::generate(),
        }
    }
}

/// Implementation of From&lt;Box&lt;dyn std::error::Error&gt;&gt; for DecrustError to support generic error handling
impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for DecrustError {
    fn from(err: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        DecrustError::Oops {
            message: "Generic error occurred".to_string(),
            source: err,
            backtrace: Backtrace::generate(),
        }
    }
}

/// Implementation of From&lt;Box&lt;dyn std::error::Error&gt;&gt; for DecrustError (non-Send+Sync version)
impl From<Box<dyn std::error::Error>> for DecrustError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        // Convert to Send+Sync version by creating a new error with the message
        let message = format!("Generic error occurred: {}", err);
        DecrustError::Internal {
            message,
            source: OptionalError(None), // Can't store non-Send+Sync error
            component: None,
            backtrace: Backtrace::generate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backtrace::BacktraceCompat; // Ensure BacktraceCompat is in scope for tests
                                    // GenerateImplicitData is not needed in tests unless you call Backtrace::generate() directly.

    #[test]
    fn test_error_creation_and_context() {
        let source_opt: Option<Box<dyn std::error::Error + Send + Sync + 'static>> = None;
        // Create the error directly
        let err = DecrustError::Internal {
            message: "Test error".to_string(),
            source: OptionalError(source_opt),
            component: None,
            backtrace: Backtrace::generate(),
        };

        assert_eq!(err.category(), types::ErrorCategory::Internal);

        // Create a Result with the error and use the extension trait
        let err_with_context_res: Result<(), DecrustError> =
            Err(err).decrust_context_msg("Additional context");
        assert!(err_with_context_res.is_err());
        let err_with_context = err_with_context_res.unwrap_err();

        if let DecrustError::WithRichContext {
            context, source, ..
        } = &err_with_context
        {
            assert_eq!(context.message, "Additional context");
            // source is &Box<DecrustError>, so we need to dereference it properly
            if let DecrustError::Internal { message, .. } = source.as_ref() {
                assert_eq!(message, "Test error");
            } else {
                panic!("Expected Internal error variant, got {:?}", source);
            }
        } else {
            panic!(
                "Expected WithRichContext error variant, got {:?}",
                err_with_context
            );
        }
    }

    #[test]
    fn test_error_clone() {
        let io_err_orig = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let original_err = DecrustError::Io {
            source: io_err_orig,
            path: Some(PathBuf::from("/path/to/file")),
            operation: "read_file".to_string(),
            backtrace: Backtrace::generate(),
        };

        let cloned_err = original_err.clone();

        assert_eq!(cloned_err.category(), types::ErrorCategory::Io);

        // Use `ref` for non-Copy fields in pattern to avoid moving
        if let DecrustError::Io {
            ref path,
            ref operation,
            ref source,
            ..
        } = cloned_err
        {
            assert_eq!(*path, Some(PathBuf::from("/path/to/file")));
            assert_eq!(*operation, "read_file");
            assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
        } else {
            panic!("Expected Io error variant");
        }
        assert!(BacktraceCompat::backtrace(&cloned_err).is_some());
    }

    #[test]
    fn test_option_ext() {
        let opt_value: Option<i32> = Some(42);
        let result = opt_value.decrust_ok_or_missing_value("test value");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        let opt_none: Option<i32> = None;
        let result = opt_none.decrust_ok_or_missing_value("test value");
        assert!(result.is_err());

        if let Err(DecrustError::MissingValue {
            item_description, ..
        }) = result
        {
            assert_eq!(item_description, "test value");
        } else {
            panic!("Expected MissingValue error variant");
        }

        // Test the owned version
        let opt_none2: Option<i32> = None;
        let result2 = opt_none2.decrust_ok_or_missing_value_owned("owned test value".to_string());
        assert!(result2.is_err());

        if let Err(DecrustError::MissingValue {
            item_description, ..
        }) = result2
        {
            assert_eq!(item_description, "owned test value");
        } else {
            panic!("Expected MissingValue error variant");
        }

        // Test the convenience method
        let opt_none3: Option<i32> = None;
        let result3 = opt_none3.decrust_ok_or_missing("convenience test value");
        assert!(result3.is_err());

        if let Err(DecrustError::MissingValue {
            item_description, ..
        }) = result3
        {
            assert_eq!(item_description, "convenience test value");
        } else {
            panic!("Expected MissingValue error variant");
        }
    }

    #[test]
    fn test_object_safety() {
        // Test that the main traits are object-safe (dyn-compatible)
        let result: Result<i32, DecrustError> = Ok(42);
        let option: Option<i32> = Some(42);

        // These should compile without errors, proving the traits are object-safe
        let _result_trait: &dyn DecrustResultExt<i32, DecrustError> = &result;
        let _option_trait: &dyn DecrustOptionExt<i32> = &option;

        // Test that we can actually use the object-safe methods
        fn use_dyn_result_trait(_r: &dyn DecrustResultExt<i32, DecrustError>) {
            // This function signature proves the trait is object-safe
        }

        fn use_dyn_option_trait(_o: &dyn DecrustOptionExt<i32>) {
            // This function signature proves the trait is object-safe
        }

        use_dyn_result_trait(&result);
        use_dyn_option_trait(&option);

        assert!(true);
    }

    #[test]
    fn test_infallible_result_ext() {
        // Test the stable alternative to nightly-only into_err()
        fn always_fails() -> Result<std::convert::Infallible, String> {
            Err("This always fails".to_string())
        }

        let error: String = always_fails().extract_err();
        assert_eq!(error, "This always fails");

        // Test with DecrustError
        fn always_fails_decrust() -> Result<std::convert::Infallible, DecrustError> {
            Err(DecrustError::Oops {
                message: "Test oops error".to_string(),
                source: Box::new(std::io::Error::other("test")),
                backtrace: Backtrace::generate(),
            })
        }

        let error: DecrustError = always_fails_decrust().extract_err();
        if let DecrustError::Oops { message, .. } = error {
            assert_eq!(message, "Test oops error");
        } else {
            panic!("Expected Oops error variant");
        }
    }

    #[test]
    fn test_multiple_errors() {
        // Create validation errors directly
        let err1 = DecrustError::Validation {
            field: "username".to_string(),
            message: "Username too short".to_string(),
            expected: None,
            actual: None,
            rule: None,
            backtrace: Backtrace::generate(),
        };

        let err2 = DecrustError::Validation {
            field: "password".to_string(),
            message: "Password too weak".to_string(),
            expected: None,
            actual: None,
            rule: None,
            backtrace: Backtrace::generate(),
        };

        // Create multiple errors directly
        let multi_err = DecrustError::MultipleErrors {
            errors: vec![err1, err2.clone()],
            backtrace: Backtrace::generate(),
        };

        if let DecrustError::MultipleErrors { errors, .. } = multi_err {
            assert_eq!(errors.len(), 2);
            if let DecrustError::Validation { field, .. } = &errors[0] {
                assert_eq!(field, "username");
            } else {
                panic!("Expected Validation error variant for errors[0]");
            }
            if let DecrustError::Validation { field, .. } = &errors[1] {
                assert_eq!(field, "password");
            } else {
                panic!("Expected Validation error variant for errors[1]");
            }
        } else {
            panic!("Expected MultipleErrors error variant");
        }
    }

    #[test]
    fn test_whatever_error() {
        let original_io_error = std::io::Error::other("some io problem");
        // Create a Oops variant directly
        let err = DecrustError::Oops {
            message: "A oops message".to_string(),
            source: Box::new(original_io_error)
                as Box<dyn std::error::Error + Send + Sync + 'static>,
            backtrace: Backtrace::generate(),
        };

        if let DecrustError::Oops {
            message, source, ..
        } = err
        {
            // Use .. for backtrace if not asserted
            assert_eq!(message, "A oops message");
            assert_eq!(source.to_string(), "some io problem");
        } else {
            panic!("Expected Oops error variant");
        }
    }

    #[test]
    fn test_io_error_display() {
        let path_buf = PathBuf::from("/my/file.txt");
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "original os error");
        let ak_err = DecrustError::Io {
            source: io_err,
            path: Some(path_buf),
            operation: "reading".to_string(),
            backtrace: Backtrace::generate(),
        };
        assert_eq!(
            ak_err.to_string(),
            "I/O error during operation 'reading' on path '/my/file.txt': original os error"
        );
    }

    #[test]
    fn test_io_error_display_no_path() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "original os error");
        let ak_err = DecrustError::Io {
            source: io_err,
            path: None,
            operation: "reading".to_string(),
            backtrace: Backtrace::generate(),
        };
        assert_eq!(
            ak_err.to_string(),
            "I/O error during operation 'reading' on path 'N/A': original os error"
        );
    }
}

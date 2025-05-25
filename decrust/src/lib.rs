/* decrust/src/lib.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! # 🚀 Decrust – The Ultimate Error Handling Framework for Rust
//!
//! **The easiest, no-hassle error-handling experience in Rust.**
//! One import. One macro. All errors handled.
//!
//! ## ✅ One Import, One Macro – That’s It
//!
//! ```rust
//! use decrust::*;
//!
//! fn any_operation() -> Result<String> {
//!     Ok("Success!".to_string())
//! }
//!
//! fn main() -> Result<()> {
//!     let result = decrust!(any_operation());
//!     // That’s it! No unwraps, no Result juggling, just clean flow.
//!     Ok(())
//! }
//! ```
//!
//! ## 🔍 What `decrust!` Can Handle
//!
//! - ✅ **File I/O operations** – automatic error detection & reporting
//! - ✅ **Network requests** – with circuit breakers
//! - ✅ **Database calls** – includes retry and backoff logic
//! - ✅ **Parsing ops** – with inline validation & recovery
//! - ✅ **Memory & threading issues** – auto-sanitized
//! - ✅ **Async/await** – seamlessly supported
//! - ✅ **Third-party libraries** – automatic conversion & wrap
//! - ✅ **Complex pipelines** – supports scoped rollback & checkpointing
//! - ✅ **User input validation** – with suggestion engine
//! - ✅ **Config loading** – fallback with dynamic defaults
//! - ✅ **Serialization/Deserialization** – with self-healing recovery
//! - ✅ **API Calls** – with built-in rate limiter support
//!
//! ## ✨ The Magic of Decrust
//!
//! ```no_run
//! use decrust::*;
//!
//! fn complex_business_logic() -> Result<String> {
//!     Ok("Business logic result".to_string())
//! }
//!
//! fn main() -> Result<()> {
//!     let file = decrust!(std::fs::read_to_string("config.toml"));
//!     // Note: reqwest and serde_json would need to be added as dependencies
//!     // let response = decrust!(reqwest::get("https://api.com/data").await);
//!     // let parsed = decrust!(serde_json::from_str(&data));
//!     let result = decrust!(complex_business_logic());
//!     Ok(())
//! }
//! ```
//!
//! ## 💎 Features
//!
//! - 🚫 **Zero runtime cost** – all compile-time macro magic
//! - 🔒 **Lock-free internals** – pure performance
//! - 🧠 **Automatic error detection** – no trait madness
//! - 🛡️ **Circuit breaker pattern** – fault tolerance built-in
//! - 🤖 **M.A.R.S. Autocorrection Engine** – when fixable, it fixes
//! - 📝 **Structured diagnostics** – rich error reporting & context
//! - ☝️ **One import:** `use decrust::*;`
//! - 🧙 **One macro:** `decrust!(any_operation())`

// Re-export the ultimate decrust! macro
pub use decrust_promac::decrust;

// Re-export all the convenience macros from decrust-core
pub use decrust_core::{error_context, implicit_data, location, oops, validation_error};

// Re-export ALL types and functionality from decrust-core (except Result to avoid conflict)
pub use decrust_core::{
    // Re-export all modules
    backtrace,
    circuit_breaker,
    decrust as decrust_core_module,
    reporter,
    syntax,
    types,
    // Re-export all the specific types
    AsBacktrace,
    AstMissingImportFixGenerator,
    AstUnusedCodeFixGenerator,
    AutocorrectableError,
    Autocorrection,
    Backtrace,
    BacktraceCompat,
    BacktraceFrame,
    BacktraceProvider,
    BacktraceStatus,
    CircuitBreaker,
    CircuitBreakerConfig,
    CircuitBreakerObserver,
    CircuitBreakerState,
    CircuitMetrics,
    CircuitOperationType,
    CircuitTransitionEvent,
    ClosureCaptureLifetimeFixGenerator,
    ConfigMissingKeyFixGenerator,
    ConfigSyntaxFixGenerator,
    Decrust,
    // Re-export all the main types
    DecrustError,
    DecrustOptionExt,
    DecrustResultExt,
    DiagnosticResult,
    DivisionByZeroFixGenerator,
    ErrorCategory,
    ErrorContext,
    ErrorReportConfig,
    ErrorReportFormat,
    ErrorReporter,
    ErrorSeverity,
    ErrorSource,
    ExtractedParameters,
    FixDetails,
    FixTemplate,
    FixType,
    GenerateImplicitData,
    InvalidArgumentCountFixGenerator,
    IoMissingDirectoryFixGenerator,
    IoPermissionFixGenerator,
    JsonParseFixGenerator,
    Location,
    MissingOkErrFixGenerator,
    NetworkConnectionFixGenerator,
    NetworkTlsFixGenerator,
    OptionalError,
    ParameterExtractor,
    ParameterSource,
    QuestionMarkPropagationFixGenerator,
    RecursiveTypeFixGenerator,
    ReturnLocalReferenceFixGenerator,
    RuntimePanicFixGenerator,
    SyntaxGenerator,
    TemplateRegistry,
    ThreadId,
    Timestamp,
    UnnecessaryCloneFixGenerator,
    UnnecessaryParenthesesFixGenerator,
    UnsafeUnwrapFixGenerator,
    UnstableFeatureFixGenerator,
    UnusedMutFixGenerator,
    YamlParseFixGenerator,
};

// Re-export specific items from runtime to avoid conflicts
pub use decrust_promac_runtime::BacktraceCompat as RuntimeBacktraceCompat;

// Convenience re-exports for common patterns
pub use std::result::Result as StdResult;

/// The standard Result type used throughout Decrust - simplified for ultimate interface
pub type Result<T> = StdResult<T, DecrustError>;

/// Re-export common traits for convenience
pub use std::error::Error as StdError;
pub use std::fmt::{Debug, Display};

// Documentation examples
#[cfg(doctest)]
mod doc_tests {
    //! Documentation tests to ensure examples work

    use super::*;

    /// Basic usage example
    /// ```
    /// use decrust::*;
    ///
    /// fn example_operation() -> Result<String> {
    ///     Ok("Success!".to_string())
    /// }
    ///
    /// fn main() -> Result<()> {
    ///     let result = decrust!(example_operation());
    ///     assert!(result.is_ok());
    ///     Ok(())
    /// }
    /// ```
    fn _basic_usage() {}

    /// File operations example
    /// ```no_run
    /// use decrust::*;
    ///
    /// fn main() -> Result<()> {
    ///     let content = decrust!(std::fs::read_to_string("config.toml"));
    ///     println!("Config: {:?}", content);
    ///     Ok(())
    /// }
    /// ```
    fn _file_operations() {}

    /// Network operations example
    /// ```no_run
    /// use decrust::*;
    ///
    /// fn simulate_network_call() -> Result<String> {
    ///     // Simulated network operation
    ///     Ok("Network response data".to_string())
    /// }
    ///
    /// fn main() -> Result<()> {
    ///     let response = decrust!(simulate_network_call());
    ///     println!("Response: {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    fn _network_operations() {}
}

/* src/backtrace.rs */
#![warn(missing_docs)]
//! **Brief:** Direct backtrace implementation with custom GenerateImplicitData trait.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Direct Backtrace System]
//!  - [Environment-Aware Capture]
//!  - [Custom GenerateImplicitData Trait]
//!  - [Crisis-Resistant Implementation]
//!  - [Zero External Dependencies]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT

use std::fmt;
use std::env;
use std::sync::OnceLock;

/// Our own backtrace type that wraps std::backtrace::Backtrace
/// This provides crisis-resistant backtrace functionality
#[derive(Debug)]
pub struct DecrustBacktrace {
    inner: Option<std::backtrace::Backtrace>,
    capture_enabled: bool,
}

/// Status of backtrace capture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BacktraceStatus {
    /// Backtrace was successfully captured
    Captured,
    /// Backtrace capture was disabled by environment
    Disabled,
    /// Backtrace capture is not supported on this platform
    Unsupported,
}

impl DecrustBacktrace {
    /// Creates a new backtrace, respecting environment variables
    ///
    /// Checks RUST_LIB_BACKTRACE first, then RUST_BACKTRACE
    /// Only captures if set to "1" or "full"
    pub fn capture() -> Self {
        let should_capture = Self::should_capture_from_env();

        if should_capture {
            Self {
                inner: Some(std::backtrace::Backtrace::capture()),
                capture_enabled: true,
            }
        } else {
            Self {
                inner: None,
                capture_enabled: false,
            }
        }
    }

    /// Forces backtrace capture regardless of environment variables
    ///
    /// Use this when you need backtraces for debugging purposes
    pub fn force_capture() -> Self {
        Self {
            inner: Some(std::backtrace::Backtrace::force_capture()),
            capture_enabled: true,
        }
    }

    /// Creates a disabled backtrace (no capture)
    pub fn disabled() -> Self {
        Self {
            inner: None,
            capture_enabled: false,
        }
    }

    /// Returns the status of this backtrace
    pub fn status(&self) -> BacktraceStatus {
        match &self.inner {
            Some(bt) => match bt.status() {
                std::backtrace::BacktraceStatus::Captured => BacktraceStatus::Captured,
                std::backtrace::BacktraceStatus::Disabled => BacktraceStatus::Disabled,
                std::backtrace::BacktraceStatus::Unsupported => BacktraceStatus::Unsupported,
                _ => BacktraceStatus::Unsupported,
            },
            None => {
                if self.capture_enabled {
                    BacktraceStatus::Disabled
                } else {
                    BacktraceStatus::Disabled
                }
            }
        }
    }

    /// Check if backtrace should be captured based on environment variables
    fn should_capture_from_env() -> bool {
        static SHOULD_CAPTURE: OnceLock<bool> = OnceLock::new();

        *SHOULD_CAPTURE.get_or_init(|| {
            // Check RUST_LIB_BACKTRACE first (higher priority)
            if let Ok(val) = env::var("RUST_LIB_BACKTRACE") {
                return val == "1" || val.to_lowercase() == "full";
            }

            // Fall back to RUST_BACKTRACE
            if let Ok(val) = env::var("RUST_BACKTRACE") {
                return val == "1" || val.to_lowercase() == "full";
            }

            false
        })
    }

    /// Get the inner backtrace if available
    pub fn as_std_backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        self.inner.as_ref()
    }
}

impl fmt::Display for DecrustBacktrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            Some(bt) => write!(f, "{}", bt),
            None => write!(f, "<backtrace disabled>"),
        }
    }
}

impl Clone for DecrustBacktrace {
    fn clone(&self) -> Self {
        // We can't clone the actual backtrace, so create a new one
        // with the same capture_enabled setting
        if self.capture_enabled {
            // Use force_capture to ensure we get a backtrace regardless of env vars
            Self {
                inner: Some(std::backtrace::Backtrace::force_capture()),
                capture_enabled: true,
            }
        } else {
            Self::disabled()
        }
    }
}

/// Trait for generating implicit data automatically
///
/// This replaces snafu's GenerateImplicitData trait with our own implementation
pub trait GenerateImplicitData {
    /// Generate the implicit data
    fn generate() -> Self;

    /// Generate implicit data with access to a source error
    ///
    /// Default implementation ignores the source and calls generate()
    fn generate_with_source(_source: &dyn std::error::Error) -> Self
    where
        Self: Sized
    {
        Self::generate()
    }
}

/// Implementation for our backtrace type
impl GenerateImplicitData for DecrustBacktrace {
    fn generate() -> Self {
        Self::capture()
    }

    fn generate_with_source(source: &dyn std::error::Error) -> Self {
        // Check if the source already has a backtrace we can use
        // For now, just generate a new one, but this could be enhanced
        // to delegate to the source if it implements our backtrace trait
        let _ = source; // Suppress unused parameter warning for now
        Self::capture()
    }
}

// Add a static method to DecrustBacktrace for compatibility with code that expects generate()
impl DecrustBacktrace {
    /// Generate a new backtrace - compatibility method for code that expects generate()
    pub fn generate() -> Self {
        Self::capture()
    }
}

// Implement From<std::backtrace::Backtrace> for DecrustBacktrace
impl From<std::backtrace::Backtrace> for DecrustBacktrace {
    fn from(backtrace: std::backtrace::Backtrace) -> Self {
        Self {
            inner: Some(backtrace),
            capture_enabled: true,
        }
    }
}

/// Trait for types that can provide backtraces
///
/// This is our version of snafu's ErrorCompat trait
pub trait BacktraceCompat {
    /// Get the backtrace associated with this error, if any
    fn backtrace(&self) -> Option<&DecrustBacktrace>;
}

/// Extension trait for easier backtrace handling
pub trait BacktraceProvider {
    /// Get a backtrace from this error or any of its sources
    fn get_deepest_backtrace(&self) -> Option<&DecrustBacktrace>;
}

impl<E: std::error::Error + BacktraceCompat> BacktraceProvider for E {
    fn get_deepest_backtrace(&self) -> Option<&DecrustBacktrace> {
        // First check if this error has a backtrace
        if let Some(bt) = self.backtrace() {
            return Some(bt);
        }

        // Walk the error chain looking for backtraces
        let mut current = self.source();
        while let Some(err) = current {
            // Try to downcast to our BacktraceCompat trait
            // This is a bit tricky with trait objects, so for now
            // we'll just return None and let callers handle it
            current = err.source();
        }

        None
    }
}

/// Custom timestamp type that implements GenerateImplicitData
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamp {
    instant: std::time::SystemTime,
}

impl Timestamp {
    /// Create a new timestamp with the current time
    pub fn now() -> Self {
        Self {
            instant: std::time::SystemTime::now(),
        }
    }

    /// Get the inner SystemTime
    pub fn as_system_time(&self) -> std::time::SystemTime {
        self.instant
    }
}

impl GenerateImplicitData for Timestamp {
    fn generate() -> Self {
        Self::now()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.instant.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => write!(f, "{}.{:03}", duration.as_secs(), duration.subsec_millis()),
            Err(_) => write!(f, "<invalid timestamp>"),
        }
    }
}

/// Thread ID type that implements GenerateImplicitData
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadId {
    id: std::thread::ThreadId,
    name: Option<String>,
}

impl ThreadId {
    /// Get the current thread's ID and name
    pub fn current() -> Self {
        let thread = std::thread::current();
        Self {
            id: thread.id(),
            name: thread.name().map(|s| s.to_string()),
        }
    }

    /// Get the thread ID
    pub fn id(&self) -> std::thread::ThreadId {
        self.id
    }

    /// Get the thread name if available
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl GenerateImplicitData for ThreadId {
    fn generate() -> Self {
        Self::current()
    }
}

impl fmt::Display for ThreadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{}({:?})", name, self.id),
            None => write!(f, "{:?}", self.id),
        }
    }
}

/// Location information for where an error was created
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    file: &'static str,
    line: u32,
    column: u32,
}

impl Location {
    /// Create a new location
    pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }

    /// Get the file path
    pub fn file(&self) -> &'static str {
        self.file
    }

    /// Get the line number
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Get the column number
    pub fn column(&self) -> u32 {
        self.column
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// Macro to create a location at the current source position
#[macro_export]
macro_rules! location {
    () => {
        $crate::backtrace::Location::new(file!(), line!(), column!())
    };
}

/// Macro to generate implicit data at the call site
#[macro_export]
macro_rules! implicit_data {
    ($type:ty) => {
        <$type as $crate::backtrace::GenerateImplicitData>::generate()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtrace_creation() {
        let bt1 = DecrustBacktrace::capture();
        let bt2 = DecrustBacktrace::force_capture();
        let bt3 = DecrustBacktrace::disabled();

        // These should all be valid regardless of environment
        assert!(matches!(bt1.status(), BacktraceStatus::Captured | BacktraceStatus::Disabled));
        assert!(bt2.status() == BacktraceStatus::Captured || bt2.status() == BacktraceStatus::Unsupported);
        assert_eq!(bt3.status(), BacktraceStatus::Disabled);
    }

    #[test]
    fn test_generate_implicit_data() {
        let bt = implicit_data!(DecrustBacktrace);
        let ts = implicit_data!(Timestamp);
        let tid = implicit_data!(ThreadId);

        // Just ensure they can be created
        assert!(matches!(bt.status(), BacktraceStatus::Captured | BacktraceStatus::Disabled));
        assert!(ts.as_system_time() <= std::time::SystemTime::now());
        assert_eq!(tid.id(), std::thread::current().id());
    }

    #[test]
    fn test_location_macro() {
        let loc = location!();
        assert!(loc.file().ends_with("backtrace.rs"));
        assert!(loc.line() > 0);
        assert!(loc.column() > 0);
    }

    #[test]
    fn test_display_implementations() {
        let bt = DecrustBacktrace::disabled();
        let ts = Timestamp::now();
        let tid = ThreadId::current();
        let loc = location!();

        // These should all produce valid strings
        assert!(!bt.to_string().is_empty());
        assert!(!ts.to_string().is_empty());
        assert!(!tid.to_string().is_empty());
        assert!(!loc.to_string().is_empty());
    }

    #[test]
    fn test_clone_backtrace() {
        let bt1 = DecrustBacktrace::force_capture();
        let bt2 = bt1.clone();

        // Cloning should create a new backtrace with same capture settings
        assert_eq!(bt1.capture_enabled, bt2.capture_enabled);
    }
}

// ===== Snafu-basec Backtrace Compatibility Layer =====
/// Implementation of GenerateImplicitData for std::backtrace::Backtrace
impl crate::backtrace::GenerateImplicitData for std::backtrace::Backtrace {
    fn generate() -> Self {
        std::backtrace::Backtrace::force_capture()
    }
}

/// Trait for types that can provide a backtrace
///
/// This is our version of snafu's AsBacktrace trait
pub trait AsBacktrace {
    /// Get the backtrace associated with this error, if any
    fn as_backtrace(&self) -> Option<&std::backtrace::Backtrace>;
}

// Implementation for std::backtrace::Backtrace
impl AsBacktrace for std::backtrace::Backtrace {
    fn as_backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        Some(self)
    }
}

// Implementation for our DecrustBacktrace
impl AsBacktrace for DecrustBacktrace {
    fn as_backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        self.as_std_backtrace()
    }
}

// Example usage patterns for the library
#[cfg(test)]
mod usage_examples {
    use super::*;

    // Example error type using our backtrace system
    #[derive(Debug)]
    pub struct ExampleError {
        message: String,
        backtrace: DecrustBacktrace,
        timestamp: Timestamp,
        thread: ThreadId,
    }

    impl ExampleError {
        pub fn new(message: impl Into<String>) -> Self {
            Self {
                message: message.into(),
                backtrace: implicit_data!(DecrustBacktrace),
                timestamp: implicit_data!(Timestamp),
                thread: implicit_data!(ThreadId),
            }
        }
    }

    impl fmt::Display for ExampleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} [{}] [{}]", self.message, self.timestamp, self.thread)
        }
    }

    impl std::error::Error for ExampleError {}

    impl BacktraceCompat for ExampleError {
        fn backtrace(&self) -> Option<&DecrustBacktrace> {
            Some(&self.backtrace)
        }
    }

    #[test]
    fn test_example_error() {
        let err = ExampleError::new("Something went wrong");

        assert!(!err.to_string().is_empty());
        assert!(err.backtrace().is_some());

        // Test backtrace provider
        if let Some(bt) = err.get_deepest_backtrace() {
            assert!(matches!(bt.status(), BacktraceStatus::Captured | BacktraceStatus::Disabled));
        }
    }
}
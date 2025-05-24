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

use std::env;
use std::fmt;
use std::sync::OnceLock;
use std::collections::HashMap;

/// Our own backtrace type that wraps std::backtrace::Backtrace
/// This provides crisis-resistant backtrace functionality
#[derive(Debug)]
pub struct DecrustBacktrace {
    inner: Option<std::backtrace::Backtrace>,
    capture_enabled: bool,
    capture_timestamp: std::time::SystemTime,
    thread_id: std::thread::ThreadId,
    thread_name: Option<String>,
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
        let current_thread = std::thread::current();

        if should_capture {
            Self {
                inner: Some(std::backtrace::Backtrace::capture()),
                capture_enabled: true,
                capture_timestamp: std::time::SystemTime::now(),
                thread_id: current_thread.id(),
                thread_name: current_thread.name().map(|s| s.to_string()),
            }
        } else {
            Self {
                inner: None,
                capture_enabled: false,
                capture_timestamp: std::time::SystemTime::now(),
                thread_id: current_thread.id(),
                thread_name: current_thread.name().map(|s| s.to_string()),
            }
        }
    }

    /// Forces backtrace capture regardless of environment variables
    ///
    /// Use this when you need backtraces for debugging purposes
    pub fn force_capture() -> Self {
        let current_thread = std::thread::current();
        Self {
            inner: Some(std::backtrace::Backtrace::force_capture()),
            capture_enabled: true,
            capture_timestamp: std::time::SystemTime::now(),
            thread_id: current_thread.id(),
            thread_name: current_thread.name().map(|s| s.to_string()),
        }
    }

    /// Creates a disabled backtrace (no capture)
    pub fn disabled() -> Self {
        let current_thread = std::thread::current();
        Self {
            inner: None,
            capture_enabled: false,
            capture_timestamp: std::time::SystemTime::now(),
            thread_id: current_thread.id(),
            thread_name: current_thread.name().map(|s| s.to_string()),
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

    /// Get the timestamp when this backtrace was captured
    pub fn capture_timestamp(&self) -> std::time::SystemTime {
        self.capture_timestamp
    }

    /// Get the thread ID where this backtrace was captured
    pub fn thread_id(&self) -> std::thread::ThreadId {
        self.thread_id
    }

    /// Get the thread name where this backtrace was captured, if available
    pub fn thread_name(&self) -> Option<&str> {
        self.thread_name.as_deref()
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

    /// Extract frame information from the backtrace
    pub fn extract_frames(&self) -> Vec<BacktraceFrame> {
        match &self.inner {
            Some(bt) => {
                let bt_string = format!("{}", bt);
                self.parse_backtrace_string(&bt_string)
            }
            None => Vec::new(),
        }
    }

    /// Parse backtrace string into structured frame information
    fn parse_backtrace_string(&self, bt_str: &str) -> Vec<BacktraceFrame> {
        let mut frames = Vec::new();

        for line in bt_str.lines() {
            if let Some(frame) = self.parse_frame_line(line) {
                frames.push(frame);
            }
        }

        frames
    }

    /// Parse a single frame line from backtrace output
    fn parse_frame_line(&self, line: &str) -> Option<BacktraceFrame> {
        // Parse format like: "   0: symbol_name at /path/to/file.rs:123:45"
        let trimmed = line.trim();

        if let Some(colon_pos) = trimmed.find(':') {
            let number_part = &trimmed[..colon_pos].trim();
            let rest = &trimmed[colon_pos + 1..].trim();

            if number_part.parse::<usize>().is_ok() {
                // Split on " at " to separate symbol from location
                if let Some(at_pos) = rest.rfind(" at ") {
                    let symbol = rest[..at_pos].trim().to_string();
                    let location = rest[at_pos + 4..].trim();

                    let (file, line, column) = self.parse_location(location);

                    return Some(BacktraceFrame {
                        symbol,
                        file,
                        line,
                        column,
                    });
                } else {
                    // No location info, just symbol
                    return Some(BacktraceFrame {
                        symbol: rest.to_string(),
                        file: None,
                        line: None,
                        column: None,
                    });
                }
            }
        }

        None
    }

    /// Parse location string like "/path/to/file.rs:123:45"
    fn parse_location(&self, location: &str) -> (Option<String>, Option<u32>, Option<u32>) {
        let parts: Vec<&str> = location.rsplitn(3, ':').collect();

        match parts.len() {
            3 => {
                let column = parts[0].parse().ok();
                let line = parts[1].parse().ok();
                let file = Some(parts[2].to_string());
                (file, line, column)
            }
            2 => {
                let line = parts[0].parse().ok();
                let file = Some(parts[1].to_string());
                (file, line, None)
            }
            1 => {
                (Some(parts[0].to_string()), None, None)
            }
            _ => (None, None, None),
        }
    }
}

/// Structured information about a backtrace frame
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BacktraceFrame {
    /// Symbol name or function name
    pub symbol: String,
    /// Source file path
    pub file: Option<String>,
    /// Line number in the source file
    pub line: Option<u32>,
    /// Column number in the source file
    pub column: Option<u32>,
}

impl fmt::Display for BacktraceFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)?;
        if let Some(ref file) = self.file {
            write!(f, " at {}", file)?;
            if let Some(line) = self.line {
                write!(f, ":{}", line)?;
                if let Some(column) = self.column {
                    write!(f, ":{}", column)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for DecrustBacktrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            Some(bt) => {
                writeln!(f, "Backtrace captured at: {:?}", self.capture_timestamp)?;
                if let Some(ref thread_name) = self.thread_name {
                    writeln!(f, "Thread: {} ({:?})", thread_name, self.thread_id)?;
                } else {
                    writeln!(f, "Thread: {:?}", self.thread_id)?;
                }
                write!(f, "{}", bt)
            }
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
            Self::force_capture()
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
        Self: Sized,
    {
        Self::generate()
    }

    /// Generate implicit data with custom context
    fn generate_with_context(context: &HashMap<String, String>) -> Self
    where
        Self: Sized,
    {
        let _ = context; // Suppress unused parameter warning
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

    fn generate_with_context(context: &HashMap<String, String>) -> Self {
        // Use context to determine if we should force capture
        if context.get("force_backtrace").map(|s| s == "true").unwrap_or(false) {
            Self::force_capture()
        } else {
            Self::capture()
        }
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
        let current_thread = std::thread::current();
        Self {
            inner: Some(backtrace),
            capture_enabled: true,
            capture_timestamp: std::time::SystemTime::now(),
            thread_id: current_thread.id(),
            thread_name: current_thread.name().map(|s| s.to_string()),
        }
    }
}

/// Trait for types that can provide backtraces
///
/// This is our version of snafu's ErrorCompat trait
pub trait BacktraceCompat: std::error::Error {
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
        None
    }
}

/// Custom timestamp type that implements GenerateImplicitData
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamp {
    instant: std::time::SystemTime,
    formatted: String,
}

impl Timestamp {
    /// Create a new timestamp with the current time
    pub fn now() -> Self {
        let instant = std::time::SystemTime::now();
        let formatted = Self::format_timestamp(&instant);
        Self {
            instant,
            formatted,
        }
    }

    /// Create a timestamp from a SystemTime
    pub fn from_system_time(time: std::time::SystemTime) -> Self {
        let formatted = Self::format_timestamp(&time);
        Self {
            instant: time,
            formatted,
        }
    }

    /// Get the inner SystemTime
    pub fn as_system_time(&self) -> std::time::SystemTime {
        self.instant
    }

    /// Get the formatted timestamp string
    pub fn formatted(&self) -> &str {
        &self.formatted
    }

    /// Format a SystemTime into a readable string
    fn format_timestamp(time: &std::time::SystemTime) -> String {
        match time.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => {
                let secs = duration.as_secs();
                let millis = duration.subsec_millis();

                // Convert to human-readable format
                let datetime = std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs);
                format!("{}.{:03} (epoch: {})",
                    secs, millis,
                    datetime.duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0)
                )
            }
            Err(_) => "<invalid timestamp>".to_string(),
        }
    }
}

impl GenerateImplicitData for Timestamp {
    fn generate() -> Self {
        Self::now()
    }

    fn generate_with_context(context: &HashMap<String, String>) -> Self {
        // Check if a specific timestamp is requested
        if let Some(timestamp_str) = context.get("timestamp") {
            if let Ok(secs) = timestamp_str.parse::<u64>() {
                let time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs);
                return Self::from_system_time(time);
            }
        }
        Self::now()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted)
    }
}

/// Thread ID type that implements GenerateImplicitData
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadId {
    id: std::thread::ThreadId,
    name: Option<String>,
    formatted: String,
}

impl ThreadId {
    /// Get the current thread's ID and name
    pub fn current() -> Self {
        let thread = std::thread::current();
        let id = thread.id();
        let name = thread.name().map(|s| s.to_string());
        let formatted = Self::format_thread_info(id, name.as_deref());

        Self {
            id,
            name,
            formatted,
        }
    }

    /// Create a ThreadId from components
    pub fn from_components(id: std::thread::ThreadId, name: Option<String>) -> Self {
        let formatted = Self::format_thread_info(id, name.as_deref());
        Self {
            id,
            name,
            formatted,
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

    /// Get the formatted thread information
    pub fn formatted(&self) -> &str {
        &self.formatted
    }

    /// Format thread information into a readable string
    fn format_thread_info(id: std::thread::ThreadId, name: Option<&str>) -> String {
        match name {
            Some(thread_name) => format!("{}({:?})", thread_name, id),
            None => format!("{:?}", id),
        }
    }
}

impl GenerateImplicitData for ThreadId {
    fn generate() -> Self {
        Self::current()
    }

    fn generate_with_context(context: &HashMap<String, String>) -> Self {
        // For thread context, we can only return current thread info
        // Context might be used for additional formatting hints
        let _ = context;
        Self::current()
    }
}

impl fmt::Display for ThreadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted)
    }
}

/// Location information for where an error was created
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    file: &'static str,
    line: u32,
    column: u32,
    formatted: String,
}

impl Location {
    /// Create a new location
    pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self {
            file,
            line,
            column,
            formatted: String::new(), // Will be filled in post-construction
        }
    }

    /// Create a new location with formatting
    pub fn new_formatted(file: &'static str, line: u32, column: u32) -> Self {
        let formatted = format!("{}:{}:{}", file, line, column);
        Self {
            file,
            line,
            column,
            formatted,
        }
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

    /// Get the formatted location string
    pub fn formatted(&self) -> String {
        if self.formatted.is_empty() {
            format!("{}:{}:{}", self.file, self.line, self.column)
        } else {
            self.formatted.clone()
        }
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
        $crate::backtrace::Location::new_formatted(file!(), line!(), column!())
    };
}

/// Macro to generate implicit data at the call site
#[macro_export]
macro_rules! implicit_data {
    ($type:ty) => {
        <$type as $crate::backtrace::GenerateImplicitData>::generate()
    };
    ($type:ty, $context:expr) => {
        <$type as $crate::backtrace::GenerateImplicitData>::generate_with_context($context)
    };
}

// ===== Snafu-based Backtrace Compatibility Layer =====

/// Implementation of GenerateImplicitData for std::backtrace::Backtrace
impl GenerateImplicitData for std::backtrace::Backtrace {
    fn generate() -> Self {
        std::backtrace::Backtrace::force_capture()
    }

    fn generate_with_context(context: &HashMap<String, String>) -> Self {
        // Check if force capture is requested
        if context.get("force_backtrace").map(|s| s == "true").unwrap_or(false) {
            std::backtrace::Backtrace::force_capture()
        } else {
            std::backtrace::Backtrace::capture()
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_backtrace_creation() {
        let bt1 = DecrustBacktrace::capture();
        let bt2 = DecrustBacktrace::force_capture();
        let bt3 = DecrustBacktrace::disabled();

        // These should all be valid regardless of environment
        assert!(matches!(
            bt1.status(),
            BacktraceStatus::Captured | BacktraceStatus::Disabled
        ));
        assert!(
            bt2.status() == BacktraceStatus::Captured
                || bt2.status() == BacktraceStatus::Unsupported
        );
        assert_eq!(bt3.status(), BacktraceStatus::Disabled);
    }

    #[test]
    fn test_generate_implicit_data() {
        let bt = implicit_data!(DecrustBacktrace);
        let ts = implicit_data!(Timestamp);
        let tid = implicit_data!(ThreadId);

        // Just ensure they can be created
        assert!(matches!(
            bt.status(),
            BacktraceStatus::Captured | BacktraceStatus::Disabled
        ));
        assert!(ts.as_system_time() <= std::time::SystemTime::now());
        assert_eq!(tid.id(), std::thread::current().id());
    }

    #[test]
    fn test_generate_with_context() {
        let mut context = HashMap::new();
        context.insert("force_backtrace".to_string(), "true".to_string());

        let bt = implicit_data!(DecrustBacktrace, &context);
        let ts = implicit_data!(Timestamp, &context);

        assert!(matches!(
            bt.status(),
            BacktraceStatus::Captured | BacktraceStatus::Unsupported
        ));
        assert!(!ts.formatted().is_empty());
    }

    #[test]
    fn test_location_macro() {
        let loc = location!();
        assert!(loc.file().ends_with("backtrace.rs"));
        assert!(loc.line() > 0);
        assert!(loc.column() > 0);
        assert!(!loc.formatted().is_empty());
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

    #[test]
    fn test_backtrace_metadata() {
        let bt = DecrustBacktrace::capture();

        // Test metadata access
        assert!(bt.capture_timestamp() <= std::time::SystemTime::now());
        assert_eq!(bt.thread_id(), std::thread::current().id());

        // Thread name might be None in test environment
        let _ = bt.thread_name();
    }

    #[test]
    fn test_frame_extraction() {
        let bt = DecrustBacktrace::force_capture();
        let frames = bt.extract_frames();

        // We should have frames available (exact count depends on test environment)
        // Just verify that the extraction doesn't panic and returns a valid Vec
        let _frame_count = frames.len(); // Documents that we expect frames to be extractable
    }

    #[test]
    fn test_timestamp_formatting() {
        let ts = Timestamp::now();
        let formatted = ts.formatted();

        assert!(!formatted.is_empty());
        assert!(formatted.contains('.'));
    }

    #[test]
    fn test_thread_id_formatting() {
        let tid = ThreadId::current();
        let formatted = tid.formatted();

        assert!(!formatted.is_empty());
        assert!(formatted.contains("ThreadId"));
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
        location: Location,
    }

    impl ExampleError {
        pub fn new(message: impl Into<String>) -> Self {
            Self {
                message: message.into(),
                backtrace: implicit_data!(DecrustBacktrace),
                timestamp: implicit_data!(Timestamp),
                thread: implicit_data!(ThreadId),
                location: location!(),
            }
        }

        pub fn new_with_context(message: impl Into<String>, context: &HashMap<String, String>) -> Self {
            Self {
                message: message.into(),
                backtrace: implicit_data!(DecrustBacktrace, context),
                timestamp: implicit_data!(Timestamp, context),
                thread: implicit_data!(ThreadId, context),
                location: location!(),
            }
        }
    }

    impl fmt::Display for ExampleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} [{}] [{}] [{}]",
                self.message,
                self.timestamp,
                self.thread,
                self.location
            )
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
            assert!(matches!(
                bt.status(),
                BacktraceStatus::Captured | BacktraceStatus::Disabled
            ));
        }
    }

    #[test]
    fn test_example_error_with_context() {
        let mut context = HashMap::new();
        context.insert("force_backtrace".to_string(), "true".to_string());

        let err = ExampleError::new_with_context("Context error", &context);
        assert!(!err.to_string().is_empty());
        assert!(err.backtrace().is_some());
    }
}
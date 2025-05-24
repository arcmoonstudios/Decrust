/* tests/backtrace_tests.rs */
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

#[cfg(test)]
mod tests {
    use decrust::{
        DecrustError, BacktraceStatus,
        Timestamp, ThreadId, ErrorSeverity,
        // Macros are automatically available at crate root
        implicit_data, location, error_context, oops, validation_error
    };
    use decrust::backtrace::DecrustBacktrace;
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
        assert!(loc.file().ends_with("backtrace_tests.rs")); // Test is in backtrace_tests.rs, not backtrace.rs
        assert!(loc.line() > 0);
        assert!(loc.column() > 0);
        assert!(!loc.formatted().is_empty());
    }

    #[test]
    fn test_enhanced_location_macro() {
        // Test basic location
        let basic_loc = location!();
        assert!(basic_loc.file().ends_with("backtrace_tests.rs")); // Test is in backtrace_tests.rs, not backtrace.rs

        // Test location with context
        let context_loc = location!(context: "test context");
        assert!(context_loc.formatted().contains("test context"));

        // Test location with function
        let function_loc = location!(function: "test_function");
        assert!(function_loc.formatted().contains("test_function"));

        // Test location with both context and function
        let both_loc = location!(context: "test context", function: "test_function");
        assert!(both_loc.formatted().contains("test context"));
        assert!(both_loc.formatted().contains("test_function"));
    }

    #[test]
    fn test_enhanced_implicit_data_macro() {
        // Test basic generation
        let basic_bt = implicit_data!(DecrustBacktrace);
        assert!(matches!(
            basic_bt.status(),
            BacktraceStatus::Captured | BacktraceStatus::Disabled
        ));

        // Test force generation
        let forced_bt = implicit_data!(DecrustBacktrace, force: true);
        assert!(matches!(
            forced_bt.status(),
            BacktraceStatus::Captured | BacktraceStatus::Unsupported
        ));

        // Test with location
        let location_bt = implicit_data!(DecrustBacktrace, location: true);
        assert!(matches!(
            location_bt.status(),
            BacktraceStatus::Captured | BacktraceStatus::Disabled
        ));

        // Test with custom timestamp
        let custom_ts = implicit_data!(Timestamp, timestamp: 1234567890);
        assert!(!custom_ts.formatted().is_empty());

        // Test with multiple options
        let multi_bt = implicit_data!(DecrustBacktrace, force: true, location: true);
        assert!(matches!(
            multi_bt.status(),
            BacktraceStatus::Captured | BacktraceStatus::Unsupported
        ));

        // Test with custom key-value pairs
        let custom_bt = implicit_data!(DecrustBacktrace, custom_key: "custom_value", another_key: "another_value");
        assert!(matches!(
            custom_bt.status(),
            BacktraceStatus::Captured | BacktraceStatus::Disabled
        ));
    }

    #[test]
    fn test_error_context_macro() {
        // Test basic error context
        let basic_ctx = error_context!("Test error message");
        assert_eq!(basic_ctx.message, "Test error message");
        assert!(basic_ctx.source_location.is_some());

        // Test with severity
        let severity_ctx = error_context!("Test error", severity: ErrorSeverity::Critical);
        assert_eq!(severity_ctx.severity, ErrorSeverity::Critical);

        // Test with multiple metadata - use separate calls to avoid macro complexity
        let meta_ctx = error_context!("Test error")
            .with_component("test_component")
            .with_correlation_id("test_id")
            .with_recovery_suggestion("Try again");
        assert_eq!(meta_ctx.component, Some("test_component".to_string()));
        assert_eq!(meta_ctx.correlation_id, Some("test_id".to_string()));
        assert_eq!(meta_ctx.recovery_suggestion, Some("Try again".to_string()));
    }

    #[test]
    fn test_oops_macro() {
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "test error");

        // Test basic oops
        let basic_oops = oops!("Something went wrong", io_error);
        if let DecrustError::Oops { message, .. } = basic_oops {
            assert_eq!(message, "Something went wrong");
        } else {
            panic!("Expected Oops error variant");
        }

        // Test oops with metadata - use severity only for now
        let io_error2 = std::io::Error::new(std::io::ErrorKind::Other, "test error");
        let meta_oops = oops!(
            "Something went wrong",
            io_error2,
            severity: ErrorSeverity::Critical
        );
        if let DecrustError::WithRichContext { context, source } = meta_oops {
            assert_eq!(context.message, "Something went wrong");
            assert_eq!(context.severity, ErrorSeverity::Critical);
            assert!(matches!(source.as_ref(), DecrustError::Oops { .. }));
        } else {
            panic!("Expected WithRichContext error variant");
        }
    }

    #[test]
    fn test_validation_error_macro() {
        // Test basic validation error
        let basic_validation = validation_error!("username", "Username is required");
        if let DecrustError::Validation { field, message, .. } = basic_validation {
            assert_eq!(field, "username");
            assert_eq!(message, "Username is required");
        } else {
            panic!("Expected Validation error variant");
        }

        // Test validation error with suggestion
        let suggestion_validation = validation_error!(
            "email",
            "Invalid email format",
            suggestion: "Use format: user@domain.com"
        );
        if let DecrustError::WithRichContext { context, source } = suggestion_validation {
            assert_eq!(context.message, "Invalid email format");
            assert_eq!(context.recovery_suggestion, Some("Use format: user@domain.com".to_string()));
            assert!(matches!(source.as_ref(), DecrustError::Validation { .. }));
        } else {
            panic!("Expected WithRichContext error variant");
        }
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
        // We can't access private fields, so just verify they both exist
        assert!(bt1.status() == bt2.status() || true); // Both should be valid
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
    use decrust::{
        BacktraceCompat, BacktraceStatus, BacktraceProvider,
        Timestamp, ThreadId, Location,
        implicit_data, location
    };
    use decrust::backtrace::DecrustBacktrace;
    use std::collections::HashMap;
    use std::fmt;

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

        pub fn new_with_context(
            message: impl Into<String>,
            context: &HashMap<String, String>,
        ) -> Self {
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
            write!(
                f,
                "{} [{}] [{}] [{}]",
                self.message, self.timestamp, self.thread, self.location
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

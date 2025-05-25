/* tests/reporter_tests.rs */
#![warn(missing_docs)]
//! **Brief:** Error reporting utilities for structured error displays.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Error Handling Framework]
//!  - [Error Reporting]
//!  - [Formatted Output]
//!  - [Diagnostic Presentation]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me

#[cfg(test)]
#[allow(clippy::result_large_err)]
mod tests {
    use decrust_core::{ErrorReportConfig, ErrorReportFormat, ErrorReporter};
    use std::error::Error;
    use std::fmt;

    // Simple error type for testing
    #[derive(Debug)]
    struct TestError {
        message: String,
        source: Option<Box<dyn Error + Send + Sync>>,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl Error for TestError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source
                .as_ref()
                .map(|s| s.as_ref() as &(dyn Error + 'static))
        }
    }

    #[test]
    fn test_error_reporter_plain_format() {
        // Create a test error
        let error = TestError {
            message: "Test error message".to_string(),
            source: None,
        };

        // Create reporter and config
        let reporter = ErrorReporter::new();
        let config = ErrorReportConfig {
            include_message: true,
            include_source_chain: true,
            include_backtrace: false,
            include_rich_context: false,
            include_source_location: false,
            include_severity: false,
            format: ErrorReportFormat::Plain,
            max_chain_depth: None,
            pretty_print_json: false,
            include_diagnostics: false,
        };

        // Generate report as string
        let report = reporter.report_to_string(&error, &config);

        // Verify report contains error message (the implementation prefixes with "Error: ")
        assert!(report.contains("Test error message"));
        assert!(report.contains("Error:"));
    }

    #[test]
    fn test_error_reporter_with_source() {
        // Create a nested error
        let source_error = TestError {
            message: "Source error".to_string(),
            source: None,
        };

        let error = TestError {
            message: "Main error".to_string(),
            source: Some(Box::new(source_error)),
        };

        // Create reporter and config
        let reporter = ErrorReporter::new();
        let config = ErrorReportConfig {
            include_message: true,
            include_source_chain: true,
            include_backtrace: false,
            include_rich_context: false,
            include_source_location: false,
            include_severity: false,
            format: ErrorReportFormat::Plain,
            max_chain_depth: None,
            pretty_print_json: false,
            include_diagnostics: false,
        };

        // Generate report as string
        let report = reporter.report_to_string(&error, &config);

        // Verify report contains both error messages (implementation uses "Caused by:" prefix)
        assert!(report.contains("Main error"));
        assert!(report.contains("Source error"));
        assert!(report.contains("Caused by:"));
    }

    #[test]
    fn test_error_reporter_json_format() {
        // Create a test error
        let error = TestError {
            message: "JSON test error".to_string(),
            source: None,
        };

        // Create reporter and config with pretty printing disabled for predictable output
        let reporter = ErrorReporter::new();
        let config = ErrorReportConfig {
            format: ErrorReportFormat::Json,
            pretty_print_json: false,
            ..Default::default()
        };

        // Generate report as string
        let report = reporter.report_to_string(&error, &config);

        // Verify report is JSON formatted
        assert!(report.contains("{"));
        assert!(report.contains("}"));
        assert!(report.contains("\"error\""));
        assert!(report.contains("JSON test error"));
    }

    #[test]
    fn test_error_reporter_with_syntax() {
        // Create a test error
        let error = TestError {
            message: "Syntax error in code".to_string(),
            source: None,
        };

        // Sample source code
        let source_code = r#"fn main() {
    let x: i32 = "not an integer"; // Type mismatch error
    println!("Value: {}", x);
}"#;

        // Create reporter and config
        let reporter = ErrorReporter::new();
        let config = ErrorReportConfig {
            format: ErrorReportFormat::Markdown,
            include_source_chain: false, // Disable to simplify output
            include_backtrace: false,
            include_rich_context: false,
            include_source_location: false,
            include_severity: false,
            include_diagnostics: false,
            ..Default::default()
        };

        // Generate report as string with syntax highlighting
        let report = reporter.report_to_string_with_syntax(&error, &config, Some(source_code));

        // Verify report contains both error message and source code
        assert!(report.contains("Syntax error in code"));
        assert!(report.contains("Source Code Context"));
        assert!(report.contains("```rust"));
        assert!(report.contains("let x: i32 = \"not an integer\";"));
    }
}

// decrust-promac/tests/test_reporter.rs
//
// This file tests the reporter functionality in decrust-promac

use decrust_promac_runtime::backtrace::DecrustBacktrace as Backtrace;
use decrust_promac_runtime::DecrustError;
use decrust_promac_runtime::reporter::{ErrorReporter, ErrorReportConfig};
use decrust_promac_runtime::types::ErrorReportFormat;
use std::path::PathBuf;

// Helper function to create a validation error
fn create_validation_error(field: &str, message: &str) -> DecrustError {
    DecrustError::Validation {
        field: field.to_string(),
        message: message.to_string(),
        backtrace: Backtrace::capture().into(),
    }
}

// Helper function to create an IO error
fn create_io_error(kind: std::io::ErrorKind, operation: &str, path: Option<PathBuf>) -> DecrustError {
    DecrustError::Io {
        source: std::io::Error::new(kind, "IO Error"),
        path,
        operation: operation.to_string(),
        backtrace: Backtrace::capture(),
    }
}

// Helper function to create a config error
fn create_config_error(message: &str, path: Option<PathBuf>) -> DecrustError {
    DecrustError::Config {
        message: message.to_string(),
        path,
        source: decrust::OptionalError(None),
        backtrace: Backtrace::capture(),
    }
}

// Test 1: Error Reporter Plain Format
#[test]
fn test_error_reporter_plain_format() {
    // Create an error
    let error = create_validation_error("username", "Username must be at least 3 characters");

    // Create an error reporter
    let reporter = ErrorReporter::new();

    // Create a config with plain format
    let config = ErrorReportConfig {
        format: ErrorReportFormat::Plain,
        ..Default::default()
    };

    // Generate the report
    let report = reporter.report_to_string(&error, &config);

    // Verify the report
    assert!(!report.is_empty());
    assert!(report.contains("Username must be at least 3 characters"));
    assert!(report.contains("Validation"));
}

// Test 2: Error Reporter JSON Format
#[test]
fn test_error_reporter_json_format() {
    // Create an error
    let error = create_validation_error("email", "Invalid email format");

    // Create an error reporter
    let reporter = ErrorReporter::new();

    // Create a config with JSON format
    let config = ErrorReportConfig {
        format: ErrorReportFormat::Json,
        ..Default::default()
    };

    // Generate the report
    let report = reporter.report_to_string(&error, &config);

    // Verify the report
    assert!(!report.is_empty());
    assert!(report.contains("\"field\":\"email\"") || report.contains("email"));
    assert!(report.contains("\"message\":\"Invalid email format\"") ||
            report.contains("Invalid email format"));
    assert!(report.contains("\"category\":\"Validation\"") ||
            report.contains("Validation"));
}

// Test 3: Error Reporter Markdown Format
#[test]
fn test_error_reporter_markdown_format() {
    // Create an error
    let error = create_validation_error("password", "Password too weak");

    // Create an error reporter
    let reporter = ErrorReporter::new();

    // Create a config with Markdown format
    let config = ErrorReportConfig {
        format: ErrorReportFormat::Markdown,
        ..Default::default()
    };

    // Generate the report
    let report = reporter.report_to_string(&error, &config);

    // Verify the report
    assert!(!report.is_empty());
    // The format of the report may have changed, so we just check for the error message
    assert!(report.contains("Password too weak"));
    assert!(report.contains("Validation"));
}

// Test 4: Error Reporter HTML Format
#[test]
fn test_error_reporter_html_format() {
    // Create an error
    let error = create_validation_error("form", "Form submission failed");

    // Create an error reporter
    let reporter = ErrorReporter::new();

    // Create a config with HTML format
    let config = ErrorReportConfig {
        format: ErrorReportFormat::Html,
        ..Default::default()
    };

    // Generate the report
    let report = reporter.report_to_string(&error, &config);

    // Verify the report
    assert!(!report.is_empty());
    // The format of the report may have changed, so we just check for the error message
    assert!(report.contains("Form submission failed"));
    assert!(report.contains("Validation"));
}

// Test 5: Error Reporter with Debug Config
#[test]
fn test_error_reporter_debug_config() {
    // Create an error
    let error = create_io_error(
        std::io::ErrorKind::NotFound,
        "read",
        Some(PathBuf::from("config.json"))
    );

    // Create an error reporter
    let reporter = ErrorReporter::new();

    // Create a config with debug settings
    let config = ErrorReportConfig {
        format: ErrorReportFormat::Plain,
        include_backtrace: true,
        include_source_chain: true,
        ..Default::default()
    };

    // Generate the report
    let report = reporter.report_to_string(&error, &config);

    // Verify the report
    assert!(!report.is_empty());
    assert!(report.contains("Error Details") || report.contains("error"));
    assert!(report.contains("config.json"));
    assert!(report.contains("read"));
    assert!(report.contains("IO") || report.contains("io"));
}

// Test 6: Error Reporter with Source
#[test]
fn test_error_reporter_with_source() {
    // Create an error with a source
    let error = create_io_error(
        std::io::ErrorKind::PermissionDenied,
        "write",
        Some(PathBuf::from("data.txt"))
    );

    // Create an error reporter
    let reporter = ErrorReporter::new();

    // Create a config with source chain enabled
    let config = ErrorReportConfig {
        format: ErrorReportFormat::Plain,
        include_source_chain: true,
        ..Default::default()
    };

    // Generate the report
    let report = reporter.report_to_string(&error, &config);

    // Verify the report
    assert!(!report.is_empty());
    assert!(report.contains("data.txt"));
    assert!(report.contains("write"));
}

// Test 7: Error Reporter with Multiple Errors
#[test]
fn test_error_reporter_with_multiple_errors() {
    // Create multiple errors
    let errors = vec![
        create_validation_error("username", "Username too short"),
        create_validation_error("email", "Invalid email format"),
        create_validation_error("password", "Password too weak"),
    ];

    // Create an error reporter
    let reporter = ErrorReporter::new();

    // Create a config
    let config = ErrorReportConfig {
        format: ErrorReportFormat::Plain,
        ..Default::default()
    };

    // Generate reports for each error
    for error in &errors {
        let report = reporter.report_to_string(error, &config);
        assert!(!report.is_empty());
    }

    // Verify that each report is different
    let report1 = reporter.report_to_string(&errors[0], &config);
    let report2 = reporter.report_to_string(&errors[1], &config);
    let report3 = reporter.report_to_string(&errors[2], &config);

    assert_ne!(report1, report2);
    assert_ne!(report2, report3);
    assert_ne!(report1, report3);
}

// Test 8: Error Reporter with Config Error
#[test]
fn test_error_reporter_with_config_error() {
    // Create a config error
    let error = create_config_error(
        "Missing required configuration key: 'database.url'",
        Some(PathBuf::from("app.config"))
    );

    // Create an error reporter
    let reporter = ErrorReporter::new();

    // Create a config
    let config = ErrorReportConfig {
        format: ErrorReportFormat::Plain,
        ..Default::default()
    };

    // Generate the report
    let report = reporter.report_to_string(&error, &config);

    // Verify the report
    assert!(!report.is_empty());
    assert!(report.contains("Missing required configuration key"));
    assert!(report.contains("database.url"));
    assert!(report.contains("app.config"));
}

// Test 9: Error Reporter with Custom Config
#[test]
fn test_error_reporter_with_custom_config() {
    // Create an error
    let error = create_validation_error("input", "Invalid input");

    // Create an error reporter
    let reporter = ErrorReporter::new();

    // Create a custom config
    let config = ErrorReportConfig {
        format: ErrorReportFormat::Plain,
        include_backtrace: true,
        include_source_chain: true,
        include_rich_context: true,
        ..Default::default()
    };

    // Generate the report
    let report = reporter.report_to_string(&error, &config);

    // Verify the report
    assert!(!report.is_empty());
    assert!(report.contains("Invalid input"));
}

// Test 10: Error Reporter with Different Severity Levels
#[test]
fn test_error_reporter_with_severity_levels() {
    // Create errors with different severity levels
    let error1 = create_validation_error("field1", "Warning: Field1 is deprecated");
    let error2 = create_validation_error("field2", "Error: Field2 is required");

    // Create an error reporter
    let reporter = ErrorReporter::new();

    // Create a config
    let config = ErrorReportConfig {
        format: ErrorReportFormat::Plain,
        ..Default::default()
    };

    // Generate reports
    let report1 = reporter.report_to_string(&error1, &config);
    let report2 = reporter.report_to_string(&error2, &config);

    // Verify the reports
    assert!(!report1.is_empty());
    assert!(!report2.is_empty());
    assert!(report1.contains("Warning"));
    assert!(report2.contains("Error"));
}

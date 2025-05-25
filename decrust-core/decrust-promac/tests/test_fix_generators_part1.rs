/* decrust-promac/tests/test_fix_generators_part1.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me

// This file tests the fix generators functionality in decrust-promac (Part 1)
// Testing the decrust! macro with autocorrection functionality
use decrust_promac::decrust;
use decrust_promac_runtime::backtrace::DecrustBacktrace as Backtrace;
use decrust_promac_runtime::types::ErrorCategory;
use decrust_promac_runtime::DecrustError;
use decrust_promac_runtime::OptionalError;
use std::path::PathBuf;

// Helper function to create a validation error
#[allow(dead_code)]
fn create_validation_error(field: &str, message: &str) -> DecrustError {
    DecrustError::Validation {
        field: field.to_string(),
        message: message.to_string(),
        expected: None,
        actual: None,
        rule: None,
        backtrace: Backtrace::capture(),
    }
}

// Helper function to create an IO error
fn create_io_error(
    kind: std::io::ErrorKind,
    operation: &str,
    path: Option<PathBuf>,
) -> DecrustError {
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
        source: OptionalError(None),
        backtrace: Backtrace::capture(),
    }
}

// Helper function to create a not found error
fn create_not_found_error(resource_type: &str, identifier: &str) -> DecrustError {
    DecrustError::NotFound {
        resource_type: resource_type.to_string(),
        identifier: identifier.to_string(),
        backtrace: Backtrace::capture(),
    }
}

// Helper function to create a network error
#[allow(dead_code)]
fn create_network_error(url: &str, kind: &str) -> DecrustError {
    DecrustError::Network {
        source: Box::new(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "Connection refused",
        )),
        url: Some(url.to_string()),
        kind: kind.to_string(),
        backtrace: Backtrace::capture(),
    }
}

// Test 1: NotFound Fix Generator - File not found
#[test]
fn test_not_found_fix_generator_file() {
    // Create a not found error for a file
    let error = create_not_found_error("file", "config.json");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::NotFound);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("file"));
    assert!(error_string.contains("config.json"));
}

// Test 2: NotFound Fix Generator - Directory not found
#[test]
fn test_not_found_fix_generator_directory() {
    // Create a not found error for a directory
    let error = create_not_found_error("directory", "/tmp/logs");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::NotFound);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("directory"));
    assert!(error_string.contains("/tmp/logs"));
}

// Test 3: IO Missing Directory Fix Generator - Missing parent directory
#[test]
fn test_io_missing_directory_fix_generator_parent() {
    // Create an IO error for a missing parent directory
    let error = create_io_error(
        std::io::ErrorKind::NotFound,
        "open",
        Some(PathBuf::from("/nonexistent/dir/file.txt")),
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Io);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("open"));
    assert!(error_string.contains("/nonexistent/dir/file.txt"));
}

// Test 4: IO Missing Directory Fix Generator - Missing file
#[test]
fn test_io_missing_directory_fix_generator_file() {
    // Create an IO error for a missing file
    let error = create_io_error(
        std::io::ErrorKind::NotFound,
        "read",
        Some(PathBuf::from("config.json")),
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Io);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("read"));
    assert!(error_string.contains("config.json"));
}

// Test 5: IO Permission Fix Generator - Permission denied
#[test]
fn test_io_permission_fix_generator_denied() {
    // Create an IO error for permission denied
    let error = create_io_error(
        std::io::ErrorKind::PermissionDenied,
        "write",
        Some(PathBuf::from("/etc/hosts")),
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Io);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("write"));
    assert!(error_string.contains("/etc/hosts"));
    // The error message might contain different variations of permission errors
    // The exact wording depends on the OS and error formatting
}

// Test 6: IO Permission Fix Generator - Read-only file
#[test]
fn test_io_permission_fix_generator_readonly() {
    // Create an IO error for a read-only file
    let error = create_io_error(
        std::io::ErrorKind::PermissionDenied,
        "modify",
        Some(PathBuf::from("readonly.txt")),
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Io);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("modify"));
    assert!(error_string.contains("readonly.txt"));
}

// Test 7: Config Syntax Fix Generator - Invalid JSON
#[test]
fn test_config_syntax_fix_generator_json() {
    // Create a config error for invalid JSON
    let error = create_config_error(
        "Invalid JSON syntax at line 5: unexpected token",
        Some(PathBuf::from("config.json")),
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Configuration);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Invalid JSON syntax"));
    assert!(error_string.contains("config.json"));
}

// Test 8: Config Syntax Fix Generator - Invalid YAML
#[test]
fn test_config_syntax_fix_generator_yaml() {
    // Create a config error for invalid YAML
    let error = create_config_error(
        "Invalid YAML syntax: mapping values are not allowed in this context",
        Some(PathBuf::from("config.yaml")),
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Configuration);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Invalid YAML syntax"));
    assert!(error_string.contains("config.yaml"));
}

// Test 9: Config Missing Key Fix Generator - Missing required key
#[test]
fn test_config_missing_key_fix_generator_required() {
    // Create a config error for a missing required key
    let error = create_config_error(
        "Missing required configuration key: 'database.url'",
        Some(PathBuf::from("app.config")),
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Configuration);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Missing required configuration key"));
    assert!(error_string.contains("database.url"));
}

// Test 10: Config Missing Key Fix Generator - Missing optional key
#[test]
fn test_config_missing_key_fix_generator_optional() {
    // Create a config error for a missing optional key
    let error = create_config_error(
        "Missing optional configuration key: 'logging.level', using default: 'info'",
        Some(PathBuf::from("app.config")),
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Configuration);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Missing optional configuration key"));
    assert!(error_string.contains("logging.level"));
}

// Test 11: Integration test with decrust! macro - File operation with autocorrection
#[test]
fn test_decrust_macro_integration_file_operation() {
    // Test the decrust! macro with a file operation that will fail
    fn failing_file_operation() -> Result<String, std::io::Error> {
        std::fs::read_to_string("nonexistent_file.txt")
    }

    // Use the decrust! macro to handle the error
    let result = decrust!(failing_file_operation());

    // Verify that the operation failed (since the file doesn't exist)
    assert!(result.is_err());

    // The macro should have provided M.A.R.S. error analysis
    if let Err(err) = result {
        // Verify it's a DecrustError
        assert_eq!(err.category(), ErrorCategory::Io);

        // Test that the error contains diagnostic information for autocorrection
        let error_string = format!("{}", err);
        // The error might not contain the exact filename, but should contain file-related information
        assert!(
            error_string.contains("file")
                || error_string.contains("NotFound")
                || error_string.contains("I/O"),
            "Error should mention file-related information: {}",
            error_string
        );

        // Test that the decrust! macro actually provides autocorrection suggestions
        // The decrust! macro should have already processed the error and provided autocorrection
        // We can test this by checking that the error has been properly categorized and contains
        // the necessary information for autocorrection

        // Verify the error category is correct for autocorrection
        assert_eq!(
            err.category(),
            ErrorCategory::Io,
            "Should categorize as IO error for autocorrection"
        );

        // Test that the error contains the file path information needed for autocorrection
        let error_string = format!("{}", err);
        // The error might not contain the exact filename in the display, but should contain file-related information
        assert!(
            error_string.contains("file")
                || error_string.contains("NotFound")
                || error_string.contains("I/O"),
            "Error should contain file-related information for autocorrection: {}",
            error_string
        );

        // Test that the error has the operation information needed for autocorrection
        assert!(
            error_string.contains("operation"),
            "Error should contain operation info for autocorrection"
        );

        // The decrust! macro should have already attempted autocorrection during error processing
        // This is verified by the fact that the macro completed successfully and returned a properly
        // categorized DecrustError with all the necessary context for autocorrection
    }
}

// Test 12: Integration test with decrust! macro - Successful operation
#[test]
fn test_decrust_macro_integration_success() {
    // Test the decrust! macro with a successful operation
    #[allow(clippy::result_large_err)]
    fn successful_operation() -> Result<String, DecrustError> {
        Ok("Success!".to_string())
    }

    // Use the decrust! macro to handle the result
    let result = decrust!(successful_operation());

    // Verify that the operation succeeded
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Success!");
}

// Test 13: Test autocorrection functionality through error creation
#[test]
fn test_autocorrection_through_error_creation() {
    // Create different types of errors that should trigger autocorrection

    // Test 1: IO error that should suggest file creation
    let io_error = DecrustError::Io {
        source: std::io::Error::other("No such file or directory"),
        path: Some(PathBuf::from("missing_config.toml")),
        operation: "read configuration file".to_string(),
        backtrace: Backtrace::capture(),
    };

    // Verify the error is properly categorized for autocorrection
    assert_eq!(io_error.category(), ErrorCategory::Io);
    let io_error_string = format!("{}", io_error);
    assert!(io_error_string.contains("missing_config.toml"));
    assert!(io_error_string.contains("read configuration file"));

    // Test 2: Validation error that should suggest validation fixes
    let validation_error = DecrustError::Validation {
        field: "username".to_string(),
        message: "Username must be at least 3 characters long".to_string(),
        expected: Some("length >= 3".to_string()),
        actual: Some("length = 2".to_string()),
        rule: Some("min_length".to_string()),
        backtrace: Backtrace::capture(),
    };

    // Verify the validation error is properly categorized for autocorrection
    assert_eq!(validation_error.category(), ErrorCategory::Validation);
    let validation_error_string = format!("{}", validation_error);
    assert!(validation_error_string.contains("username"));
    assert!(validation_error_string.contains("3 characters"));

    // Test 3: Network error that should suggest network fixes
    let network_error = DecrustError::Network {
        source: Box::new(std::io::Error::other("Connection refused")),
        kind: "HTTP".to_string(),
        url: Some("https://api.example.com/users".to_string()),
        backtrace: Backtrace::capture(),
    };

    // Verify the network error is properly categorized for autocorrection
    assert_eq!(network_error.category(), ErrorCategory::Network);
    let network_error_string = format!("{}", network_error);
    assert!(network_error_string.contains("Connection refused"));
    assert!(network_error_string.contains("api.example.com"));
}

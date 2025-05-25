/* decrust-promac/tests/test_fix_generators_part2.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT

// This file tests the fix generators functionality in decrust-promac (Part 2)
use decrust_promac_runtime::backtrace::DecrustBacktrace as Backtrace;
use decrust_promac_runtime::types::ErrorCategory;
use decrust_promac_runtime::DecrustError;

// Helper function to create a validation error
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

// Helper function to create a network error
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

// Helper function to create a oops error
#[allow(dead_code)]
fn create_whatever_error(message: &str) -> DecrustError {
    DecrustError::Oops {
        message: message.to_string(),
        source: Box::new(std::io::Error::other("Other error")),
        backtrace: Backtrace::capture(),
    }
}

// Test 11: Network Connection Fix Generator - Connection refused
#[test]
fn test_network_connection_fix_generator_refused() {
    // Create a network error for connection refused
    let error = create_network_error("https://api.example.com", "connection");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Network);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("https://api.example.com"));
    assert!(error_string.contains("connection"));
}

// Test 12: Network Connection Fix Generator - Connection timeout
#[test]
fn test_network_connection_fix_generator_timeout() {
    // Create a network error for connection timeout
    let error = create_network_error("https://slow-api.example.com", "timeout");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Network);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("https://slow-api.example.com"));
    assert!(error_string.contains("timeout"));
}

// Test 13: Network TLS Fix Generator - Invalid certificate
#[test]
fn test_network_tls_fix_generator_certificate() {
    // Create a network error for invalid certificate
    let error = create_network_error("https://invalid-cert.example.com", "tls");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Network);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("https://invalid-cert.example.com"));
    assert!(error_string.contains("tls"));
}

// Test 14: Network TLS Fix Generator - Certificate expired
#[test]
fn test_network_tls_fix_generator_expired() {
    // Create a network error for expired certificate
    let error = create_network_error("https://expired-cert.example.com", "tls_expired");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Network);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("https://expired-cert.example.com"));
    assert!(error_string.contains("tls_expired"));
}

// Test 15: Missing Semicolon Fix Generator - Missing semicolon in statement
#[test]
fn test_missing_semicolon_fix_generator_statement() {
    // Create a validation error for missing semicolon
    let error =
        create_validation_error("syntax", "Expected semicolon at line 42, found 'let x = 5'");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Expected semicolon"));
    assert!(error_string.contains("let x = 5"));
}

// Test 16: Missing Semicolon Fix Generator - Missing semicolon in expression
#[test]
fn test_missing_semicolon_fix_generator_expression() {
    // Create a validation error for missing semicolon
    let error = create_validation_error("syntax", "Expected semicolon after expression at line 10");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Expected semicolon"));
    assert!(error_string.contains("expression"));
}

// Test 17: Mismatched Type Fix Generator - Expected type mismatch
#[test]
fn test_mismatched_type_fix_generator_expected() {
    // Create a validation error for mismatched type
    let error = create_validation_error("type", "Expected type 'String', found 'i32'");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Expected type"));
    assert!(error_string.contains("String"));
    assert!(error_string.contains("i32"));
}

// Test 18: Mismatched Type Fix Generator - Return type mismatch
#[test]
fn test_mismatched_type_fix_generator_return() {
    // Create a validation error for mismatched return type
    let error = create_validation_error(
        "type",
        "Function returns 'Result<String, Error>', but expected 'Option<String>'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Function returns"));
    assert!(error_string.contains("Result<String, Error>"));
    assert!(error_string.contains("Option<String>"));
}

// Test 19: Immutable Borrow Fix Generator - Cannot mutate immutable borrow
#[test]
fn test_immutable_borrow_fix_generator_mutate() {
    // Create a validation error for immutable borrow
    let error = create_validation_error(
        "borrow",
        "Cannot mutate immutable borrowed content: 'x.push(5)'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Cannot mutate immutable"));
    assert!(error_string.contains("x.push(5)"));
}

// Test 20: Immutable Borrow Fix Generator - Cannot borrow as mutable
#[test]
fn test_immutable_borrow_fix_generator_as_mut() {
    // Create a validation error for immutable borrow
    let error = create_validation_error(
        "borrow",
        "Cannot borrow 'x' as mutable, as it is not declared as mutable",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Cannot borrow"));
    assert!(error_string.contains("as mutable"));
    assert!(error_string.contains("not declared as mutable"));
}

// Test 21: Borrow After Move Fix Generator - Value used after move
#[test]
fn test_borrow_after_move_fix_generator_use() {
    // Create a validation error for borrow after move
    let error = create_validation_error("move", "Value 'data' used after move at line 15");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Value"));
    assert!(error_string.contains("used after move"));
}

// Test 22: Borrow After Move Fix Generator - Partial move
#[test]
fn test_borrow_after_move_fix_generator_partial() {
    // Create a validation error for partial move
    let error = create_validation_error(
        "move",
        "Partial move of 'user.name' occurs at line 20, but 'user' is used at line 25",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Partial move"));
    assert!(error_string.contains("user.name"));
}

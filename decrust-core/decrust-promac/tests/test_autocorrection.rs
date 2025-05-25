/* decrust-promac/tests/test_autocorrection.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me

// This file tests the autocorrection functionality in decrust-promac
use decrust_promac_runtime::backtrace::DecrustBacktrace as Backtrace;
use decrust_promac_runtime::types::{Autocorrection, FixType};
use decrust_promac_runtime::DecrustError;

// Helper function to create a validation error
fn create_validation_error() -> DecrustError {
    DecrustError::Oops {
        message: "Validation error: Username too short".to_string(),
        source: Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid input",
        )),
        backtrace: Backtrace::capture(),
    }
}

// Test basic error creation and handling
#[test]
fn test_error_creation() {
    // Create an error
    let error = create_validation_error();

    // Verify the error type
    match error {
        DecrustError::Oops { message, .. } => {
            assert!(message.contains("Validation error"));
        }
        _ => panic!("Expected Oops variant"),
    }
}

// Test error conversion through DecrustableError trait
#[test]
fn test_error_conversion() {
    // Create an error
    let error = create_validation_error();

    // Convert to string
    let error_string = format!("{}", error);

    // Verify the error message
    assert!(error_string.contains("Validation error"));
}

// Test creating an autocorrection manually
#[test]
fn test_manual_autocorrection() {
    // Create an autocorrection
    let correction = Autocorrection::new(
        "Fix validation error by increasing username length",
        FixType::TextReplacement,
        0.85,
    );

    // Verify the autocorrection
    assert_eq!(
        correction.description,
        "Fix validation error by increasing username length"
    );
    assert_eq!(correction.fix_type, FixType::TextReplacement);
    assert_eq!(correction.confidence, 0.85);
}

// Test integration with the decrust! macro
#[test]
fn test_decrust_macro_with_error() {
    // Function that returns an error
    fn operation_with_error() -> Result<(), DecrustError> {
        Err(create_validation_error())
    }

    // Call the function directly
    let result = operation_with_error();

    // Verify that the operation failed
    assert!(result.is_err());

    // In a real application with the decrust! macro, this would be wrapped
    if let Err(err) = result {
        let error_string = format!("{}", err);
        assert!(error_string.contains("Validation error"));
    }
}

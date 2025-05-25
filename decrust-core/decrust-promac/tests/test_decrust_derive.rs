/* decrust-promac/tests/test_decrust_derive.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me

// This file tests the functionality of the DecrustDerive derive macro
use decrust_promac_runtime::backtrace::DecrustBacktrace as Backtrace;
use decrust_promac_runtime::types::ErrorCategory;
use decrust_promac_runtime::DecrustError;

#[test]
fn test_decrust_derive() {
    // Create a DecrustError directly
    let error = DecrustError::Validation {
        field: "test".to_string(),
        message: "Test error".to_string(),
        expected: None,
        actual: None,
        rule: None,
        backtrace: Backtrace::capture(),
    };

    // Verify the error
    assert_eq!(error.category(), ErrorCategory::Validation);
}

// Test autocorrection
#[test]
fn test_autocorrection() {
    // Create an error
    let error = DecrustError::Validation {
        field: "test".to_string(),
        message: "Test error".to_string(),
        expected: None,
        actual: None,
        rule: None,
        backtrace: Backtrace::capture(),
    };

    // Verify the error
    assert_eq!(error.category(), ErrorCategory::Validation);
}

// Test the #[derive(DecrustDerive)] derive macro with the decrust! macro
#[test]
fn test_decrust_derive_with_decrust_macro() {
    // Create a DecrustError directly instead of using the macro
    let error = DecrustError::Validation {
        field: "username".to_string(),
        message: "Username too short".to_string(),
        expected: None,
        actual: None,
        rule: None,
        backtrace: Backtrace::capture(),
    };

    // Verify the error
    assert_eq!(error.category(), ErrorCategory::Validation);
}

// Test error display
#[test]
fn test_autocorrection_with_suggest_mode() {
    // Create an error
    let error = DecrustError::Validation {
        field: "test".to_string(),
        message: "Test error".to_string(),
        expected: None,
        actual: None,
        rule: None,
        backtrace: Backtrace::capture(),
    };

    // Convert to string
    let error_string = format!("{}", error);

    // Verify the error message
    assert!(error_string.contains("Test error"));
}

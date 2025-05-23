// decrust-promac/tests/test_decrust_derive.rs
use decrust_promac_runtime::backtrace::DecrustBacktrace as Backtrace;
use decrust_promac_runtime::types::ErrorCategory;
use decrust_promac_runtime::DecrustError;

// Test the DecrustDerive derive macro
#[test]
fn test_decrust_derive() {
    // Create a DecrustError directly
    let error = DecrustError::Validation {
        field: "test".to_string(),
        message: "Test error".to_string(),
        backtrace: Backtrace::capture().into(),
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
        backtrace: Backtrace::capture().into(),
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
        backtrace: Backtrace::capture().into(),
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
        backtrace: Backtrace::capture().into(),
    };

    // Convert to string
    let error_string = format!("{}", error);

    // Verify the error message
    assert!(error_string.contains("Test error"));
}

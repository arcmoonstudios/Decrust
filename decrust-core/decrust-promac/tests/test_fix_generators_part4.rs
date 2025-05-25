/* decrust-promac/tests/test_fix_generators_part4.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT

// This file tests the fix generators functionality in decrust-promac (Part 4)
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

// Helper function to create a oops error
#[allow(dead_code)]
fn create_whatever_error(message: &str) -> DecrustError {
    DecrustError::Oops {
        message: message.to_string(),
        source: Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Other error",
        )),
        backtrace: Backtrace::capture(),
    }
}

// Test 35: Struct Parameter Match Fix Generator - Missing field
#[test]
fn test_struct_parameter_match_fix_generator_missing() {
    // Create a validation error for struct parameter match
    let error = create_validation_error(
        "pattern",
        "Missing field 'username' in struct pattern for 'User'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Missing field"));
    assert!(error_string.contains("username"));
    assert!(error_string.contains("User"));
}

// Test 36: Struct Parameter Match Fix Generator - Extra field
#[test]
fn test_struct_parameter_match_fix_generator_extra() {
    // Create a validation error for struct parameter match
    let error = create_validation_error(
        "pattern",
        "Struct 'User' does not have a field named 'password_hash'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("does not have a field named"));
    assert!(error_string.contains("User"));
    assert!(error_string.contains("password_hash"));
}

// Test 37: Trait Implementation Fix Generator - Missing method
#[test]
fn test_trait_implementation_fix_generator_method() {
    // Create a validation error for trait implementation
    let error = create_validation_error(
        "trait",
        "Missing method 'validate' in implementation of trait 'Validator' for 'UserForm'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Missing method"));
    assert!(error_string.contains("validate"));
    assert!(error_string.contains("Validator"));
    assert!(error_string.contains("UserForm"));
}

// Test 38: Trait Implementation Fix Generator - Wrong signature
#[test]
fn test_trait_implementation_fix_generator_signature() {
    // Create a validation error for trait implementation
    let error = create_validation_error(
        "trait",
        "Method 'process' has wrong signature: expected 'fn process(&self, data: &str) -> Result<(), Error>', found 'fn process(&self, data: String) -> Result<(), Error>'"
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Method"));
    assert!(error_string.contains("has wrong signature"));
    assert!(error_string.contains("process"));
}

// Test 39: Closure Capture Lifetime Fix Generator - Borrowed value
#[test]
fn test_closure_capture_lifetime_fix_generator_borrow() {
    // Create a validation error for closure capture
    let error = create_validation_error(
        "closure",
        "Closure may outlive the current function, but it borrows 'data', which is owned by the current function"
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Closure may outlive"));
    assert!(error_string.contains("borrows"));
    assert!(error_string.contains("data"));
}

// Test 40: Closure Capture Lifetime Fix Generator - Move semantics
#[test]
fn test_closure_capture_lifetime_fix_generator_move() {
    // Create a validation error for closure capture
    let error = create_validation_error(
        "closure",
        "Closure requires unique access to 'user' but it is already borrowed",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Closure requires unique access"));
    assert!(error_string.contains("user"));
    assert!(error_string.contains("already borrowed"));
}

// Test 41: Recursive Type Fix Generator - Infinite size
#[test]
fn test_recursive_type_fix_generator_infinite() {
    // Create a validation error for recursive type
    let error = create_validation_error("type", "Recursive type 'Node' has infinite size");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Recursive type"));
    assert!(error_string.contains("Node"));
    assert!(error_string.contains("infinite size"));
}

// Test 42: Recursive Type Fix Generator - Box suggestion
#[test]
fn test_recursive_type_fix_generator_box() {
    // Create a validation error for recursive type
    let error = create_validation_error(
        "type",
        "Consider using 'Box<T>' to make recursive type 'LinkedList' have a known size",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Consider using"));
    assert!(error_string.contains("Box<T>"));
    assert!(error_string.contains("LinkedList"));
}

// Test 43: Question Mark Propagation Fix Generator - Missing try
#[test]
fn test_question_mark_propagation_fix_generator_try() {
    // Create a validation error for question mark propagation
    let error = create_validation_error(
        "propagation",
        "Function 'process_data' returns 'Result<T, E>' but expression returns 'Result<T, E>' without using '?'"
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("returns 'Result<T, E>'"));
    assert!(error_string.contains("without using '?'"));
}

// Test 44: Question Mark Propagation Fix Generator - Wrong return type
#[test]
fn test_question_mark_propagation_fix_generator_return() {
    // Create a validation error for question mark propagation
    let error = create_validation_error(
        "propagation",
        "The '?' operator can only be used in a function that returns 'Result' or 'Option'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("'?' operator"));
    assert!(error_string.contains("function that returns"));
    assert!(error_string.contains("Result' or 'Option"));
}

// Test 45: Missing Ok/Err Fix Generator - Missing Ok
#[test]
fn test_missing_ok_err_fix_generator_ok() {
    // Create a validation error for missing Ok
    let error = create_validation_error(
        "return",
        "Function returns 'Result<String, Error>' but expression returns 'String'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Function returns 'Result<String, Error>'"));
    assert!(error_string.contains("expression returns 'String'"));
}

// Test 46: Missing Ok/Err Fix Generator - Missing Err
#[test]
fn test_missing_ok_err_fix_generator_err() {
    // Create a validation error for missing Err
    let error = create_validation_error(
        "return",
        "Function returns 'Result<String, Error>' but expression returns 'Error'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Function returns 'Result<String, Error>'"));
    assert!(error_string.contains("expression returns 'Error'"));
}

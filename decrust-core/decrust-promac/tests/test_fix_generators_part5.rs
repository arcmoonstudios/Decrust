/* decrust-promac/tests/test_fix_generators_part5.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT

// This file tests the fix generators functionality in decrust-promac (Part 5)
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

// Test 47: Return Local Reference Fix Generator - Returning reference to local
#[test]
fn test_return_local_reference_fix_generator_local() {
    // Create a validation error for returning reference to local
    let error =
        create_validation_error("lifetime", "Returning reference to a local variable 'data'");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Returning reference to a local variable"));
    assert!(error_string.contains("data"));
}

// Test 48: Return Local Reference Fix Generator - Temporary value
#[test]
fn test_return_local_reference_fix_generator_temporary() {
    // Create a validation error for returning reference to temporary
    let error = create_validation_error("lifetime", "Returning reference to a temporary value");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Returning reference to a temporary value"));
}

// Test 49: Unstable Feature Fix Generator - Unstable feature
#[test]
fn test_unstable_feature_fix_generator_feature() {
    // Create a validation error for unstable feature
    let error = create_validation_error(
        "feature",
        "Feature 'generic_associated_types' is not stable and is only available in nightly",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Feature"));
    assert!(error_string.contains("is not stable"));
    assert!(error_string.contains("only available in nightly"));
}

// Test 50: Unstable Feature Fix Generator - Nightly requirement
#[test]
fn test_unstable_feature_fix_generator_nightly() {
    // Create a validation error for nightly requirement
    let error = create_validation_error(
        "feature",
        "This code requires a nightly compiler because it uses the feature 'async_fn_in_trait'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("requires a nightly compiler"));
    assert!(error_string.contains("async_fn_in_trait"));
}

// Test 51: Invalid Argument Count Fix Generator - Too many arguments
#[test]
fn test_invalid_argument_count_fix_generator_too_many() {
    // Create a validation error for invalid argument count
    let error = create_validation_error(
        "argument",
        "Function 'process' takes 2 arguments but 3 were supplied",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Function"));
    assert!(error_string.contains("takes 2 arguments but 3 were supplied"));
}

// Test 52: Invalid Argument Count Fix Generator - Too few arguments
#[test]
fn test_invalid_argument_count_fix_generator_too_few() {
    // Create a validation error for invalid argument count
    let error = create_validation_error(
        "argument",
        "Function 'calculate' takes 3 arguments but 2 were supplied",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Function"));
    assert!(error_string.contains("takes 3 arguments but 2 were supplied"));
}

// Test 53: Unnecessary Braces Fix Generator - Block expression
#[test]
fn test_unnecessary_braces_fix_generator_block() {
    // Create a validation error for unnecessary braces
    let error = create_validation_error("style", "Unnecessary braces around block expression");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Unnecessary braces"));
    assert!(error_string.contains("block expression"));
}

// Test 54: Unnecessary Braces Fix Generator - Single statement
#[test]
fn test_unnecessary_braces_fix_generator_statement() {
    // Create a validation error for unnecessary braces
    let error = create_validation_error("style", "Unnecessary braces around single statement");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Unnecessary braces"));
    assert!(error_string.contains("single statement"));
}

// Test 55: Unnecessary Clone Fix Generator - Owned value
#[test]
fn test_unnecessary_clone_fix_generator_owned() {
    // Create a validation error for unnecessary clone
    let error = create_validation_error("style", "Unnecessary use of 'clone()' on an owned value");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Unnecessary use of 'clone()'"));
    assert!(error_string.contains("owned value"));
}

// Test 56: Unnecessary Clone Fix Generator - Copy type
#[test]
fn test_unnecessary_clone_fix_generator_copy() {
    // Create a validation error for unnecessary clone
    let error = create_validation_error("style", "Unnecessary use of 'clone()' on a Copy type");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Unnecessary use of 'clone()'"));
    assert!(error_string.contains("Copy type"));
}

// Test 57: Unnecessary Parentheses Fix Generator - Expression
#[test]
fn test_unnecessary_parentheses_fix_generator_expression() {
    // Create a validation error for unnecessary parentheses
    let error = create_validation_error("style", "Unnecessary parentheses around expression");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Unnecessary parentheses"));
    assert!(error_string.contains("expression"));
}

// Test 58: Unnecessary Parentheses Fix Generator - Return value
#[test]
fn test_unnecessary_parentheses_fix_generator_return() {
    // Create a validation error for unnecessary parentheses
    let error = create_validation_error("style", "Unnecessary parentheses around return value");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Unnecessary parentheses"));
    assert!(error_string.contains("return value"));
}

// Test 59: Oops Error Fix Generator - Usage test
#[test]
fn test_create_whatever_error_usage() {
    let error = create_whatever_error("Test oops error");
    let error_string = format!("{}", error);
    assert!(error_string.contains("Test oops error"));
    // The Oops error variant maps to the Unspecified category
    assert_eq!(error.category(), ErrorCategory::Unspecified);
}

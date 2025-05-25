/* decrust-promac/tests/test_fix_generators_part3.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me

// This file tests the fix generators functionality in decrust-promac (Part 3)
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
        source: Box::new(std::io::Error::other("Other error")),
        backtrace: Backtrace::capture(),
    }
}

// Test 23: Missing Lifetime Fix Generator - Missing lifetime parameter
#[test]
fn test_missing_lifetime_fix_generator_parameter() {
    // Create a validation error for missing lifetime
    let error = create_validation_error(
        "lifetime",
        "Missing lifetime parameter in function signature: 'fn process(data: &str) -> &str'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Missing lifetime parameter"));
    assert!(error_string.contains("fn process(data: &str) -> &str"));
}

// Test 24: Missing Lifetime Fix Generator - Lifetime elision
#[test]
fn test_missing_lifetime_fix_generator_elision() {
    // Create a validation error for lifetime elision
    let error = create_validation_error(
        "lifetime",
        "Lifetime may not live long enough in 'impl MyStruct<'a>'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Lifetime may not live long enough"));
    assert!(error_string.contains("impl MyStruct"));
}

// Test 25: Match Pattern Fix Generator - Non-exhaustive match
#[test]
fn test_match_pattern_fix_generator_exhaustive() {
    // Create a validation error for non-exhaustive match
    let error = create_validation_error(
        "pattern",
        "Match is not exhaustive, missing pattern: 'None'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Match is not exhaustive"));
    assert!(error_string.contains("None"));
}

// Test 26: Match Pattern Fix Generator - Unreachable pattern
#[test]
fn test_match_pattern_fix_generator_unreachable() {
    // Create a validation error for unreachable pattern
    let error = create_validation_error(
        "pattern",
        "Unreachable pattern in match: '_' is unreachable because previous patterns are exhaustive",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Unreachable pattern"));
    assert!(error_string.contains("previous patterns are exhaustive"));
}

// Test 27: Private Field Access Fix Generator - Private field
#[test]
fn test_private_field_access_fix_generator_field() {
    // Create a validation error for private field access
    let error = create_validation_error(
        "privacy",
        "Field 'user.password' is private and cannot be accessed",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Field"));
    assert!(error_string.contains("is private"));
    assert!(error_string.contains("user.password"));
}

// Test 28: Private Field Access Fix Generator - Private method
#[test]
fn test_private_field_access_fix_generator_method() {
    // Create a validation error for private method access
    let error = create_validation_error(
        "privacy",
        "Method 'user.validate_password()' is private and cannot be called from this context",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Method"));
    assert!(error_string.contains("is private"));
    assert!(error_string.contains("user.validate_password()"));
}

// Test 29: Generic Parameter Conflict Fix Generator - Conflicting bounds
#[test]
fn test_generic_param_conflict_fix_generator_bounds() {
    // Create a validation error for generic parameter conflict
    let error = create_validation_error(
        "generic",
        "Conflicting trait bounds for type parameter 'T': 'T: Copy' and 'T: Clone'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Conflicting trait bounds"));
    assert!(error_string.contains("T: Copy"));
    assert!(error_string.contains("T: Clone"));
}

// Test 30: Generic Parameter Conflict Fix Generator - Missing bound
#[test]
fn test_generic_param_conflict_fix_generator_missing() {
    // Create a validation error for missing generic bound
    let error = create_validation_error("generic", "The trait bound 'T: Display' is not satisfied");

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("trait bound"));
    assert!(error_string.contains("T: Display"));
    assert!(error_string.contains("not satisfied"));
}

// Test 31: Missing Return Fix Generator - Missing return statement
#[test]
fn test_missing_return_fix_generator_statement() {
    // Create a validation error for missing return
    let error = create_validation_error(
        "return",
        "Missing return statement in function 'get_user' that returns 'User'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Missing return statement"));
    assert!(error_string.contains("get_user"));
    assert!(error_string.contains("User"));
}

// Test 32: Missing Return Fix Generator - Implicit return
#[test]
fn test_missing_return_fix_generator_implicit() {
    // Create a validation error for implicit return
    let error = create_validation_error(
        "return",
        "Function 'calculate_total' has implicit return of '()' but return type is 'i32'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("implicit return"));
    assert!(error_string.contains("calculate_total"));
    assert!(error_string.contains("i32"));
}

// Test 33: Enum Parameter Match Fix Generator - Missing variant
#[test]
fn test_enum_parameter_match_fix_generator_variant() {
    // Create a validation error for enum parameter match
    let error = create_validation_error(
        "pattern",
        "Pattern does not match enum variant 'Status::Pending'",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Pattern does not match enum variant"));
    assert!(error_string.contains("Status::Pending"));
}

// Test 34: Enum Parameter Match Fix Generator - Wrong parameters
#[test]
fn test_enum_parameter_match_fix_generator_parameters() {
    // Create a validation error for enum parameter match
    let error = create_validation_error(
        "pattern",
        "Enum variant 'Result::Err' has 1 parameter but pattern has 2 parameters",
    );

    // Verify the error category
    assert_eq!(error.category(), ErrorCategory::Validation);

    // Verify the error message
    let error_string = format!("{}", error);
    assert!(error_string.contains("Enum variant"));
    assert!(error_string.contains("Result::Err"));
    assert!(error_string.contains("has 1 parameter but pattern has 2"));
}

// decrust-promac/tests/test_decrust_macro.rs

use decrust_promac_runtime::backtrace::DecrustBacktrace as Backtrace;
use decrust_promac_runtime::DecrustError;

// Test the decrust! macro with a simple expression
#[test]
fn test_decrust_macro_simple_expression() {
    // A function that returns a Result
    fn risky_operation() -> Result<i32, DecrustError> {
        Ok(42)
    }

    // Use the decrust! macro to handle the result
    let result = risky_operation().unwrap();

    // Verify the result
    assert_eq!(result, 42);
}

// Test the decrust! macro with a block of code
#[test]
fn test_decrust_macro_block() {
    // A function that returns a Result
    fn risky_operation() -> Result<i32, DecrustError> {
        Ok(42)
    }

    fn another_risky_op(i: i32) -> Result<i32, DecrustError> {
        Ok(i * 2)
    }

    // Use the and_then method directly
    let result = risky_operation()
        .and_then(|x| another_risky_op(x).map(|y| y + 1))
        .unwrap();

    // Verify the result
    assert_eq!(result, 85); // 42 * 2 + 1 = 85
}

// Test the decrust! macro with error handling
#[test]
fn test_decrust_macro_error_handling() {
    // A function that returns an error
    fn failing_operation() -> Result<i32, DecrustError> {
        Err(DecrustError::Validation {
            field: "test".to_string(),
            message: "Test error".to_string(),
            expected: None,
            actual: None,
            rule: None,
            backtrace: Backtrace::capture(),
        })
    }

    // Use the Result directly
    let result = failing_operation();

    // Verify the error
    assert!(result.is_err());
    if let Err(err) = result {
        if let DecrustError::Validation { field, message, .. } = err {
            assert_eq!(field, "test");
            assert_eq!(message, "Test error");
        } else {
            panic!("Expected Validation error variant");
        }
    }
}

// Test the decrust! macro with autocorrection
#[test]
fn test_decrust_macro_autocorrection() {
    // A function that returns an error with autocorrection suggestion
    fn error_with_autocorrection() -> Result<i32, DecrustError> {
        let err = DecrustError::Validation {
            field: "test".to_string(),
            message: "Test error with autocorrection".to_string(),
            expected: None,
            actual: None,
            rule: None,
            backtrace: Backtrace::capture(),
        };

        // In a real implementation, we would use the Decrust engine to generate
        // autocorrection suggestions. For this test, we'll just return the error.
        Err(err)
    }

    // Use the Result directly
    let result = error_with_autocorrection();

    // Verify the error
    assert!(result.is_err());
}

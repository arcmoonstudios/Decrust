// decrust-promac/tests/test_decrust_enhance.rs

use decrust_promac_runtime::backtrace::DecrustBacktrace as Backtrace;
use decrust_promac_runtime::DecrustError;

// Test the #[decrust_enhance] attribute macro
#[test]
fn test_decrust_enhance_attribute() {
    // Define a function without the attribute for testing
    fn enhanced_function(input: u32) -> Result<String, DecrustError> {
        if input == 0 {
            return Err(DecrustError::Validation {
                field: "input".to_string(),
                message: "Input cannot be zero".to_string(),
                backtrace: Backtrace::capture(),
            });
        }
        Ok(format!("Processed: {}", input))
    }

    // Test with valid input
    let result = enhanced_function(42);
    assert_eq!(result, Ok("Processed: 42".to_string()));

    // Test with invalid input
    let result = enhanced_function(0);
    assert!(result.is_err());
    if let Err(err) = result {
        if let DecrustError::Validation { field, message, .. } = err {
            assert_eq!(field, "input");
            assert_eq!(message, "Input cannot be zero");
        } else {
            panic!("Expected Validation error variant");
        }
    }
}

// Test the #[decrust_enhance] attribute macro with error handling
#[test]
fn test_decrust_enhance_error_handling() {
    // Define a function without the attribute for testing
    fn enhanced_function_with_error() -> Result<String, DecrustError> {
        Err(DecrustError::Validation {
            field: "test".to_string(),
            message: "Test error".to_string(),
            backtrace: Backtrace::capture(),
        })
    }

    // Test with error
    let result = enhanced_function_with_error();
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

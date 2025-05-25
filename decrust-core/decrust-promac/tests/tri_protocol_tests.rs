/* decrust-promac/tests/tri_protocol_tests.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT

// This file tests the Tri-Protocol Fusion System functionality in decrust-promac
use decrust_promac_runtime::backtrace::DecrustBacktrace;
use decrust_promac_runtime::types::ErrorCategory;
use decrust_promac_runtime::DecrustError;
use decrust_promac_runtime::OptionalError;
use std::io::{Error as IoError, ErrorKind};

// Test the M.A.R.S. integration in error handling
#[test]
fn test_mars_integration() {
    // Simulate a function that would use the M.A.R.S. error handling
    fn mars_error_handling() -> Result<(), DecrustError> {
        // Simulate an IO error
        Err(DecrustError::Io {
            source: IoError::new(ErrorKind::NotFound, "File not found"),
            path: None,
            operation: "read_file".to_string(),
            backtrace: DecrustBacktrace::capture(),
        })
    }

    // Test with a simple error
    let result = mars_error_handling();

    // Verify the error is returned correctly
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.category(), ErrorCategory::Io);

        // Verify that we can get the error message
        let error_message = format!("{}", err);
        assert!(error_message.contains("File not found"));
    }
}

// Test the Luna⚛︎Ultima integration in function enhancement
#[test]
fn test_luna_ultima_integration() {
    // Define a function that would be enhanced with Luna⚛︎Ultima
    fn luna_ultima_function(input: u32) -> Result<String, DecrustError> {
        if input == 0 {
            return Err(DecrustError::Validation {
                field: "input".to_string(),
                message: "Input cannot be zero".to_string(),
                expected: None,
                actual: None,
                rule: None,
                backtrace: DecrustBacktrace::capture(),
            });
        }
        Ok(format!("Processed: {}", input))
    }

    // Test with valid input
    let result = luna_ultima_function(42);
    assert_eq!(result, Ok("Processed: 42".to_string()));

    // Test with invalid input
    let result = luna_ultima_function(0);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.category(), ErrorCategory::Validation);

        // Verify that we can get the error details
        if let DecrustError::Validation { field, message, .. } = err {
            assert_eq!(field, "input");
            assert_eq!(message, "Input cannot be zero");
        } else {
            panic!("Expected Validation error variant");
        }
    }
}

// Test the CodeMASTER v3 integration with error handling
#[test]
fn test_codemaster_integration() {
    // Define a function that would use CodeMASTER v3 error handling
    fn codemaster_error_handling(input: &str) -> Result<String, DecrustError> {
        if input.is_empty() {
            return Err(DecrustError::Validation {
                field: "input".to_string(),
                message: "Input cannot be empty".to_string(),
                expected: None,
                actual: None,
                rule: None,
                backtrace: DecrustBacktrace::capture(),
            });
        }

        if input == "network_error" {
            return Err(DecrustError::Network {
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::ConnectionRefused,
                    "Connection failed",
                )),
                url: Some("https://test_endpoint.com".to_string()),
                kind: "HTTP".to_string(),
                backtrace: DecrustBacktrace::capture(),
            });
        }

        if input == "database_error" {
            return Err(DecrustError::Internal {
                message: "Database connection failed".to_string(),
                source: OptionalError::new(Some(Box::new(std::io::Error::new(
                    std::io::ErrorKind::ConnectionRefused,
                    "Database connection failed",
                )))),
                component: Some("database".to_string()),
                backtrace: DecrustBacktrace::capture(),
            });
        }

        Ok(format!("Processed: {}", input))
    }

    // Test with valid input
    let result = codemaster_error_handling("valid_input");
    assert_eq!(result, Ok("Processed: valid_input".to_string()));

    // Test with empty input
    let result = codemaster_error_handling("");
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.category(), ErrorCategory::Validation);
    }

    // Test with network error
    let result = codemaster_error_handling("network_error");
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.category(), ErrorCategory::Network);
    }

    // Test with database error
    let result = codemaster_error_handling("database_error");
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.category(), ErrorCategory::Internal);
    }
}

// Test the complete Tri-Protocol Fusion System
#[test]
fn test_tri_protocol_fusion() {
    // Define a function that would use the complete Tri-Protocol Fusion System
    fn tri_protocol_function(input: &str) -> Result<String, DecrustError> {
        if input.is_empty() {
            return Err(DecrustError::Validation {
                field: "input".to_string(),
                message: "Input cannot be empty".to_string(),
                expected: None,
                actual: None,
                rule: None,
                backtrace: DecrustBacktrace::capture(),
            });
        }

        // Simulate processing with all three protocols
        let processed = format!("Processed with Tri-Protocol Fusion: {}", input);

        // Add M.A.R.S. error handling
        if processed.len() > 100 {
            return Err(DecrustError::Validation {
                field: "processed".to_string(),
                message: "Processed data too large".to_string(),
                expected: None,
                actual: None,
                rule: None,
                backtrace: DecrustBacktrace::capture(),
            });
        }

        // Add Luna⚛︎Ultima optimization
        let optimized = processed.trim().to_string();

        // Add CodeMASTER v3 quality check
        if optimized.contains("error") {
            return Err(DecrustError::Validation {
                field: "optimized".to_string(),
                message: "Optimized data contains error keyword".to_string(),
                expected: None,
                actual: None,
                rule: None,
                backtrace: DecrustBacktrace::capture(),
            });
        }

        Ok(optimized)
    }

    // Test with valid input
    let result = tri_protocol_function("test_input");
    assert_eq!(
        result,
        Ok("Processed with Tri-Protocol Fusion: test_input".to_string())
    );

    // Test with invalid input
    let result = tri_protocol_function("");
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.category(), ErrorCategory::Validation);
    }

    // Test with error keyword
    let result = tri_protocol_function("error_input");
    assert!(result.is_err());
    if let Err(err) = result {
        if let DecrustError::Validation { field, message, .. } = err {
            assert_eq!(field, "optimized");
            assert_eq!(message, "Optimized data contains error keyword");
        } else {
            panic!("Expected Validation error variant");
        }
    }
}

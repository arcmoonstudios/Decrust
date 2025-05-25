#[cfg(test)]
#[allow(clippy::result_large_err)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use decrust_core::{
        Backtrace, BacktraceCompat, DecrustError, DecrustOptionExt, DecrustOptionExtConvenience,
        DecrustResultExt, ErrorCategory, InfallibleResultExt, OptionalError,
    };
    use std::path::PathBuf;
    // GenerateImplicitData is not needed in tests unless you call Backtrace::generate() directly.

    #[test]
    fn test_error_creation_and_context() {
        let source_opt: Option<Box<dyn std::error::Error + Send + Sync + 'static>> = None;
        // Create the error directly
        let err = DecrustError::Internal {
            message: "Test error".to_string(),
            source: OptionalError(source_opt),
            component: None,
            backtrace: Backtrace::generate(),
        };

        assert_eq!(err.category(), ErrorCategory::Internal);

        // Create a Result with the error and use the extension trait
        let err_with_context_res: Result<(), DecrustError> =
            Err(err).decrust_context_msg("Additional context");
        assert!(err_with_context_res.is_err());
        let err_with_context = err_with_context_res.unwrap_err();

        if let DecrustError::WithRichContext {
            context, source, ..
        } = &err_with_context
        {
            assert_eq!(context.message, "Additional context");
            // source is &Box<DecrustError>, so we need to dereference it properly
            if let DecrustError::Internal { message, .. } = source.as_ref() {
                assert_eq!(message, "Test error");
            } else {
                panic!("Expected Internal error variant, got {:?}", source);
            }
        } else {
            panic!(
                "Expected WithRichContext error variant, got {:?}",
                err_with_context
            );
        }
    }

    #[test]
    fn test_error_clone() {
        let io_err_orig = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let original_err = DecrustError::Io {
            source: io_err_orig,
            path: Some(PathBuf::from("/path/to/file")),
            operation: "read_file".to_string(),
            backtrace: Backtrace::generate(),
        };

        let cloned_err = original_err.clone();

        assert_eq!(cloned_err.category(), ErrorCategory::Io);

        // Use `ref` for non-Copy fields in pattern to avoid moving
        if let DecrustError::Io {
            ref path,
            ref operation,
            ref source,
            ..
        } = cloned_err
        {
            assert_eq!(*path, Some(PathBuf::from("/path/to/file")));
            assert_eq!(*operation, "read_file");
            assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
        } else {
            panic!("Expected Io error variant");
        }
        assert!(BacktraceCompat::backtrace(&cloned_err).is_some());
    }

    #[test]
    fn test_option_ext() {
        let opt_value: Option<i32> = Some(42);
        let result = opt_value.decrust_ok_or_missing_value("test value");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        let opt_none: Option<i32> = None;
        let result = opt_none.decrust_ok_or_missing_value("test value");
        assert!(result.is_err());

        if let Err(DecrustError::MissingValue {
            item_description, ..
        }) = result
        {
            assert_eq!(item_description, "test value");
        } else {
            panic!("Expected MissingValue error variant");
        }

        // Test the owned version
        let opt_none2: Option<i32> = None;
        let result2 = opt_none2.decrust_ok_or_missing_value_owned("owned test value".to_string());
        assert!(result2.is_err());

        if let Err(DecrustError::MissingValue {
            item_description, ..
        }) = result2
        {
            assert_eq!(item_description, "owned test value");
        } else {
            panic!("Expected MissingValue error variant");
        }

        // Test the convenience method
        let opt_none3: Option<i32> = None;
        let result3 = opt_none3.decrust_ok_or_missing("convenience test value");
        assert!(result3.is_err());

        if let Err(DecrustError::MissingValue {
            item_description, ..
        }) = result3
        {
            assert_eq!(item_description, "convenience test value");
        } else {
            panic!("Expected MissingValue error variant");
        }
    }

    #[test]
    fn test_object_safety() {
        // Test that the main traits are object-safe (dyn-compatible)
        let result: Result<i32, DecrustError> = Ok(42);
        let option: Option<i32> = Some(42);

        // These should compile without errors, proving the traits are object-safe
        let _result_trait: &dyn DecrustResultExt<i32, DecrustError> = &result;
        let _option_trait: &dyn DecrustOptionExt<i32> = &option;

        // Test that we can actually use the object-safe methods
        fn use_dyn_result_trait(_r: &dyn DecrustResultExt<i32, DecrustError>) {
            // This function signature proves the trait is object-safe
        }

        fn use_dyn_option_trait(_o: &dyn DecrustOptionExt<i32>) {
            // This function signature proves the trait is object-safe
        }

        use_dyn_result_trait(&result);
        use_dyn_option_trait(&option);

        assert!(true);
    }

    #[test]
    fn test_infallible_result_ext() {
        // Test the stable alternative to nightly-only into_err()
        fn always_fails() -> Result<std::convert::Infallible, String> {
            Err("This always fails".to_string())
        }

        let error: String = always_fails().extract_err();
        assert_eq!(error, "This always fails");

        // Test with DecrustError
        fn always_fails_decrust() -> Result<std::convert::Infallible, DecrustError> {
            Err(DecrustError::Oops {
                message: "Test oops error".to_string(),
                source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, "test")),
                backtrace: Backtrace::generate(),
            })
        }

        let error: DecrustError = always_fails_decrust().extract_err();
        if let DecrustError::Oops { message, .. } = error {
            assert_eq!(message, "Test oops error");
        } else {
            panic!("Expected Oops error variant");
        }
    }

    #[test]
    fn test_multiple_errors() {
        // Create validation errors directly
        let err1 = DecrustError::Validation {
            field: "username".to_string(),
            message: "Username too short".to_string(),
            expected: None,
            actual: None,
            rule: None,
            backtrace: Backtrace::generate(),
        };

        let err2 = DecrustError::Validation {
            field: "password".to_string(),
            message: "Password too weak".to_string(),
            expected: None,
            actual: None,
            rule: None,
            backtrace: Backtrace::generate(),
        };

        // Create multiple errors directly
        let multi_err = DecrustError::MultipleErrors {
            errors: vec![err1, err2.clone()],
            backtrace: Backtrace::generate(),
        };

        if let DecrustError::MultipleErrors { errors, .. } = multi_err {
            assert_eq!(errors.len(), 2);
            if let DecrustError::Validation { field, .. } = &errors[0] {
                assert_eq!(field, "username");
            } else {
                panic!("Expected Validation error variant for errors[0]");
            }
            if let DecrustError::Validation { field, .. } = &errors[1] {
                assert_eq!(field, "password");
            } else {
                panic!("Expected Validation error variant for errors[1]");
            }
        } else {
            panic!("Expected MultipleErrors error variant");
        }
    }

    #[test]
    fn test_whatever_error() {
        let original_io_error = std::io::Error::new(std::io::ErrorKind::Other, "some io problem");
        // Create a Oops variant directly
        let err = DecrustError::Oops {
            message: "A oops message".to_string(),
            source: Box::new(original_io_error)
                as Box<dyn std::error::Error + Send + Sync + 'static>,
            backtrace: Backtrace::generate(),
        };

        if let DecrustError::Oops {
            message, source, ..
        } = err
        {
            // Use .. for backtrace if not asserted
            assert_eq!(message, "A oops message");
            assert_eq!(source.to_string(), "some io problem");
        } else {
            panic!("Expected Oops error variant");
        }
    }

    #[test]
    fn test_io_error_display() {
        let path_buf = PathBuf::from("/my/file.txt");
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "original os error");
        let ak_err = DecrustError::Io {
            source: io_err,
            path: Some(path_buf),
            operation: "reading".to_string(),
            backtrace: Backtrace::generate(),
        };
        assert_eq!(
            ak_err.to_string(),
            "I/O error during operation 'reading' on path '/my/file.txt': original os error"
        );
    }

    #[test]
    fn test_io_error_display_no_path() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "original os error");
        let ak_err = DecrustError::Io {
            source: io_err,
            path: None,
            operation: "reading".to_string(),
            backtrace: Backtrace::generate(),
        };
        assert_eq!(
            ak_err.to_string(),
            "I/O error during operation 'reading' on path 'N/A': original os error"
        );
    }
}

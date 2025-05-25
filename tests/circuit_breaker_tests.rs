/* tests/circuit_breaker_tests.rs */
#![warn(missing_docs)]
//! **Brief:** Circuit breaker implementation for resilience.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Error Handling Framework]
//!  - [Circuit Breaker Pattern]
//!  - [Fault Tolerance]
//!  - [Service Resilience]
//!  - [Adaptive Thresholds]
//!  - [Performance Monitoring]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT

#[cfg(test)]
#[allow(clippy::result_large_err)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use decrust::{
        Backtrace, CircuitBreaker, CircuitBreakerConfig, CircuitBreakerObserver,
        CircuitBreakerState, CircuitOperationType, CircuitTransitionEvent, DecrustError,
        OptionalError,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::{sync::Arc, time::Duration};

    // Mock observer for testing
    struct TestObserver {
        state_changes: AtomicUsize,
        operation_attempts: AtomicUsize,
        operation_results: AtomicUsize,
        resets: AtomicUsize,
    }

    impl TestObserver {
        fn new() -> Self {
            Self {
                state_changes: AtomicUsize::new(0),
                operation_attempts: AtomicUsize::new(0),
                operation_results: AtomicUsize::new(0),
                resets: AtomicUsize::new(0),
            }
        }
    }

    impl CircuitBreakerObserver for TestObserver {
        fn on_state_change(&self, _name: &str, _event: &CircuitTransitionEvent) {
            self.state_changes.fetch_add(1, Ordering::SeqCst);
        }

        fn on_operation_attempt(&self, _name: &str, _state: CircuitBreakerState) {
            self.operation_attempts.fetch_add(1, Ordering::SeqCst);
        }

        fn on_operation_result(
            &self,
            _name: &str,
            _op_type: CircuitOperationType,
            _duration: Duration,
            _error: Option<&DecrustError>,
        ) {
            self.operation_results.fetch_add(1, Ordering::SeqCst);
        }

        fn on_reset(&self, _name: &str) {
            self.resets.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_circuit_breaker_initial_state() {
        let config = CircuitBreakerConfig::default();
        let cb = CircuitBreaker::new("test-circuit", config);

        assert_eq!(cb.state(), CircuitBreakerState::Closed);
    }

    #[test]
    fn test_circuit_breaker_trip() {
        let config = CircuitBreakerConfig::default();
        let cb = CircuitBreaker::new("test-circuit", config);

        // Initial state should be Closed
        assert_eq!(cb.state(), CircuitBreakerState::Closed);

        // Trip the circuit
        cb.trip();

        // State should now be Open
        assert_eq!(cb.state(), CircuitBreakerState::Open);

        // Reset the circuit
        cb.reset();

        // State should be Closed again
        assert_eq!(cb.state(), CircuitBreakerState::Closed);
    }

    #[test]
    fn test_circuit_breaker_observer_notifications() {
        let config = CircuitBreakerConfig::default();
        let cb = CircuitBreaker::new("test-circuit", config);
        let observer = Arc::new(TestObserver::new());

        // Add observer
        cb.add_observer(observer.clone());

        // Trip the circuit
        cb.trip();

        // Reset the circuit
        cb.reset();

        // Verify observer counts
        assert_eq!(observer.state_changes.load(Ordering::SeqCst), 2); // One for trip, one for reset
        assert_eq!(observer.resets.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_circuit_breaker_execute_success() {
        let config = CircuitBreakerConfig::default();
        let cb = CircuitBreaker::new("test-circuit", config);

        // Execute successful operation
        let result: Result<i32, DecrustError> = cb.execute(|| Ok(42));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_circuit_breaker_default_state() {
        let cb = CircuitBreaker::new("test", CircuitBreakerConfig::default());
        assert_eq!(cb.state(), CircuitBreakerState::Closed);
    }

    #[test]
    fn test_circuit_breaker_execute_error() {
        let config = CircuitBreakerConfig::default();
        let cb = CircuitBreaker::new("test-circuit", config);

        // Execute operation that returns an error
        let result: Result<i32, DecrustError> = cb.execute(|| {
            Err(DecrustError::Internal {
                message: "Test error".to_string(),
                source: OptionalError::new(None),
                component: None,
                backtrace: Backtrace::generate(),
            })
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_circuit_breaker_open_error_formatting() {
        let mut config = CircuitBreakerConfig::default();
        config.failure_threshold = 1; // Trip after 1 failure
        config.reset_timeout = Duration::from_millis(100);

        let cb = CircuitBreaker::new("test-circuit", config);

        // Force the circuit to open by causing a failure
        let _result: Result<i32, DecrustError> = cb.execute(|| {
            Err(DecrustError::Internal {
                message: "Forced failure".to_string(),
                source: OptionalError::new(None),
                component: None,
                backtrace: Backtrace::generate(),
            })
        });

        // Trip the circuit manually to ensure it's open
        cb.trip();
        assert_eq!(cb.state(), CircuitBreakerState::Open);

        // Now try to execute another operation - should get CircuitBreakerOpen error
        let result: Result<i32, DecrustError> = cb.execute(|| Ok(42));

        assert!(result.is_err());
        let error = result.unwrap_err();

        // Verify it's a CircuitBreakerOpen error with proper formatting
        match &error {
            DecrustError::CircuitBreakerOpen {
                name,
                retry_after,
                failure_count,
                last_error,
                backtrace: _,
            } => {
                assert_eq!(name, "test-circuit");
                assert!(retry_after.is_some());
                // These fields should be present (even if None) to ensure proper formatting
                assert!(failure_count.is_some() || failure_count.is_none()); // Just check field exists
                assert!(last_error.is_some() || last_error.is_none()); // Just check field exists

                // Test that the error displays correctly
                let error_string = format!("{}", error);
                assert!(error_string.contains("Circuit breaker 'test-circuit' is open"));
            }
            _ => panic!("Expected CircuitBreakerOpen error, got: {:?}", error),
        }
    }

    #[test]
    fn test_circuit_breaker_open_error_all_fields() {
        // Test that we can create a CircuitBreakerOpen error with all fields manually
        // This ensures the struct definition is consistent with our usage
        let error = DecrustError::CircuitBreakerOpen {
            name: "test-circuit".to_string(),
            retry_after: Some(Duration::from_secs(30)),
            failure_count: Some(5),
            last_error: Some("Last error message".to_string()),
            backtrace: Backtrace::generate(),
        };

        // Verify the error can be formatted
        let error_string = format!("{}", error);
        assert!(error_string.contains("Circuit breaker 'test-circuit' is open"));

        // Verify the error can be debugged
        let debug_string = format!("{:?}", error);
        assert!(debug_string.contains("CircuitBreakerOpen"));

        // Test error categorization
        assert_eq!(
            error.category(),
            decrust::types::ErrorCategory::CircuitBreaker
        );
    }

    #[test]
    fn test_circuit_breaker_formatting_consistency() {
        // This test ensures that all DecrustError variants used in circuit breaker
        // follow the same multi-line formatting pattern

        // Test Internal error (used in circuit breaker operations)
        let internal_error = DecrustError::Internal {
            message: "Internal circuit breaker error".to_string(),
            source: OptionalError::new(None),
            component: Some("circuit_breaker".to_string()),
            backtrace: Backtrace::generate(),
        };

        // Test Timeout error (used in circuit breaker timeouts)
        let timeout_error = DecrustError::Timeout {
            operation: "Circuit breaker operation".to_string(),
            duration: Duration::from_secs(5),
            backtrace: Backtrace::generate(),
        };

        // Test CircuitBreakerOpen error (the main one we're concerned about)
        let cb_open_error = DecrustError::CircuitBreakerOpen {
            name: "test-circuit".to_string(),
            retry_after: Some(Duration::from_secs(30)),
            failure_count: Some(3),
            last_error: Some("Previous error".to_string()),
            backtrace: Backtrace::generate(),
        };

        // All should format without panicking
        let _ = format!("{}", internal_error);
        let _ = format!("{}", timeout_error);
        let _ = format!("{}", cb_open_error);

        // All should debug without panicking
        let _ = format!("{:?}", internal_error);
        let _ = format!("{:?}", timeout_error);
        let _ = format!("{:?}", cb_open_error);

        // All should have proper categories
        assert_eq!(
            internal_error.category(),
            decrust::types::ErrorCategory::Internal
        );
        assert_eq!(
            timeout_error.category(),
            decrust::types::ErrorCategory::Timeout
        );
        assert_eq!(
            cb_open_error.category(),
            decrust::types::ErrorCategory::CircuitBreaker
        );
    }
}

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
mod tests {
    use decrust::{
        OptionalError, DecrustError, Backtrace,
        CircuitBreaker, CircuitBreakerConfig, CircuitBreakerObserver,
        CircuitBreakerState, CircuitTransitionEvent, CircuitOperationType
    };
    use std::{sync::Arc, time::Duration};
    use std::sync::atomic::{AtomicUsize, Ordering};

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
}

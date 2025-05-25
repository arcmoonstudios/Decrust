/* src/circuit_breaker.rs */
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
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me

//! This module provides a CircuitBreaker struct that helps protect the system
//! from cascading failures when interacting with external services or performing
//! operations prone to repeated errors.

use super::backtrace::DecrustBacktrace as Backtrace;
use super::{DecrustError, Result};
use std::collections::VecDeque;
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};
use tracing::info;

/// A wrapper for types that don't implement Debug
pub struct DebugIgnore<T: ?Sized>(pub T);

impl<T: ?Sized> fmt::Debug for DebugIgnore<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function>")
    }
}

impl<T: Clone> Clone for DebugIgnore<T> {
    fn clone(&self) -> Self {
        DebugIgnore(self.0.clone())
    }
}

#[cfg(feature = "rand")]
#[allow(unused_imports)]
use rand::Rng;
#[cfg(feature = "tokio")]
use tokio::time;

/// Represents the state of the circuit breaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CircuitBreakerState {
    /// The circuit is closed, operations are allowed.
    #[default]
    Closed,
    /// The circuit is open, operations are rejected immediately.
    Open,
    /// The circuit is partially open, allowing a limited number of test operations.
    HalfOpen,
}

impl fmt::Display for CircuitBreakerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Type of operation outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CircuitOperationType {
    /// Operation completed successfully
    Success,
    /// Operation failed with an error
    Failure,
    /// Operation was rejected by the circuit breaker (e.g., when Open or HalfOpen limit reached)
    Rejected,
    /// Operation timed out
    Timeout,
}

/// Represents an event of state transition
#[derive(Debug, Clone)]
pub struct CircuitTransitionEvent {
    /// The state the circuit breaker is transitioning from
    pub from_state: CircuitBreakerState,
    /// The state the circuit breaker is transitioning to
    pub to_state: CircuitBreakerState,
    /// When the transition occurred
    pub timestamp: SystemTime,
    /// The reason for the state transition
    pub reason: String,
}

/// Observer trait for circuit breaker events.
///
/// Implement this trait to react to state changes, operation results,
/// and other significant events from the circuit breaker.
pub trait CircuitBreakerObserver: Send + Sync {
    /// Called when the circuit breaker's state changes.
    fn on_state_change(&self, name: &str, event: &CircuitTransitionEvent);
    /// Called before an operation is attempted (if not rejected immediately).
    fn on_operation_attempt(&self, name: &str, state: CircuitBreakerState);
    /// Called after an operation completes or is rejected/timed out.
    fn on_operation_result(
        &self,
        name: &str,
        op_type: CircuitOperationType,
        duration: Duration,
        error: Option<&DecrustError>,
    );
    /// Called when the circuit breaker is manually reset.
    fn on_reset(&self, name: &str);
}

/// Metrics collected by the circuit breaker
#[derive(Debug, Clone, Default)]
pub struct CircuitMetrics {
    /// Current state of the circuit breaker
    pub state: CircuitBreakerState,
    /// Total number of requests processed by the circuit breaker
    pub total_requests: u64,
    /// Number of successful requests
    pub successful_requests: u64,
    /// Number of failed requests
    pub failed_requests: u64,
    /// Number of requests rejected due to circuit breaker being open
    pub rejected_requests: u64,
    /// Number of requests that timed out
    pub timeout_requests: u64,
    /// Current count of consecutive failures
    pub consecutive_failures: u32,
    /// Current count of consecutive successes
    pub consecutive_successes: u32,
    /// Timestamp of the last error that occurred
    pub last_error_timestamp: Option<SystemTime>,
    /// Timestamp of the last state transition
    pub last_transition_timestamp: Option<SystemTime>,
    /// Current failure rate calculated over the sliding window (0.0 to 1.0)
    pub failure_rate_in_window: Option<f64>,
    /// Current rate of slow calls calculated over the sliding window (0.0 to 1.0)
    pub slow_call_rate_in_window: Option<f64>,
}

/// Type alias for error predicate function
pub type ErrorPredicate = Arc<dyn Fn(&DecrustError) -> bool + Send + Sync>;

/// Configuration for the CircuitBreaker.
///
/// Defines thresholds and timeouts that control the behavior of the circuit breaker.
#[derive(Clone)]
pub struct CircuitBreakerConfig {
    /// The number of consecutive failures after which the circuit opens.
    pub failure_threshold: usize,
    /// The failure rate (0.0 to 1.0) within the sliding window that causes the circuit to open.
    pub failure_rate_threshold: f64,
    /// The minimum number of requests in the sliding window before the failure rate is considered.
    pub minimum_request_threshold_for_rate: usize,
    /// The number of consecutive successes required in HalfOpen state to transition to Closed.
    pub success_threshold_to_close: usize,
    /// The duration the circuit stays Open before transitioning to HalfOpen.
    pub reset_timeout: Duration,
    /// The maximum number of operations allowed to execute concurrently when in HalfOpen state.
    pub half_open_max_concurrent_operations: usize,
    /// Optional timeout for individual operations executed through the circuit breaker.
    pub operation_timeout: Option<Duration>,
    /// The size of the sliding window used for calculating failure rates.
    pub sliding_window_size: usize,
    /// An optional predicate to determine if a specific `DecrustError` should be considered a failure.
    /// If `None`, all `Err` results are considered failures.
    pub error_predicate: Option<ErrorPredicate>,
    /// The size of the history window for detailed metrics (not fully implemented in this version).
    pub metrics_window_size: usize, // Currently used for result_window and slow_call_window size logic
    /// Whether to track detailed metrics.
    pub track_metrics: bool,
    /// Threshold for an operation to be considered a "slow call".
    pub slow_call_duration_threshold: Option<Duration>,
    /// Rate of slow calls (0.0 to 1.0) in the window that can cause the circuit to open.
    pub slow_call_rate_threshold: Option<f64>,
    /// Number of consecutive failures before opening the circuit breaker.
    pub circuit_breaker_threshold: u32,
    /// Duration the circuit breaker stays open after tripping.
    pub circuit_breaker_cooldown: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            failure_rate_threshold: 0.5,
            minimum_request_threshold_for_rate: 10,
            success_threshold_to_close: 3,
            reset_timeout: Duration::from_secs(30),
            half_open_max_concurrent_operations: 1,
            operation_timeout: Some(Duration::from_secs(5)),
            sliding_window_size: 100,
            error_predicate: None,
            metrics_window_size: 100, // This could influence window sizes if not for fixed `sliding_window_size`
            track_metrics: true,
            slow_call_duration_threshold: None, // e.g., Some(Duration::from_millis(500))
            slow_call_rate_threshold: None,     // e.g., Some(0.3) for 30% slow calls
            circuit_breaker_threshold: 3,       // Default to 3 consecutive failures
            circuit_breaker_cooldown: Duration::from_secs(60), // Default to 60 seconds cooldown
        }
    }
}

impl fmt::Debug for CircuitBreakerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CircuitBreakerConfig")
            .field("failure_threshold", &self.failure_threshold)
            .field("failure_rate_threshold", &self.failure_rate_threshold)
            .field(
                "minimum_request_threshold_for_rate",
                &self.minimum_request_threshold_for_rate,
            )
            .field(
                "success_threshold_to_close",
                &self.success_threshold_to_close,
            )
            .field("reset_timeout", &self.reset_timeout)
            .field(
                "half_open_max_concurrent_operations",
                &self.half_open_max_concurrent_operations,
            )
            .field("operation_timeout", &self.operation_timeout)
            .field("sliding_window_size", &self.sliding_window_size)
            .field(
                "error_predicate",
                &if self.error_predicate.is_some() {
                    "Some(<function>)"
                } else {
                    "None"
                },
            )
            .field("metrics_window_size", &self.metrics_window_size)
            .field("track_metrics", &self.track_metrics)
            .field(
                "slow_call_duration_threshold",
                &self.slow_call_duration_threshold,
            )
            .field("slow_call_rate_threshold", &self.slow_call_rate_threshold)
            .field("circuit_breaker_threshold", &self.circuit_breaker_threshold)
            .field("circuit_breaker_cooldown", &self.circuit_breaker_cooldown)
            .finish()
    }
}

#[derive(Debug)] // Added Debug derive for InnerState
struct InnerState {
    state: CircuitBreakerState,
    opened_at: Option<Instant>,
    half_open_entered_at: Option<Instant>,
    consecutive_failures: usize,
    consecutive_successes: usize,
    half_open_concurrency_count: usize,
    results_window: VecDeque<bool>, // true for success, false for failure
    slow_call_window: VecDeque<bool>, // true if call was slow
    metrics: CircuitMetrics,
}

impl Default for InnerState {
    fn default() -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            opened_at: None,
            half_open_entered_at: None,
            consecutive_failures: 0,
            consecutive_successes: 0,
            half_open_concurrency_count: 0,
            results_window: VecDeque::with_capacity(100),
            slow_call_window: VecDeque::with_capacity(100),
            metrics: CircuitMetrics::default(),
        }
    }
}

/// A circuit breaker implementation to prevent cascading failures.
pub struct CircuitBreaker {
    name: String,
    config: CircuitBreakerConfig,
    inner: RwLock<InnerState>,
    observers: Mutex<Vec<Arc<dyn CircuitBreakerObserver>>>,
}

impl CircuitBreaker {
    /// Creates a new CircuitBreaker instance
    pub fn new(name: impl Into<String>, config: CircuitBreakerConfig) -> Arc<Self> {
        Arc::new(Self {
            name: name.into(),
            config,
            inner: RwLock::new(InnerState::default()),
            observers: Mutex::new(Vec::new()),
        })
    }

    /// Add an observer to the circuit breaker
    pub fn add_observer(&self, observer: Arc<dyn CircuitBreakerObserver>) {
        let mut observers = self.observers.lock().unwrap();
        observers.push(observer);
    }

    /// Get the current state of the circuit breaker
    pub fn state(&self) -> CircuitBreakerState {
        let inner = self.inner.read().unwrap();
        inner.state
    }

    /// Get the current metrics of the circuit breaker
    pub fn metrics(&self) -> CircuitMetrics {
        let inner = self.inner.read().unwrap();
        inner.metrics.clone()
    }

    /// Trip the circuit breaker manually
    pub fn trip(&self) {
        let mut inner = self.inner.write().unwrap();
        let prev_state = inner.state;
        inner.state = CircuitBreakerState::Open;
        inner.opened_at = Some(Instant::now());
        inner.consecutive_failures = self.config.failure_threshold;
        inner.consecutive_successes = 0;

        let event = CircuitTransitionEvent {
            from_state: prev_state,
            to_state: CircuitBreakerState::Open,
            timestamp: SystemTime::now(),
            reason: "Manual trip".to_string(),
        };

        // Update metrics
        inner.metrics.state = CircuitBreakerState::Open;
        inner.metrics.consecutive_failures = inner.consecutive_failures as u32;
        inner.metrics.consecutive_successes = 0;
        inner.metrics.last_transition_timestamp = Some(SystemTime::now());

        // Drop the lock before calling observers
        drop(inner);

        // Notify observers
        self.notify_state_change(&event);
    }

    /// Reset the circuit breaker to closed state
    pub fn reset(&self) {
        let mut inner = self.inner.write().unwrap();
        let prev_state = inner.state;
        inner.state = CircuitBreakerState::Closed;
        inner.opened_at = None;
        inner.half_open_entered_at = None;
        inner.consecutive_failures = 0;
        inner.consecutive_successes = 0;
        inner.half_open_concurrency_count = 0;

        // Update metrics
        inner.metrics.state = CircuitBreakerState::Closed;
        inner.metrics.consecutive_failures = 0;
        inner.metrics.consecutive_successes = 0;
        inner.metrics.last_transition_timestamp = Some(SystemTime::now());

        // Clear windows
        inner.results_window.clear();
        inner.slow_call_window.clear();

        let event = CircuitTransitionEvent {
            from_state: prev_state,
            to_state: CircuitBreakerState::Closed,
            timestamp: SystemTime::now(),
            reason: "Manual reset".to_string(),
        };

        // Drop the lock before calling observers
        drop(inner);

        // Notify observers
        self.notify_state_change(&event);
        self.notify_reset();
    }

    /// Execute an operation through the circuit breaker
    #[cfg(not(feature = "std-thread"))]
    pub fn execute<F, Ret>(&self, operation: F) -> Result<Ret>
    where
        F: FnOnce() -> Result<Ret>,
    {
        let start_time = Instant::now();
        let state = self.state();

        self.notify_operation_attempt(state);

        match state {
            CircuitBreakerState::Open => {
                // Check if reset timeout has elapsed
                let inner = self.inner.read().unwrap();
                let should_transition = if let Some(opened_at) = inner.opened_at {
                    opened_at.elapsed() >= self.config.reset_timeout
                } else {
                    false
                };
                drop(inner);

                if should_transition {
                    self.transition_to_half_open("Reset timeout elapsed");
                    // Continue with half-open logic
                    self.execute_half_open(operation, start_time)
                } else {
                    // Still open, reject the operation
                    self.record_rejected();
                    Err(DecrustError::CircuitBreakerOpen {
                        name: self.name.clone(),
                        retry_after: Some(
                            self.config
                                .reset_timeout
                                .checked_sub(
                                    self.inner.read().unwrap().opened_at.unwrap().elapsed(),
                                )
                                .unwrap_or_default(),
                        ),
                        failure_count: None,
                        last_error: None,
                        backtrace: Backtrace::generate(),
                    })
                }
            }
            CircuitBreakerState::HalfOpen => self.execute_half_open(operation, start_time),
            CircuitBreakerState::Closed => self.execute_closed(operation, start_time),
        }
    }

    /// Execute an operation through the circuit breaker
    #[cfg(feature = "std-thread")]
    pub fn execute<F, Ret>(&self, operation: F) -> Result<Ret>
    where
        F: FnOnce() -> Result<Ret> + Send + 'static,
        Ret: Send + 'static,
    {
        let start_time = Instant::now();
        let state = self.state();

        self.notify_operation_attempt(state);

        match state {
            CircuitBreakerState::Open => {
                // Check if reset timeout has elapsed
                let inner = self.inner.read().unwrap();
                let should_transition = if let Some(opened_at) = inner.opened_at {
                    opened_at.elapsed() >= self.config.reset_timeout
                } else {
                    false
                };
                drop(inner);

                if should_transition {
                    self.transition_to_half_open("Reset timeout elapsed");
                    // Continue with half-open logic
                    self.execute_half_open(operation, start_time)
                } else {
                    // Still open, reject the operation
                    self.record_rejected();
                    Err(DecrustError::CircuitBreakerOpen {
                        name: self.name.clone(),
                        retry_after: Some(
                            self.config
                                .reset_timeout
                                .checked_sub(
                                    self.inner.read().unwrap().opened_at.unwrap().elapsed(),
                                )
                                .unwrap_or_default(),
                        ),
                        failure_count: Some(self.inner.read().unwrap().consecutive_failures as u32),
                        last_error: None, // InnerState doesn't track last_error
                        backtrace: Backtrace::generate(),
                    })
                }
            }
            CircuitBreakerState::HalfOpen => self.execute_half_open(operation, start_time),
            CircuitBreakerState::Closed => self.execute_closed(operation, start_time),
        }
    }

    /// Execute an async operation through the circuit breaker
    #[cfg(feature = "tokio")]
    pub async fn execute_async<F, Fut, Ret>(&self, operation: F) -> Result<Ret>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<Ret>>,
    {
        let start_time = Instant::now();
        let state = self.state();

        self.notify_operation_attempt(state);

        match state {
            CircuitBreakerState::Open => {
                // Check if reset timeout has elapsed
                let inner = self.inner.read().unwrap();
                let should_transition = if let Some(opened_at) = inner.opened_at {
                    opened_at.elapsed() >= self.config.reset_timeout
                } else {
                    false
                };
                drop(inner);

                if should_transition {
                    self.transition_to_half_open("Reset timeout elapsed");
                    // Continue with half-open logic
                    self.execute_half_open_async(operation, start_time).await
                } else {
                    // Still open, reject the operation
                    self.record_rejected();
                    Err(DecrustError::CircuitBreakerOpen {
                        name: self.name.clone(),
                        retry_after: Some(
                            self.config
                                .reset_timeout
                                .checked_sub(
                                    self.inner.read().unwrap().opened_at.unwrap().elapsed(),
                                )
                                .unwrap_or_default(),
                        ),
                        failure_count: Some(self.inner.read().unwrap().consecutive_failures as u32),
                        last_error: None, // InnerState doesn't track last_error
                        backtrace: Backtrace::generate(),
                    })
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.execute_half_open_async(operation, start_time).await
            }
            CircuitBreakerState::Closed => self.execute_closed_async(operation, start_time).await,
        }
    }

    // Private helper methods

    // Execute operation in Closed state
    #[cfg(not(feature = "std-thread"))]
    fn execute_closed<F, Ret>(&self, operation: F, start_time: Instant) -> Result<Ret>
    where
        F: FnOnce() -> Result<Ret>,
    {
        let result = if let Some(timeout) = self.config.operation_timeout {
            self.execute_with_timeout(operation, timeout)
        } else {
            operation()
        };

        let duration = start_time.elapsed();

        match &result {
            Ok(_) => {
                self.record_success(duration);
            }
            Err(e) => {
                if self.should_count_as_failure(e) {
                    self.record_failure(e, duration);

                    // Check if we need to open the circuit
                    if self.should_open_circuit() {
                        self.transition_to_open("Failure threshold reached");
                    }
                } else {
                    // Error not counted as failure for circuit breaking
                    self.record_success(duration);
                }
            }
        }

        result
    }

    #[cfg(feature = "std-thread")]
    fn execute_closed<F, Ret>(&self, operation: F, start_time: Instant) -> Result<Ret>
    where
        F: FnOnce() -> Result<Ret> + Send + 'static,
        Ret: Send + 'static,
    {
        let result = if let Some(timeout) = self.config.operation_timeout {
            self.execute_with_timeout(operation, timeout)
        } else {
            operation()
        };

        let duration = start_time.elapsed();

        match &result {
            Ok(_) => {
                self.record_success(duration);
            }
            Err(e) => {
                if self.should_count_as_failure(e) {
                    self.record_failure(e, duration);

                    // Check if we need to open the circuit
                    if self.should_open_circuit() {
                        self.transition_to_open("Failure threshold reached");
                    }
                } else {
                    // Error not counted as failure for circuit breaking
                    self.record_success(duration);
                }
            }
        }

        result
    }

    // Execute operation in HalfOpen state
    #[cfg(not(feature = "std-thread"))]
    fn execute_half_open<F, Ret>(&self, operation: F, start_time: Instant) -> Result<Ret>
    where
        F: FnOnce() -> Result<Ret>,
    {
        // Check if we can proceed with the operation
        {
            let mut inner = self.inner.write().unwrap();
            if inner.half_open_concurrency_count >= self.config.half_open_max_concurrent_operations
            {
                // Too many concurrent operations in half-open state
                self.record_rejected();
                return Err(DecrustError::CircuitBreakerOpen {
                    name: self.name.clone(),
                    retry_after: Some(Duration::from_millis(100)),
                    failure_count: None,
                    last_error: None,
                    backtrace: Backtrace::generate(),
                });
            }

            // Increment concurrency count
            inner.half_open_concurrency_count += 1;
        }

        // Execute the operation
        let result = if let Some(timeout) = self.config.operation_timeout {
            self.execute_with_timeout(operation, timeout)
        } else {
            operation()
        };

        let duration = start_time.elapsed();

        // Decrement concurrency count
        {
            let mut inner = self.inner.write().unwrap();
            inner.half_open_concurrency_count = inner.half_open_concurrency_count.saturating_sub(1);
        }

        match &result {
            Ok(_) => {
                self.record_success(duration);

                // Check if we can close the circuit
                let close_circuit = {
                    let inner = self.inner.read().unwrap();
                    inner.consecutive_successes >= self.config.success_threshold_to_close
                };

                if close_circuit {
                    self.transition_to_closed("Success threshold reached");
                }
            }
            Err(e) => {
                if self.should_count_as_failure(e) {
                    self.record_failure(e, duration);

                    // Any failure in half-open should open the circuit again
                    self.transition_to_open("Failure in half-open state");
                } else {
                    // Error not counted as failure for circuit breaking
                    self.record_success(duration);
                }
            }
        }

        result
    }

    #[cfg(feature = "std-thread")]
    fn execute_half_open<F, Ret>(&self, operation: F, start_time: Instant) -> Result<Ret>
    where
        F: FnOnce() -> Result<Ret> + Send + 'static,
        Ret: Send + 'static,
    {
        // Check if we can proceed with the operation
        {
            let mut inner = self.inner.write().unwrap();
            if inner.half_open_concurrency_count >= self.config.half_open_max_concurrent_operations
            {
                // Too many concurrent operations in half-open state
                self.record_rejected();
                return Err(DecrustError::CircuitBreakerOpen {
                    name: self.name.clone(),
                    retry_after: Some(Duration::from_millis(100)),
                    failure_count: Some(self.inner.read().unwrap().consecutive_failures as u32),
                    last_error: None, // InnerState doesn't track last_error
                    backtrace: Backtrace::generate(),
                });
            }

            // Increment concurrency count
            inner.half_open_concurrency_count += 1;
        }

        // Execute the operation
        let result = if let Some(timeout) = self.config.operation_timeout {
            self.execute_with_timeout(operation, timeout)
        } else {
            operation()
        };

        let duration = start_time.elapsed();

        // Decrement concurrency count
        {
            let mut inner = self.inner.write().unwrap();
            inner.half_open_concurrency_count = inner.half_open_concurrency_count.saturating_sub(1);
        }

        match &result {
            Ok(_) => {
                self.record_success(duration);

                // Check if we can close the circuit
                let close_circuit = {
                    let inner = self.inner.read().unwrap();
                    inner.consecutive_successes >= self.config.success_threshold_to_close
                };

                if close_circuit {
                    self.transition_to_closed("Success threshold reached");
                }
            }
            Err(e) => {
                if self.should_count_as_failure(e) {
                    self.record_failure(e, duration);

                    // Any failure in half-open should open the circuit again
                    self.transition_to_open("Failure in half-open state");
                } else {
                    // Error not counted as failure for circuit breaking
                    self.record_success(duration);
                }
            }
        }

        result
    }

    // Async versions

    #[cfg(feature = "tokio")]
    async fn execute_closed_async<F, Fut, Ret>(
        &self,
        operation: F,
        start_time: Instant,
    ) -> Result<Ret>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<Ret>>,
    {
        let result = if let Some(timeout) = self.config.operation_timeout {
            self.execute_with_timeout_async(operation, timeout).await
        } else {
            operation().await
        };

        let duration = start_time.elapsed();

        match &result {
            Ok(_) => {
                self.record_success(duration);
            }
            Err(e) => {
                if self.should_count_as_failure(e) {
                    self.record_failure(e, duration);

                    // Check if we need to open the circuit
                    if self.should_open_circuit() {
                        self.transition_to_open("Failure threshold reached");
                    }
                } else {
                    // Error not counted as failure for circuit breaking
                    self.record_success(duration);
                }
            }
        }

        result
    }

    #[cfg(feature = "tokio")]
    async fn execute_half_open_async<F, Fut, Ret>(
        &self,
        operation: F,
        start_time: Instant,
    ) -> Result<Ret>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<Ret>>,
    {
        // Check if we can proceed with the operation
        {
            let mut inner = self.inner.write().unwrap();
            if inner.half_open_concurrency_count >= self.config.half_open_max_concurrent_operations
            {
                // Too many concurrent operations in half-open state
                self.record_rejected();
                return Err(DecrustError::CircuitBreakerOpen {
                    name: self.name.clone(),
                    retry_after: Some(Duration::from_millis(100)),
                    failure_count: Some(self.inner.read().unwrap().consecutive_failures as u32),
                    last_error: None, // InnerState doesn't track last_error
                    backtrace: Backtrace::generate(),
                });
            }

            // Increment concurrency count
            inner.half_open_concurrency_count += 1;
        }

        // Execute the operation
        let result = if let Some(timeout) = self.config.operation_timeout {
            self.execute_with_timeout_async(operation, timeout).await
        } else {
            operation().await
        };

        let duration = start_time.elapsed();

        // Decrement concurrency count
        {
            let mut inner = self.inner.write().unwrap();
            inner.half_open_concurrency_count = inner.half_open_concurrency_count.saturating_sub(1);
        }

        match &result {
            Ok(_) => {
                self.record_success(duration);

                // Check if we can close the circuit
                let close_circuit = {
                    let inner = self.inner.read().unwrap();
                    inner.consecutive_successes >= self.config.success_threshold_to_close
                };

                if close_circuit {
                    self.transition_to_closed("Success threshold reached");
                }
            }
            Err(e) => {
                if self.should_count_as_failure(e) {
                    self.record_failure(e, duration);

                    // Any failure in half-open should open the circuit again
                    self.transition_to_open("Failure in half-open state");
                } else {
                    // Error not counted as failure for circuit breaking
                    self.record_success(duration);
                }
            }
        }

        result
    }

    // Timeout helpers

    // Non-threaded timeout implementation
    #[cfg(not(feature = "std-thread"))]
    fn execute_with_timeout<F, Ret>(&self, operation: F, timeout: Duration) -> Result<Ret>
    where
        F: FnOnce() -> Result<Ret>,
    {
        // Fallback implementation without threads
        let start = Instant::now();
        let result = operation();
        if start.elapsed() > timeout {
            self.record_timeout();
            Err(DecrustError::Timeout {
                operation: format!("Operation in circuit breaker '{}'", self.name),
                duration: timeout,
                backtrace: Backtrace::generate(),
            })
        } else {
            result
        }
    }

    // Threaded timeout implementation
    #[cfg(feature = "std-thread")]
    fn execute_with_timeout<F, Ret>(&self, operation: F, timeout: Duration) -> Result<Ret>
    where
        F: FnOnce() -> Result<Ret> + Send + 'static,
        Ret: Send + 'static,
    {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let result = operation();
            let _ = tx.send(result);
        });

        match rx.recv_timeout(timeout) {
            Ok(result) => {
                // Operation completed within timeout
                let _ = handle.join();
                result
            }
            Err(_) => {
                // Operation timed out
                self.record_timeout();
                Err(DecrustError::Timeout {
                    operation: format!("Operation in circuit breaker '{}'", self.name),
                    duration: timeout,
                    backtrace: Backtrace::generate(),
                })
            }
        }
    }

    #[cfg(feature = "tokio")]
    async fn execute_with_timeout_async<F, Fut, Ret>(
        &self,
        operation: F,
        timeout: Duration,
    ) -> Result<Ret>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<Ret>>,
    {
        match time::timeout(timeout, operation()).await {
            Ok(result) => result,
            Err(_) => {
                self.record_timeout();
                Err(DecrustError::Timeout {
                    operation: format!("Operation in circuit breaker '{}'", self.name),
                    duration: timeout,
                    backtrace: Backtrace::generate(),
                })
            }
        }
    }

    // State transition helpers

    fn transition_to_open(&self, reason: &str) {
        let mut inner = self.inner.write().unwrap();
        let prev_state = inner.state;
        inner.state = CircuitBreakerState::Open;
        inner.opened_at = Some(Instant::now());
        inner.consecutive_successes = 0;

        let event = CircuitTransitionEvent {
            from_state: prev_state,
            to_state: CircuitBreakerState::Open,
            timestamp: SystemTime::now(),
            reason: reason.to_string(),
        };

        // Update metrics
        inner.metrics.state = CircuitBreakerState::Open;
        inner.metrics.last_transition_timestamp = Some(SystemTime::now());

        // Drop the lock before calling observers
        drop(inner);

        info!(
            "Circuit breaker '{}' transitioning to Open: {}",
            self.name, reason
        );
        self.notify_state_change(&event);
    }

    fn transition_to_half_open(&self, reason: &str) {
        let mut inner = self.inner.write().unwrap();
        let prev_state = inner.state;
        inner.state = CircuitBreakerState::HalfOpen;
        inner.half_open_entered_at = Some(Instant::now());
        inner.consecutive_successes = 0;
        inner.half_open_concurrency_count = 0;

        let event = CircuitTransitionEvent {
            from_state: prev_state,
            to_state: CircuitBreakerState::HalfOpen,
            timestamp: SystemTime::now(),
            reason: reason.to_string(),
        };

        // Update metrics
        inner.metrics.state = CircuitBreakerState::HalfOpen;
        inner.metrics.last_transition_timestamp = Some(SystemTime::now());

        // Drop the lock before calling observers
        drop(inner);

        info!(
            "Circuit breaker '{}' transitioning to HalfOpen: {}",
            self.name, reason
        );
        self.notify_state_change(&event);
    }

    fn transition_to_closed(&self, reason: &str) {
        let mut inner = self.inner.write().unwrap();
        let prev_state = inner.state;
        inner.state = CircuitBreakerState::Closed;
        inner.opened_at = None;
        inner.half_open_entered_at = None;
        inner.consecutive_failures = 0;

        let event = CircuitTransitionEvent {
            from_state: prev_state,
            to_state: CircuitBreakerState::Closed,
            timestamp: SystemTime::now(),
            reason: reason.to_string(),
        };

        // Update metrics
        inner.metrics.state = CircuitBreakerState::Closed;
        inner.metrics.last_transition_timestamp = Some(SystemTime::now());

        // Drop the lock before calling observers
        drop(inner);

        info!(
            "Circuit breaker '{}' transitioning to Closed: {}",
            self.name, reason
        );
        self.notify_state_change(&event);
    }

    // Result recording helpers

    fn record_success(&self, duration: Duration) {
        let mut inner = self.inner.write().unwrap();
        inner.consecutive_successes += 1;
        inner.consecutive_failures = 0;

        // Update sliding window
        if inner.results_window.len() >= self.config.sliding_window_size {
            inner.results_window.pop_front();
        }
        inner.results_window.push_back(true);

        // Check if the call was slow
        let was_slow = if let Some(threshold) = self.config.slow_call_duration_threshold {
            duration >= threshold
        } else {
            false
        };

        // Update slow call window
        if inner.slow_call_window.len() >= self.config.sliding_window_size {
            inner.slow_call_window.pop_front();
        }
        inner.slow_call_window.push_back(was_slow);

        // Update metrics
        inner.metrics.total_requests += 1;
        inner.metrics.successful_requests += 1;
        inner.metrics.consecutive_successes = inner.consecutive_successes as u32;
        inner.metrics.consecutive_failures = 0;

        // Calculate rates
        self.update_rates(&mut inner);

        drop(inner);

        self.notify_operation_result(CircuitOperationType::Success, duration, None);
    }

    fn record_failure(&self, error: &DecrustError, duration: Duration) {
        let mut inner = self.inner.write().unwrap();
        inner.consecutive_failures += 1;
        inner.consecutive_successes = 0;

        // Update sliding window
        if inner.results_window.len() >= self.config.sliding_window_size {
            inner.results_window.pop_front();
        }
        inner.results_window.push_back(false);

        // Check if the call was slow (although it failed)
        let was_slow = if let Some(threshold) = self.config.slow_call_duration_threshold {
            duration >= threshold
        } else {
            false
        };

        // Update slow call window
        if inner.slow_call_window.len() >= self.config.sliding_window_size {
            inner.slow_call_window.pop_front();
        }
        inner.slow_call_window.push_back(was_slow);

        // Update metrics
        inner.metrics.total_requests += 1;
        inner.metrics.failed_requests += 1;
        inner.metrics.consecutive_failures = inner.consecutive_failures as u32;
        inner.metrics.consecutive_successes = 0;
        inner.metrics.last_error_timestamp = Some(SystemTime::now());

        // Calculate rates
        self.update_rates(&mut inner);

        let error_clone = error.clone(); // This requires Clone for DecrustError
        drop(inner);

        self.notify_operation_result(CircuitOperationType::Failure, duration, Some(&error_clone));
    }

    fn record_rejected(&self) {
        let mut inner = self.inner.write().unwrap();
        inner.metrics.total_requests += 1;
        inner.metrics.rejected_requests += 1;
        drop(inner);

        // Zero duration since operation was rejected
        self.notify_operation_result(CircuitOperationType::Rejected, Duration::from_secs(0), None);
    }

    fn record_timeout(&self) {
        let mut inner = self.inner.write().unwrap();
        inner.consecutive_failures += 1;
        inner.consecutive_successes = 0;

        // Update sliding window
        if inner.results_window.len() >= self.config.sliding_window_size {
            inner.results_window.pop_front();
        }
        inner.results_window.push_back(false);

        // Update metrics
        inner.metrics.total_requests += 1;
        inner.metrics.timeout_requests += 1;
        inner.metrics.consecutive_failures = inner.consecutive_failures as u32;
        inner.metrics.consecutive_successes = 0;
        inner.metrics.last_error_timestamp = Some(SystemTime::now());

        // Calculate rates
        self.update_rates(&mut inner);

        drop(inner);

        let timeout_error = DecrustError::Timeout {
            operation: format!("Operation in circuit breaker '{}'", self.name),
            duration: self.config.operation_timeout.unwrap_or_default(),
            backtrace: Backtrace::generate(),
        };

        self.notify_operation_result(
            CircuitOperationType::Timeout,
            self.config.operation_timeout.unwrap_or_default(),
            Some(&timeout_error),
        );
    }

    // Helper methods

    fn should_open_circuit(&self) -> bool {
        let inner = self.inner.read().unwrap();

        // Open if consecutive failures exceed threshold
        if inner.consecutive_failures >= self.config.failure_threshold {
            return true;
        }

        // Check failure rate if we have enough samples
        if inner.results_window.len() >= self.config.minimum_request_threshold_for_rate {
            let failure_count = inner
                .results_window
                .iter()
                .filter(|&&success| !success)
                .count();
            let failure_rate = failure_count as f64 / inner.results_window.len() as f64;

            if failure_rate >= self.config.failure_rate_threshold {
                return true;
            }
        }

        // Check slow call rate if configured
        if let (Some(threshold), true) = (
            self.config.slow_call_rate_threshold,
            !inner.slow_call_window.is_empty(),
        ) {
            let slow_count = inner.slow_call_window.iter().filter(|&&slow| slow).count();
            let slow_rate = slow_count as f64 / inner.slow_call_window.len() as f64;

            if slow_rate >= threshold {
                return true;
            }
        }

        false
    }

    fn should_count_as_failure(&self, error: &DecrustError) -> bool {
        // If there's a custom predicate, use that
        if let Some(predicate) = &self.config.error_predicate {
            return (predicate.as_ref())(error);
        }

        // By default, all errors count as failures
        true
    }

    fn update_rates(&self, inner: &mut InnerState) {
        if inner.results_window.is_empty() {
            inner.metrics.failure_rate_in_window = None;
        } else {
            let failure_count = inner
                .results_window
                .iter()
                .filter(|&&success| !success)
                .count();
            let failure_rate = failure_count as f64 / inner.results_window.len() as f64;
            inner.metrics.failure_rate_in_window = Some(failure_rate);
        }

        if inner.slow_call_window.is_empty() {
            inner.metrics.slow_call_rate_in_window = None;
        } else {
            let slow_count = inner.slow_call_window.iter().filter(|&&slow| slow).count();
            let slow_rate = slow_count as f64 / inner.slow_call_window.len() as f64;
            inner.metrics.slow_call_rate_in_window = Some(slow_rate);
        }
    }

    // Observer notification methods

    fn notify_state_change(&self, event: &CircuitTransitionEvent) {
        let observers = self.observers.lock().unwrap();
        for observer in &*observers {
            observer.on_state_change(&self.name, event);
        }
    }

    fn notify_operation_attempt(&self, state: CircuitBreakerState) {
        let observers = self.observers.lock().unwrap();
        for observer in &*observers {
            observer.on_operation_attempt(&self.name, state);
        }
    }

    fn notify_operation_result(
        &self,
        op_type: CircuitOperationType,
        duration: Duration,
        error: Option<&DecrustError>,
    ) {
        let observers = self.observers.lock().unwrap();
        for observer in &*observers {
            observer.on_operation_result(&self.name, op_type, duration, error);
        }
    }
    fn notify_reset(&self) {
        let observers = self.observers.lock().unwrap();
        for observer in &*observers {
            observer.on_reset(&self.name);
        }
    }
}

// decrust-promac-runtime/src/lib.rs
//
// This crate provides runtime components for the decrust-promac procedural macros.
// It fully encapsulates ALL of Decrust's capabilities for use with procedural macros.

//! # Decrust Procedural Macro Runtime
//!
//! This crate provides complete runtime support for the `decrust-promac` procedural macros.
//! It re-exports the entire Decrust framework, ensuring that all capabilities are available
//! when using the procedural macro interface.
//!
//! ## Complete Feature Coverage
//!
//! This runtime provides access to:
//! - **Error Handling**: Complete `DecrustError` system with all variants
//! - **Extension Traits**: All result and option extension traits
//! - **Circuit Breaker**: Full circuit breaker implementation with observers
//! - **Backtrace**: Advanced backtrace capture and analysis
//! - **Fix Generators**: All 29+ fix generators for automatic error correction
//! - **Reporting**: Comprehensive error reporting and formatting
//! - **Syntax Generation**: AST-aware code generation and templates
//! - **Type System**: Complete type definitions and utilities
//! - **Macros**: All convenience macros for error creation

// Re-export all modules with complete coverage
pub mod backtrace {
    //! Complete backtrace functionality from Decrust
    pub use ::decrust::backtrace::*;
}

pub mod circuit_breaker {
    //! Complete circuit breaker functionality from Decrust
    pub use ::decrust::circuit_breaker::*;
}

pub mod decrust {
    //! Complete fix generation and autocorrection functionality from Decrust
    pub use ::decrust::decrust::*;
}

pub mod reporter {
    //! Complete error reporting functionality from Decrust
    pub use ::decrust::reporter::*;
}

pub mod syntax {
    //! Complete syntax generation functionality from Decrust
    pub use ::decrust::syntax::*;
}

pub mod types {
    //! Complete type system from Decrust
    pub use ::decrust::types::*;
}

// Re-export ALL core types and traits at the root level for maximum convenience
pub use ::decrust::{
    // Backtrace types
    AsBacktrace,
    Backtrace,
    BacktraceCompat,
    BacktraceFrame,
    BacktraceProvider,
    BacktraceStatus,
    // Circuit breaker types
    CircuitBreaker,
    CircuitBreakerConfig,
    CircuitBreakerObserver,
    CircuitBreakerState,
    CircuitMetrics,
    CircuitOperationType,
    CircuitTransitionEvent,
    // Core error types
    DecrustError,
    DecrustOptionExt,
    DecrustOptionExtConvenience,
    // Extension traits (object-safe)
    DecrustResultExt,
    DecrustResultExtConvenience,
    GenerateImplicitData,
    InfallibleResultExt,

    Location,
    OptionalError,

    ThreadId,
    Timestamp,
};

// Re-export all macros (they're automatically available due to #[macro_export])
// But we document them here for clarity:
//
// Available macros from Decrust:
// - `implicit_data!()` - Generate implicit data for error context
// - `location!()` - Capture current location information
// - `error_context!(msg, err)` - Create error with rich context
// - `oops!(msg, source)` - Create "oops" errors with source
// - `validation_error!(field, msg)` - Create validation errors

// Utility functions for common operations
pub mod utils {
    //! Utility functions for common Decrust operations

    use std::time::Duration;

    /// Parse a duration string (e.g., "30s", "5m", "1h") into a Duration
    ///
    /// This is used by the procedural macros for parsing timeout configurations.
    pub fn parse_duration(s: &str) -> Result<Duration, String> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Empty duration string".to_string());
        }

        let (number_part, unit_part) = if let Some(pos) = s.find(|c: char| c.is_alphabetic()) {
            (&s[..pos], &s[pos..])
        } else {
            // No unit, assume seconds
            (s, "s")
        };

        let number: f64 = number_part
            .parse()
            .map_err(|_| format!("Invalid number in duration: {}", number_part))?;

        let duration = match unit_part.to_lowercase().as_str() {
            "ns" | "nanosecond" | "nanoseconds" => Duration::from_nanos(number as u64),
            "us" | "microsecond" | "microseconds" => Duration::from_micros(number as u64),
            "ms" | "millisecond" | "milliseconds" => Duration::from_millis(number as u64),
            "s" | "sec" | "second" | "seconds" => Duration::from_secs_f64(number),
            "m" | "min" | "minute" | "minutes" => Duration::from_secs_f64(number * 60.0),
            "h" | "hour" | "hours" => Duration::from_secs_f64(number * 3600.0),
            "d" | "day" | "days" => Duration::from_secs_f64(number * 86400.0),
            _ => return Err(format!("Unknown duration unit: {}", unit_part)),
        };

        Ok(duration)
    }

    /// Track performance metrics for a named operation
    ///
    /// This is used by the procedural macros for performance tracking.
    pub fn track_performance(operation_name: &str, duration: Duration) {
        // For now, just log to stderr. In a real implementation, this might
        // send metrics to a monitoring system.
        eprintln!("PERF: {} took {:?}", operation_name, duration);
    }
}

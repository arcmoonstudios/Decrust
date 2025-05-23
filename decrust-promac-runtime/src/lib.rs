// decrust-promac-runtime/src/lib.rs
//
// This crate provides runtime components for the decrust-promac procedural macros.

// Re-export all the necessary types and functions from decrust
// This allows users to import from decrust-promac-runtime instead of directly from decrust
pub mod backtrace {
    pub use ::decrust::backtrace::*;
    pub use ::decrust::backtrace::DecrustBacktrace as Backtrace;
}

pub mod circuit_breaker {
    pub use ::decrust::circuit_breaker::*;
}

pub mod decrust {
    pub use decrust::decrust::*;
}

pub mod reporter {
    pub use decrust::reporter::*;
}

pub mod syntax {
    pub use decrust::syntax::*;
}

pub mod types {
    pub use decrust::types::*;
}

// Re-export the DecrustError and OptionalError types at the root level for convenience
pub use ::decrust::DecrustError;
pub use ::decrust::OptionalError;

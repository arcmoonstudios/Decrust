//! Basic usage example for Decrust error handling framework
//!
//! This example demonstrates the core functionality of Decrust including:
//! - Creating different types of errors
//! - Error propagation and context
//! - Using the error reporting system
//! - Basic autocorrection suggestions

use decrust::backtrace::{BacktraceCompat, DecrustBacktrace as Backtrace};
use decrust::reporter::{ErrorReportConfig, ErrorReporter};
use decrust::types::ErrorReportFormat;
use decrust::{DecrustError, Result};
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("=== Decrust Error Handling Examples ===\n");

    // Example 1: I/O Error
    println!("1. I/O Error Example:");
    if let Err(e) = simulate_io_error() {
        println!("   Error: {}", e);
        print_error_details(&e);
    }

    // Example 2: Configuration Error
    println!("\n2. Configuration Error Example:");
    if let Err(e) = simulate_config_error() {
        println!("   Error: {}", e);
        print_error_details(&e);
    }

    // Example 3: Validation Error
    println!("\n3. Validation Error Example:");
    if let Err(e) = simulate_validation_error() {
        println!("   Error: {}", e);
        print_error_details(&e);
    }

    // Example 4: Network Error
    println!("\n4. Network Error Example:");
    if let Err(e) = simulate_network_error() {
        println!("   Error: {}", e);
        print_error_details(&e);
    }

    // Example 5: Error Reporting
    println!("\n5. Error Reporting Example:");
    demonstrate_error_reporting()?;

    println!("\n=== Examples completed successfully! ===");
    Ok(())
}

/// Simulates an I/O error
fn simulate_io_error() -> Result<()> {
    let path = PathBuf::from("/nonexistent/file.txt");
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "No such file or directory");

    Err(DecrustError::Io {
        source: io_error,
        path: Some(path),
        operation: "read_file".to_string(),
        backtrace: Backtrace::capture(),
    })
}

/// Simulates a configuration error
fn simulate_config_error() -> Result<()> {
    Err(DecrustError::Config {
        message: "Missing required configuration key 'database_url'".to_string(),
        path: Some(PathBuf::from("config.toml")),
        source: decrust::OptionalError::new(None),
        backtrace: Backtrace::capture(),
    })
}

/// Simulates a validation error
fn simulate_validation_error() -> Result<()> {
    Err(DecrustError::Validation {
        field: "email".to_string(),
        message: "Invalid email format: missing '@' symbol".to_string(),
        expected: None,
        actual: None,
        rule: None,
        backtrace: Backtrace::capture(),
    })
}

/// Simulates a network error
fn simulate_network_error() -> Result<()> {
    let network_error = std::io::Error::new(std::io::ErrorKind::TimedOut, "Connection timed out");

    Err(DecrustError::Network {
        source: Box::new(network_error),
        url: Some("https://api.example.com/users".to_string()),
        kind: "HTTP".to_string(),
        backtrace: Backtrace::capture(),
    })
}

/// Demonstrates error reporting functionality
fn demonstrate_error_reporting() -> Result<()> {
    let error = DecrustError::NotFound {
        resource_type: "User".to_string(),
        identifier: "user_123".to_string(),
        backtrace: Backtrace::capture(),
    };

    let reporter = ErrorReporter::new();

    // Report in different formats
    println!("   Plain format:");
    let plain_config = ErrorReportConfig {
        format: ErrorReportFormat::Plain,
        ..Default::default()
    };
    let plain_report = reporter.report_to_string(&error, &plain_config);
    println!("   {}", plain_report);

    println!("\n   JSON format:");
    let json_config = ErrorReportConfig {
        format: ErrorReportFormat::Json,
        ..Default::default()
    };
    let json_report = reporter.report_to_string(&error, &json_config);
    println!("   {}", json_report);

    Ok(())
}

/// Helper function to print error details
fn print_error_details(error: &DecrustError) {
    println!("   Category: {:?}", error.category());

    if let Some(source) = error.source() {
        println!("   Source: {}", source);
    }

    // Print backtrace if available
    if let Some(backtrace) = error.backtrace() {
        println!("   Backtrace available: {:?}", backtrace.status());
    }
}

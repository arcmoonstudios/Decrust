//! Error Usage Examples for Decrust Framework
//!
//! This example demonstrates modern error handling patterns including:
//! - The new Oops error variant for unexpected situations
//! - All 16 comprehensive error types in Decrust
//! - Error creation with rich context and backtraces
//! - Error propagation and chaining
//! - Extension traits for better error handling

use decrust::{
    DecrustError, DecrustResultExt, DecrustOptionExt, Result,
    oops, validation_error,
    backtrace::DecrustBacktrace as Backtrace,
};
use std::path::PathBuf;
use std::time::Duration;


fn main() -> Result<()> {
    println!("🚨 === DECRUST ERROR USAGE EXAMPLES === 🚨\n");

    // Example 1: The Modern Oops Error
    println!("1. 💥 The Modern Oops Error (for unexpected situations):");
    demonstrate_oops_error()?;

    // Example 2: Comprehensive Error Types
    println!("\n2. 📋 Comprehensive Error Types:");
    demonstrate_all_error_types()?;

    // Example 3: Error Propagation and Context
    println!("\n3. 🔗 Error Propagation and Context:");
    demonstrate_error_propagation()?;

    // Example 4: Extension Traits
    println!("\n4. 🎯 Extension Traits for Better Error Handling:");
    demonstrate_extension_traits()?;

    // Example 5: Error Categories and Classification
    println!("\n5. 🏷️  Error Categories and Classification:");
    demonstrate_error_categories()?;

    println!("\n✅ === ALL ERROR EXAMPLES COMPLETED! === ✅");
    Ok(())
}

/// Demonstrates the modern Oops error for unexpected situations
fn demonstrate_oops_error() -> Result<()> {
    println!("   Creating Oops errors for unexpected situations...");

    // Example 1: Simple Oops with message and source
    let source_error = std::io::Error::new(std::io::ErrorKind::Other, "Unexpected condition");
    let simple_oops = oops!("Something unexpected happened during processing", source_error);
    println!("   Simple Oops: {}", simple_oops);

    // Example 2: Oops created directly
    let another_source = std::io::Error::new(std::io::ErrorKind::Other, "State inconsistency");
    let detailed_oops = DecrustError::Oops {
        message: "Unexpected state: user exists but has no profile".to_string(),
        source: Box::new(another_source),
        backtrace: Backtrace::capture(),
    };
    println!("   Detailed Oops: {}", detailed_oops);
    println!("   Category: {:?}", detailed_oops.category());

    // Example 3: Using Oops in error chains
    let result: Result<String> = Err(detailed_oops);
    let chained = result.decrust_context_msg("Failed to validate user profile");

    match chained {
        Err(e) => println!("   Chained error: {}", e),
        Ok(_) => unreachable!(),
    }

    Ok(())
}

/// Demonstrates all 16 comprehensive error types
fn demonstrate_all_error_types() -> Result<()> {
    println!("   Showcasing key Decrust error types...");

    // 1. Validation Error
    let validation_err = validation_error!("email", "Invalid email format", suggestion: "Use format: user@domain.com");
    println!("   1. Validation: {}", validation_err);

    // 2. I/O Error
    let io_err = DecrustError::Io {
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        path: Some(PathBuf::from("config.toml")),
        operation: "read_config".to_string(),
        backtrace: Backtrace::capture(),
    };
    println!("   2. I/O: {}", io_err);

    // 3. Network Error
    let network_err = DecrustError::Network {
        source: Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Connection timeout")),
        url: Some("https://api.example.com/users".to_string()),
        kind: "HTTP".to_string(),
        backtrace: Backtrace::capture(),
    };
    println!("   3. Network: {}", network_err);

    // 4. NotFound Error
    let not_found_err = DecrustError::NotFound {
        resource_type: "User".to_string(),
        identifier: "user_12345".to_string(),
        backtrace: Backtrace::capture(),
    };
    println!("   4. NotFound: {}", not_found_err);

    // 5. Timeout Error
    let timeout_err = DecrustError::Timeout {
        operation: "database_query".to_string(),
        duration: Duration::from_secs(30),
        backtrace: Backtrace::capture(),
    };
    println!("   5. Timeout: {}", timeout_err);

    println!("   ... and many more error types available!");

    Ok(())
}

/// Demonstrates error propagation and context addition
fn demonstrate_error_propagation() -> Result<()> {
    println!("   Demonstrating error propagation with context...");

    // Simulate a chain of operations that can fail
    let result = process_user_data("invalid_email")
        .decrust_context_msg("Failed to process user registration")
        .decrust_context_msg("User onboarding pipeline failed");

    match result {
        Err(e) => {
            println!("   Final error with context chain: {}", e);
            println!("   Error category: {:?}", e.category());
        }
        Ok(_) => unreachable!(),
    }

    Ok(())
}

/// Demonstrates extension traits for better error handling
fn demonstrate_extension_traits() -> Result<()> {
    println!("   Using extension traits for cleaner error handling...");

    // Result extension traits
    let file_result: std::result::Result<String, std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied"));

    let enhanced_result = file_result
        .decrust_context_msg("Reading configuration file")
        .decrust_context_msg("Application startup failed");

    match enhanced_result {
        Err(e) => println!("   Enhanced error: {}", e),
        Ok(_) => unreachable!(),
    }

    // Option extension traits
    let maybe_config: Option<String> = None;
    let config_result = maybe_config
        .decrust_ok_or_missing_value("database_url")
        .decrust_context_msg("Configuration validation");

    match config_result {
        Err(e) => println!("   Option to error: {}", e),
        Ok(_) => unreachable!(),
    }

    Ok(())
}

/// Demonstrates error categories and classification
fn demonstrate_error_categories() -> Result<()> {
    println!("   Demonstrating error categories for classification...");

    let source_error = std::io::Error::new(std::io::ErrorKind::Other, "test");
    let errors = vec![
        oops!("Unexpected state", source_error),
        validation_error!("email", "Invalid format", suggestion: "Use format: user@domain.com"),
        DecrustError::NotFound {
            resource_type: "File".to_string(),
            identifier: "config.toml".to_string(),
            backtrace: Backtrace::capture(),
        },
    ];

    for (i, error) in errors.iter().enumerate() {
        println!("   Error {}: Category = {:?}, Message = {}",
                 i + 1, error.category(), error);
    }

    Ok(())
}

/// Helper function that simulates user data processing
fn process_user_data(email: &str) -> Result<String> {
    if !email.contains('@') {
        return Err(validation_error!("email", "Must contain @ symbol", suggestion: "Use format: user@domain.com"));
    }

    // Simulate another operation that could fail
    validate_email_domain(email)
}

/// Helper function that validates email domain
fn validate_email_domain(email: &str) -> Result<String> {
    if email.ends_with("@invalid.com") {
        let source = std::io::Error::new(std::io::ErrorKind::Other, "Domain validation failed");
        return Err(oops!("Domain validation service returned unexpected result", source));
    }

    Ok(format!("Validated email: {}", email))
}
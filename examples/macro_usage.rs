//! Macro Usage Examples for Decrust Framework
//!
//! This example demonstrates the powerful procedural macros including:
//! - The all-in-one `decrust!` macro from decrust_promac
//! - Convenience macros for error creation (oops!, validation_error!)
//! - Macro-based error handling patterns
//! - Integration with M.A.R.S., Luna⚛︎Ultima, and CodeMASTER v3
//!
//! Note: Due to Rust limitations, proc-macro crates cannot re-export types.
//! So we import the macro from decrust_promac and types from decrust-promac-runtime.

// Import the decrust! macro from decrust_promac
use decrust_promac::decrust;

// Import types from decrust-promac-runtime (which re-exports Decrust types)
use decrust_promac_runtime::{
    DecrustError,
    BacktraceCompat,  // Needed for .backtrace() method
    backtrace::DecrustBacktrace as Backtrace,
};

// Import macros and Result type from decrust (not re-exported by runtime)
use decrust::{Result, oops, validation_error};
use std::time::Duration;

fn main() -> Result<()> {
    println!("⚡ === DECRUST MACRO USAGE EXAMPLES === ⚡\n");

    // Example 1: The All-in-One decrust! Macro
    println!("1. 🎯 The All-in-One decrust! Macro:");
    demonstrate_decrust_macro()?;

    // Example 2: Convenience Error Creation Macros
    println!("\n2. 🛠️  Convenience Error Creation Macros:");
    demonstrate_error_macros()?;

    // Example 3: Macro-Based Error Handling Patterns
    println!("\n3. 🔄 Macro-Based Error Handling Patterns:");
    demonstrate_macro_patterns()?;

    // Example 4: Advanced Macro Features
    println!("\n4. 🚀 Advanced Macro Features:");
    demonstrate_advanced_macros()?;

    println!("\n🎉 === ALL MACRO EXAMPLES COMPLETED! === 🎉");
    Ok(())
}

/// Demonstrates the all-in-one decrust! macro
fn demonstrate_decrust_macro() -> Result<()> {
    println!("   Using the decrust! macro for comprehensive error handling...");

    // Example 1: Basic decrust! macro usage - wraps a function call
    let result = decrust!(process_file("important_data.txt"));
    match result {
        Ok(content) => println!("   ✅ File processed successfully: {}", content),
        Err(e) => println!("   ❌ Processing failed: {}", e),
    }

    // Example 2: decrust! with network request
    let result = decrust!(make_network_request("https://api.example.com/data"));
    match result {
        Ok(data) => println!("   ✅ Network request successful: {}", data),
        Err(e) => println!("   ❌ Network request failed: {}", e),
    }

    // Example 3: decrust! with complex expression
    let result = decrust!({
        let step1 = step_one_with_macro()?;
        let step2 = step_two_with_macro()?;
        step_three_with_macro()
    });
    match result {
        Ok(final_result) => println!("   ✅ Multi-step process completed: {}", final_result),
        Err(e) => println!("   ❌ Multi-step process failed: {}", e),
    }

    Ok(())
}

/// Demonstrates convenience error creation macros
fn demonstrate_error_macros() -> Result<()> {
    println!("   Creating errors with convenience macros...");

    // Example 1: oops! macro for unexpected situations
    let source_error = std::io::Error::new(std::io::ErrorKind::Other, "Unexpected condition");
    let oops_error = oops!("Something unexpected happened during user authentication", source_error);
    println!("   💥 Oops Error: {}", oops_error);

    // Example 2: validation_error! macro for validation failures
    let validation_err = validation_error!("password", "Password too weak", suggestion: "Use at least 8 characters with mixed case");
    println!("   🔍 Validation Error: {}", validation_err);

    // Example 3: Creating complex errors with macros
    let complex_error = create_complex_error_with_macro("user_123");
    println!("   🔧 Complex Error: {}", complex_error);

    Ok(())
}

/// Demonstrates macro-based error handling patterns
fn demonstrate_macro_patterns() -> Result<()> {
    println!("   Using macros for error handling patterns...");

    // Example 1: Macro-enhanced error propagation
    let result = decrust!({
        step_one_with_macro()?;
        step_two_with_macro()?;
        step_three_with_macro()
    });

    match result {
        Ok(final_result) => println!("   ✅ Multi-step process completed: {}", final_result),
        Err(e) => println!("   ❌ Multi-step process failed: {}", e),
    }

    Ok(())
}

/// Demonstrates advanced macro features
fn demonstrate_advanced_macros() -> Result<()> {
    println!("   Exploring advanced macro capabilities...");

    // Example 1: Macro with performance monitoring
    let result = decrust! {
        operation: "performance_critical_task",
        timeout: "5s",
        monitor_performance: true
    };

    match result {
        Ok(value) => println!("   ⚡ Performance task completed: {}", value),
        Err(e) => println!("   ⚠️  Performance task failed: {}", e),
    }

    // Example 2: Macro with custom error transformation
    let result = decrust! {
        operation: "data_transformation",
        transform_errors: true,
        body: {
            transform_data("input_data")
        }
    };

    match result {
        Ok(transformed) => println!("   🔄 Data transformation successful: {}", transformed),
        Err(e) => println!("   ❌ Data transformation failed: {}", e),
    }

    Ok(())
}

/// Helper function that creates a complex error using macros
fn create_complex_error_with_macro(user_id: &str) -> DecrustError {
    // This would typically use a custom macro, but for demonstration:
    DecrustError::NotFound {
        resource_type: "User".to_string(),
        identifier: user_id.to_string(),
        backtrace: Backtrace::capture(),
    }
}

/// Simulated step functions for macro patterns
fn step_one_with_macro() -> Result<String> {
    println!("     🔄 Executing step 1...");
    Ok("Step 1 completed".to_string())
}

fn step_two_with_macro() -> Result<String> {
    println!("     🔄 Executing step 2...");
    Ok("Step 2 completed".to_string())
}

fn step_three_with_macro() -> Result<String> {
    println!("     🔄 Executing step 3...");
    Ok("All steps completed successfully".to_string())
}

/// Simulated file processing function
fn process_file(filename: &str) -> Result<String> {
    if filename.contains("important") {
        Ok(format!("Processed file: {}", filename))
    } else {
        Err(DecrustError::NotFound {
            resource_type: "File".to_string(),
            identifier: filename.to_string(),
            backtrace: Backtrace::capture(),
        })
    }
}

/// Simulated network request function
fn make_network_request(url: &str) -> Result<String> {
    if url.contains("api.example.com") {
        Ok(format!("Data from {}", url))
    } else {
        Err(DecrustError::Network {
            source: Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Request timeout")),
            url: Some(url.to_string()),
            kind: "HTTP".to_string(),
            backtrace: Backtrace::capture(),
        })
    }
}

/// Simulated performance critical operation
fn performance_critical_operation() -> Result<String> {
    // Simulate some work
    std::thread::sleep(Duration::from_millis(100));
    Ok("Performance operation completed".to_string())
}

/// Simulated data transformation function
fn transform_data(input: &str) -> Result<String> {
    if input == "input_data" {
        Ok(format!("Transformed: {}", input.to_uppercase()))
    } else {
        let source = std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid input format");
        Err(oops!("Data transformation failed unexpectedly", source))
    }
}
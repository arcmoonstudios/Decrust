//! Advanced Decrust Framework Usage Examples
//!
//! This example demonstrates advanced Decrust capabilities including:
//! - Circuit breaker patterns for resilient systems
//! - Automatic error correction with M.A.R.S. system
//! - Advanced error reporting and diagnostics
//! - Performance tracking and monitoring
//! - Complex error handling workflows

use decrust::{
    DecrustError, DecrustResultExt, Result,
    circuit_breaker::{CircuitBreaker, CircuitBreakerConfig},
    decrust::{Decrust, AutocorrectableError},
    reporter::{ErrorReporter, ErrorReportConfig},
    types::ErrorReportFormat,
    backtrace::DecrustBacktrace as Backtrace,
};
use std::time::Duration;

fn main() -> Result<()> {
    println!("🚀 === ADVANCED DECRUST FRAMEWORK USAGE === 🚀\n");

    // Example 1: Circuit Breaker Pattern
    println!("1. ⚡ Circuit Breaker Pattern:");
    demonstrate_circuit_breaker()?;

    // Example 2: Automatic Error Correction (M.A.R.S.)
    println!("\n2. 🔧 Automatic Error Correction (M.A.R.S. System):");
    demonstrate_mars_autocorrection()?;

    // Example 3: Advanced Error Reporting
    println!("\n3. 📊 Advanced Error Reporting:");
    demonstrate_advanced_reporting()?;

    // Example 4: Performance Monitoring
    println!("\n4. 📈 Performance Monitoring:");
    demonstrate_performance_tracking()?;

    // Example 5: Complex Error Workflows
    println!("\n5. 🔄 Complex Error Workflows:");
    demonstrate_complex_workflows()?;

    println!("\n🎉 === ADVANCED EXAMPLES COMPLETED! === 🎉");
    Ok(())
}

/// Demonstrates circuit breaker pattern for resilient systems
fn demonstrate_circuit_breaker() -> Result<()> {
    println!("   Setting up circuit breaker for external service calls...");

    // Configure circuit breaker
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        reset_timeout: Duration::from_secs(10),
        operation_timeout: Some(Duration::from_secs(5)),
        ..Default::default()
    };

    let circuit_breaker = CircuitBreaker::new("external_service", config);
    println!("   Initial state: {:?}", circuit_breaker.state());

    // Simulate successful operations
    for i in 1..=2 {
        let result = circuit_breaker.execute(|| -> Result<String> {
            println!("     Executing operation {}...", i);
            Ok(format!("Success {}", i))
        });
        match result {
            Ok(value) => println!("   ✅ Operation {}: {}", i, value),
            Err(e) => println!("   ❌ Operation {}: {}", i, e),
        }
    }

    // Simulate failures to trip the circuit breaker
    for i in 3..=5 {
        let result = circuit_breaker.execute(|| -> Result<String> {
            println!("     Executing failing operation {}...", i);
            Err(DecrustError::Network {
                source: Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Service timeout")),
                url: Some("https://api.example.com".to_string()),
                kind: "HTTP".to_string(),
                backtrace: Backtrace::capture(),
            })
        });
        match result {
            Ok(value) => println!("   ✅ Operation {}: {}", i, value),
            Err(e) => println!("   ❌ Operation {}: {}", i, e),
        }
        println!("     Circuit breaker state: {:?}", circuit_breaker.state());
    }

    Ok(())
}

/// Demonstrates M.A.R.S. (Mathematical Analysis & Recursive Systematic) error correction
fn demonstrate_mars_autocorrection() -> Result<()> {
    println!("   Initializing M.A.R.S. autocorrection system...");

    let decrust_engine = Decrust::new();

    // Create an error that can be auto-corrected
    let error = DecrustError::NotFound {
        resource_type: "File".to_string(),
        identifier: "/config/app.toml".to_string(),
        backtrace: Backtrace::capture(),
    };

    println!("   Original error: {}", error);
    println!("   Error category: {:?}", error.category());

    // Extract parameters for analysis
    let params = decrust_engine.extract_parameters(&error);
    println!("   Extracted parameters: confidence = {:.2}", params.confidence);

    // Get autocorrection suggestion
    if let Some(suggestion) = error.suggest_autocorrection(&decrust_engine, None) {
        println!("   🔧 M.A.R.S. Suggestion:");
        println!("     Description: {}", suggestion.description);
        println!("     Fix type: {:?}", suggestion.fix_type);
        println!("     Confidence: {:.1}%", suggestion.confidence * 100.0);

        if !suggestion.commands_to_apply.is_empty() {
            println!("     Commands to apply:");
            for cmd in &suggestion.commands_to_apply {
                println!("       $ {}", cmd);
            }
        }
    } else {
        println!("   ℹ️  No automatic correction available for this error type");
    }

    Ok(())
}

/// Demonstrates advanced error reporting with multiple formats
fn demonstrate_advanced_reporting() -> Result<()> {
    println!("   Creating comprehensive error reports...");

    let error = DecrustError::Validation {
        field: "user_email".to_string(),
        message: "Email format validation failed".to_string(),
        expected: Some("user@domain.com".to_string()),
        actual: Some("invalid-email".to_string()),
        rule: Some("RFC 5322 compliant email".to_string()),
        backtrace: Backtrace::capture(),
    };

    let reporter = ErrorReporter::new();

    // Plain text report
    let plain_config = ErrorReportConfig {
        format: ErrorReportFormat::Plain,
        include_backtrace: true,
        include_source_chain: true,
        max_chain_depth: Some(10),
        ..Default::default()
    };
    let plain_report = reporter.report_to_string(&error, &plain_config);
    println!("   📄 Plain Text Report:");
    println!("{}", plain_report);

    // JSON report for structured logging
    let json_config = ErrorReportConfig {
        format: ErrorReportFormat::Json,
        include_backtrace: false,
        include_source_chain: true,
        ..Default::default()
    };
    let json_report = reporter.report_to_string(&error, &json_config);
    println!("\n   📊 JSON Report:");
    println!("{}", json_report);

    Ok(())
}

/// Demonstrates performance tracking and monitoring
fn demonstrate_performance_tracking() -> Result<()> {
    println!("   Tracking performance metrics...");

    let start = std::time::Instant::now();

    // Simulate some work
    std::thread::sleep(Duration::from_millis(100));

    let duration = start.elapsed();
    println!("   ⏱️  Operation completed in: {:?}", duration);

    // Create a performance-related error
    if duration > Duration::from_millis(50) {
        let timeout_error = DecrustError::Timeout {
            operation: "data_processing".to_string(),
            duration,
            backtrace: Backtrace::capture(),
        };
        println!("   ⚠️  Performance warning: {}", timeout_error);
    }

    Ok(())
}

/// Demonstrates complex error handling workflows
fn demonstrate_complex_workflows() -> Result<()> {
    println!("   Executing complex error handling workflow...");

    // Simulate a multi-step process with error recovery
    let result = step_one()
        .or_else(|e| {
            println!("     Step 1 failed, attempting recovery: {}", e);
            recover_from_step_one_failure()
        })
        .and_then(|_| step_two())
        .and_then(|_| step_three())
        .decrust_context_msg("Multi-step workflow failed");

    match result {
        Ok(_) => println!("   ✅ Workflow completed successfully"),
        Err(e) => {
            println!("   ❌ Workflow failed: {}", e);
            println!("   📋 Error category: {:?}", e.category());
        }
    }

    Ok(())
}

/// Simulated workflow step
fn step_one() -> Result<String> {
    // Simulate failure
    Err(DecrustError::NotFound {
        resource_type: "Configuration".to_string(),
        identifier: "database_config".to_string(),
        backtrace: Backtrace::capture(),
    })
}

/// Recovery function for step one
fn recover_from_step_one_failure() -> Result<String> {
    println!("     🔄 Applying recovery strategy...");
    Ok("Recovered configuration".to_string())
}

/// Simulated workflow step
fn step_two() -> Result<String> {
    println!("     ✅ Step 2 completed");
    Ok("Step 2 result".to_string())
}

/// Simulated workflow step
fn step_three() -> Result<String> {
    println!("     ✅ Step 3 completed");
    Ok("Final result".to_string())
}
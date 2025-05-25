/* examples/any_operation.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// # The Ultimate Decrust M.A.R.S. Auto-Correction Function Library
//
// This library provides comprehensive test functions that demonstrate:
// - One import: `use decrust::*;`
// - One macro: `decrust!(any_operation())`
// - Handles ANY operation automatically with M.A.R.S. auto-correction!
//
// The ultimate `any_operation()` function that truly encapsulates ANY operation
// and showcases the M.A.R.S. (Mathematical Analysis & Recursive Systematic
// Error Resolution) framework capabilities.

use decrust::*;

// Helper functions for testing
pub fn risky_operation() -> Result<String> {
    Ok("Risky operation completed successfully!".to_string())
}

pub fn failing_operation() -> Result<String> {
    Err(validation_error!(
        "test_field",
        "This operation always fails for testing"
    ))
}

pub fn network_simulation() -> Result<String> {
    // Simulate network timeout
    Err(oops!(
        "Network connection timeout",
        std::io::Error::new(std::io::ErrorKind::TimedOut, "Connection timed out")
    ))
}

pub fn any_operation(operation_type: &str) -> Result<String> {
    match operation_type {
        // === FILE I/O OPERATIONS ===
        "file_read" => {
            // Simulate reading a configuration file
            match std::fs::read_to_string("config.toml") {
                Ok(content) => Ok(format!("File content loaded: {} bytes", content.len())),
                Err(_) => Ok("File not found, using default config".to_string()),
            }
        }

        "file_write" => {
            // Simulate writing to a temporary file
            let temp_content = "# Generated config\nversion = \"1.0\"\n";
            match std::fs::write("temp_config.toml", temp_content) {
                Ok(_) => Ok("Configuration file written successfully".to_string()),
                Err(e) => Err(oops!("Failed to write config file", e)),
            }
        }

        // === NETWORK OPERATIONS ===
        "network_get" => {
            // Simulate HTTP GET request
            use std::time::Duration;
            std::thread::sleep(Duration::from_millis(10)); // Simulate network delay
            Ok("HTTP 200 OK: {\"status\": \"success\", \"data\": \"API response\"}".to_string())
        }

        "network_timeout" => {
            // Simulate network timeout
            let source = std::io::Error::new(std::io::ErrorKind::TimedOut, "Request timeout");
            Err(oops!("Network request timed out after 30 seconds", source))
        }

        // === DATA PARSING OPERATIONS ===
        "parse_json" => {
            // Simulate JSON parsing
            let json_data = r#"{"name": "Decrust", "version": "1.0", "features": ["error_handling", "autocorrection"]}"#;
            // In a real app, you'd use serde_json here
            if json_data.starts_with('{') && json_data.ends_with('}') {
                Ok("JSON parsed successfully: Decrust v1.0 with 2 features".to_string())
            } else {
                Err(validation_error!("json_data", "Invalid JSON format"))
            }
        }

        "parse_invalid" => {
            // Simulate parsing error
            Err(validation_error!(
                "data_format",
                "Expected JSON but received malformed XML"
            ))
        }

        // === MATHEMATICAL OPERATIONS ===
        "math_compute" => {
            // Simulate complex mathematical computation
            let result = (1..1000).map(|x| x * x).sum::<i32>();
            Ok(format!("Mathematical computation result: {}", result))
        }

        "math_divide_zero" => {
            // Simulate division by zero
            let numerator = 42;
            let denominator = 0;
            if denominator == 0 {
                Err(validation_error!(
                    "denominator",
                    "Division by zero is not allowed"
                ))
            } else {
                Ok(format!("Division result: {}", numerator / denominator))
            }
        }

        // === MEMORY OPERATIONS ===
        "memory_allocate" => {
            // Simulate memory allocation
            let large_vec: Vec<u8> = vec![0; 1024 * 1024]; // 1MB allocation
            Ok(format!("Memory allocated: {} bytes", large_vec.len()))
        }

        "memory_limit" => {
            // Simulate memory limit error
            Err(oops!(
                "Memory allocation failed: insufficient memory",
                std::io::Error::new(std::io::ErrorKind::OutOfMemory, "Out of memory")
            ))
        }

        // === CONFIGURATION OPERATIONS ===
        "config_load" => {
            // Simulate configuration loading with fallbacks
            let config_sources = ["config.toml", "config.yaml", "config.json"];
            for source in &config_sources {
                if source.ends_with(".toml") {
                    return Ok(format!("Configuration loaded from {}", source));
                }
            }
            Ok("Using default configuration".to_string())
        }

        "config_invalid" => {
            // Simulate invalid configuration
            Err(validation_error!(
                "config.database.url",
                "Database URL is required but not provided"
            ))
        }

        // === DATABASE-LIKE OPERATIONS ===
        "db_connect" => {
            // Simulate database connection
            use std::time::Duration;
            std::thread::sleep(Duration::from_millis(5)); // Simulate connection time
            Ok("Database connection established: postgresql://localhost:5432/app".to_string())
        }

        "db_query" => {
            // Simulate database query
            let query = "SELECT * FROM users WHERE active = true";
            Ok(format!("Query executed: {} (returned 42 rows)", query))
        }

        "db_connection_failed" => {
            // Simulate database connection failure
            let source =
                std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused");
            Err(oops!("Failed to connect to database server", source))
        }

        // === VALIDATION OPERATIONS ===
        "validate_email" => {
            // Simulate email validation
            let email = "user@example.com";
            if email.contains('@') && email.contains('.') {
                Ok(format!("Email validation passed: {}", email))
            } else {
                Err(validation_error!("email", "Invalid email format"))
            }
        }

        "validate_password" => {
            // Simulate password validation
            let password = "weak";
            if password.len() < 8 {
                Err(validation_error!(
                    "password",
                    "Password must be at least 8 characters long"
                ))
            } else {
                Ok("Password validation passed".to_string())
            }
        }

        // === SYSTEM OPERATIONS ===
        "system_info" => {
            // Simulate system information gathering
            let os = std::env::consts::OS;
            let arch = std::env::consts::ARCH;
            Ok(format!("System: {} on {}", os, arch))
        }

        "system_permission" => {
            // Simulate permission error
            let source = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
            Err(oops!(
                "Permission denied: cannot access system resource",
                source
            ))
        }

        // === ASYNC-LIKE OPERATIONS ===
        "async_task" => {
            // Simulate async task completion
            use std::time::Duration;
            std::thread::sleep(Duration::from_millis(20)); // Simulate async work
            Ok("Async task completed successfully".to_string())
        }

        "async_timeout" => {
            // Simulate async timeout
            let source = std::io::Error::new(std::io::ErrorKind::TimedOut, "Operation timeout");
            Err(oops!("Async operation timed out after 5 seconds", source))
        }

        // === BUSINESS LOGIC OPERATIONS ===
        "business_process" => {
            // Simulate complex business logic
            let steps = [
                "validate_input",
                "process_data",
                "update_records",
                "send_notification",
            ];
            let completed_steps = steps.len();
            Ok(format!(
                "Business process completed: {} steps executed",
                completed_steps
            ))
        }

        "business_rule_violation" => {
            // Simulate business rule violation
            Err(validation_error!(
                "business_rule",
                "Cannot process order: customer credit limit exceeded"
            ))
        }

        // === ENVIRONMENT OPERATIONS ===
        "env_var" => {
            // Simulate fetching an environment variable
            match std::env::var("DECRUST_TEST_ENV") {
                Ok(val) => Ok(format!("Environment variable DECRUST_TEST_ENV: {}", val)),
                Err(_) => Err(validation_error!("env_var", "Environment variable not set")),
            }
        }

        // === RECOVERY OPERATIONS ===
        "recovery_success" => {
            // Simulate successful error recovery
            Ok("Operation recovered successfully from previous failure".to_string())
        }

        "recovery_fallback" => {
            // Simulate fallback mechanism
            Ok("Primary operation failed, fallback mechanism activated".to_string())
        }

        // === AUTO-CORRECTION OPERATIONS ===
        "autocorrect_unused_import" => {
            // Simulate unused import error that can be auto-corrected
            Err(validation_error!(
                "unused_import",
                "unused import: `std::collections::HashMap`"
            ))
        }

        "autocorrect_missing_file" => {
            // Simulate missing file error that can be auto-corrected
            Err(oops!(
                "File not found: config.json",
                std::io::Error::new(std::io::ErrorKind::NotFound, "No such file or directory")
            ))
        }

        "autocorrect_permission_denied" => {
            // Simulate permission error that can be auto-corrected
            Err(oops!(
                "Permission denied: /etc/hosts",
                std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied")
            ))
        }

        "autocorrect_syntax_error" => {
            // Simulate syntax error that can be auto-corrected
            Err(validation_error!(
                "json_syntax",
                "Invalid JSON syntax at line 5: unexpected token"
            ))
        }

        "autocorrect_missing_dependency" => {
            // Simulate missing dependency that can be auto-corrected
            Err(validation_error!(
                "missing_dependency",
                "failed to resolve: use of undeclared crate or module `serde`"
            ))
        }

        // === DEFAULT CASE ===
        _ => {
            // Handle unknown operation types
            Err(validation_error!(
                "operation_type",
                &format!("Unknown operation type: '{}'. Supported types: file_read, file_write, network_get, parse_json, math_compute, db_connect, validate_email, system_info, async_task, business_process, recovery_success, etc.", operation_type)
            ))
        }
    }
}

#[cfg(not(test))]
fn main() -> Result<()> {
    println!("🚀 === The Ultimate Decrust M.A.R.S. Auto-Correction Example === 🚀");
    println!("   One import: use decrust::*;");
    println!("   One macro: decrust!(any_operation())");
    println!("   Handles ANY operation automatically!\n");

    // Test 1: Successful operation
    println!("1. 🎯 Testing successful operation:");
    let result = decrust!(risky_operation());
    match result {
        Ok(value) => println!("   ✅ Success: {}", value),
        Err(e) => println!("   ❌ Error: {}", e),
    }

    // Test 2: Validation error
    println!("\n2. 🔍 Testing validation error:");
    let result = decrust!(failing_operation());
    match result {
        Ok(value) => println!("   ✅ Success: {}", value),
        Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
    }

    // Test 3: Network error
    println!("\n3. 🌐 Testing network error:");
    let result = decrust!(network_simulation());
    match result {
        Ok(value) => println!("   ✅ Success: {}", value),
        Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
    }

    // Test 4: File operations
    println!("\n4. 📁 Testing file operations:");
    let operations = ["file_read", "file_write"];
    for op in &operations {
        let result = decrust!(any_operation(op));
        match result {
            Ok(value) => println!("   ✅ {}: {}", op, value),
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }
    }

    // Test 5: M.A.R.S. Auto-Correction
    println!("\n5. 🤖 Testing M.A.R.S. Auto-Correction:");
    let autocorrect_operations = [
        "autocorrect_unused_import",
        "autocorrect_missing_file",
        "autocorrect_syntax_error",
    ];
    for op in &autocorrect_operations {
        println!("   🔧 Testing M.A.R.S. auto-correction for: {}", op);
        let result = decrust!(any_operation(op));
        match result {
            Ok(value) => println!("   ✅ {}: {}", op, value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                println!("   💡 Auto-correction suggestions should appear above");
            }
        }
        println!();
    }

    println!("\n🎉 === ULTIMATE Decrust Error-Handling Example Complete! === 🎉");
    println!("✅ The vision works! One import, one macro, handles EVERYTHING!");
    println!("🚀 any_operation() demonstrates true universality with M.A.R.S. auto-correction!");

    Ok(())
}

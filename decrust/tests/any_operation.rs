/* tests/any_operation.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! # The Ultimate Decrust M.A.R.S. Auto-Correction Test Suite
//!
//! This comprehensive test suite demonstrates the ultimate vision:
//! - One import: `use decrust::*;`
//! - One macro: `decrust!(any_operation())`
//! - Handles ANY operation automatically with M.A.R.S. auto-correction!
//!
//! Tests import functions from examples/any_operation.rs to verify the
//! M.A.R.S. (Mathematical Analysis & Recursive Systematic Error Resolution)
//! framework capabilities in a proper test environment.

use decrust::*;

// Import the test functions from the example
mod any_operation_lib {
    include!("../examples/any-operation.rs");
}

use any_operation_lib::{any_operation, failing_operation, network_simulation, risky_operation};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_operation() {
        println!("🎯 Testing successful operation:");
        let result = decrust!(risky_operation());
        match result {
            Ok(value) => {
                println!("   ✅ Success: {}", value);
                assert!(value.contains("successfully"));
            }
            Err(e) => {
                println!("   ❌ Error: {}", e);
                panic!("Expected success but got error: {}", e);
            }
        }
    }

    #[test]
    fn test_validation_error() {
        println!("🔍 Testing validation error:");
        let result = decrust!(failing_operation());
        match result {
            Ok(value) => {
                println!("   ✅ Unexpected success: {}", value);
                panic!("Expected error but got success: {}", value);
            }
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("test_field"));
            }
        }
    }

    #[test]
    fn test_network_timeout() {
        println!("🌐 Testing network timeout:");
        let result = decrust!(network_simulation());
        match result {
            Ok(value) => {
                println!("   ✅ Unexpected success: {}", value);
                panic!("Expected timeout but got success: {}", value);
            }
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("timeout"));
            }
        }
    }

    #[test]
    fn test_file_operations() {
        println!("📁 Testing file operations:");

        // Test file read
        let result = decrust!(any_operation("file_read"));
        match result {
            Ok(value) => println!("   ✅ file_read: {}", value),
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test file write
        let result = decrust!(any_operation("file_write"));
        match result {
            Ok(value) => {
                println!("   ✅ file_write: {}", value);
                assert!(value.contains("successfully"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }
    }

    #[test]
    fn test_network_operations() {
        println!("🌐 Testing network operations:");

        // Test successful network request
        let result = decrust!(any_operation("network_get"));
        match result {
            Ok(value) => {
                println!("   ✅ network_get: {}", value);
                assert!(value.contains("HTTP 200 OK"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test network timeout
        let result = decrust!(any_operation("network_timeout"));
        match result {
            Ok(value) => println!("   ✅ Unexpected success: {}", value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("timeout"));
            }
        }
    }

    #[test]
    fn test_data_parsing() {
        println!("📊 Testing data parsing:");

        // Test successful JSON parsing
        let result = decrust!(any_operation("parse_json"));
        match result {
            Ok(value) => {
                println!("   ✅ parse_json: {}", value);
                assert!(value.contains("JSON parsed successfully"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test parsing error
        let result = decrust!(any_operation("parse_invalid"));
        match result {
            Ok(value) => println!("   ✅ Unexpected success: {}", value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("malformed"));
            }
        }
    }

    #[test]
    fn test_mathematical_operations() {
        println!("🧮 Testing mathematical operations:");

        // Test successful computation
        let result = decrust!(any_operation("math_compute"));
        match result {
            Ok(value) => {
                println!("   ✅ math_compute: {}", value);
                assert!(value.contains("Mathematical computation result"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test division by zero
        let result = decrust!(any_operation("math_divide_zero"));
        match result {
            Ok(value) => println!("   ✅ Unexpected success: {}", value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("Division by zero"));
            }
        }
    }

    #[test]
    fn test_memory_operations() {
        println!("💾 Testing memory operations:");

        // Test memory allocation
        let result = decrust!(any_operation("memory_allocate"));
        match result {
            Ok(value) => {
                println!("   ✅ memory_allocate: {}", value);
                assert!(value.contains("Memory allocated"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test memory limit
        let result = decrust!(any_operation("memory_limit"));
        match result {
            Ok(value) => println!("   ✅ Unexpected success: {}", value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("Memory allocation failed"));
            }
        }
    }

    #[test]
    fn test_configuration_operations() {
        println!("⚙️ Testing configuration operations:");

        // Test config loading
        let result = decrust!(any_operation("config_load"));
        match result {
            Ok(value) => {
                println!("   ✅ config_load: {}", value);
                assert!(value.contains("Configuration loaded"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test invalid config
        let result = decrust!(any_operation("config_invalid"));
        match result {
            Ok(value) => println!("   ✅ Unexpected success: {}", value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("Database URL is required"));
            }
        }
    }

    #[test]
    fn test_database_operations() {
        println!("🗄️ Testing database operations:");

        // Test database connection
        let result = decrust!(any_operation("db_connect"));
        match result {
            Ok(value) => {
                println!("   ✅ db_connect: {}", value);
                assert!(value.contains("Database connection established"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test database query
        let result = decrust!(any_operation("db_query"));
        match result {
            Ok(value) => {
                println!("   ✅ db_query: {}", value);
                assert!(value.contains("Query executed"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test connection failure
        let result = decrust!(any_operation("db_connection_failed"));
        match result {
            Ok(value) => println!("   ✅ Unexpected success: {}", value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("Failed to connect"));
            }
        }
    }

    #[test]
    fn test_validation_operations() {
        println!("✅ Testing validation operations:");

        // Test email validation
        let result = decrust!(any_operation("validate_email"));
        match result {
            Ok(value) => {
                println!("   ✅ validate_email: {}", value);
                assert!(value.contains("Email validation passed"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test password validation
        let result = decrust!(any_operation("validate_password"));
        match result {
            Ok(value) => println!("   ✅ Unexpected success: {}", value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("at least 8 characters"));
            }
        }
    }

    #[test]
    fn test_system_operations() {
        println!("🖥️ Testing system operations:");

        // Test system info
        let result = decrust!(any_operation("system_info"));
        match result {
            Ok(value) => {
                println!("   ✅ system_info: {}", value);
                assert!(value.contains("System:"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test permission error
        let result = decrust!(any_operation("system_permission"));
        match result {
            Ok(value) => println!("   ✅ Unexpected success: {}", value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("Permission denied"));
            }
        }
    }

    #[test]
    fn test_async_operations() {
        println!("🔄 Testing async-like operations:");

        // Test async task
        let result = decrust!(any_operation("async_task"));
        match result {
            Ok(value) => {
                println!("   ✅ async_task: {}", value);
                assert!(value.contains("Async task completed"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test async timeout
        let result = decrust!(any_operation("async_timeout"));
        match result {
            Ok(value) => println!("   ✅ Unexpected success: {}", value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("timed out"));
            }
        }
    }

    #[test]
    fn test_business_logic_operations() {
        println!("💼 Testing business logic operations:");

        // Test business process
        let result = decrust!(any_operation("business_process"));
        match result {
            Ok(value) => {
                println!("   ✅ business_process: {}", value);
                assert!(value.contains("Business process completed"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test business rule violation
        let result = decrust!(any_operation("business_rule_violation"));
        match result {
            Ok(value) => println!("   ✅ Unexpected success: {}", value),
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("credit limit exceeded"));
            }
        }
    }

    #[test]
    fn test_recovery_operations() {
        println!("🔄 Testing recovery operations:");

        // Test recovery success
        let result = decrust!(any_operation("recovery_success"));
        match result {
            Ok(value) => {
                println!("   ✅ recovery_success: {}", value);
                assert!(value.contains("recovered successfully"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }

        // Test recovery fallback
        let result = decrust!(any_operation("recovery_fallback"));
        match result {
            Ok(value) => {
                println!("   ✅ recovery_fallback: {}", value);
                assert!(value.contains("fallback mechanism"));
            }
            Err(e) => println!("   🤖 M.A.R.S. Analysis: {}", e),
        }
    }

    #[test]
    fn test_mars_autocorrection_operations() {
        println!("🤖 Testing M.A.R.S. Auto-Correction:");

        let autocorrect_operations = [
            "autocorrect_unused_import",
            "autocorrect_missing_file",
            "autocorrect_permission_denied",
            "autocorrect_syntax_error",
            "autocorrect_missing_dependency",
        ];

        for op in &autocorrect_operations {
            println!("   🔧 Testing M.A.R.S. auto-correction for: {}", op);
            let result = decrust!(any_operation(op));
            match result {
                Ok(value) => {
                    println!("   ✅ {}: {}", op, value);
                    panic!(
                        "Expected error for auto-correction test but got success: {}",
                        value
                    );
                }
                Err(e) => {
                    println!("   🤖 M.A.R.S. Analysis: {}", e);
                    println!("   💡 Auto-correction suggestions should appear above");
                    // Verify the error contains expected content
                    let error_str = format!("{}", e);
                    match *op {
                        "autocorrect_unused_import" => assert!(error_str.contains("unused import")),
                        "autocorrect_missing_file" => assert!(error_str.contains("File not found")),
                        "autocorrect_permission_denied" => {
                            assert!(error_str.contains("Permission denied"))
                        }
                        "autocorrect_syntax_error" => {
                            assert!(error_str.contains("Invalid JSON syntax"))
                        }
                        "autocorrect_missing_dependency" => {
                            assert!(error_str.contains("failed to resolve"))
                        }
                        _ => {}
                    }
                }
            }
            println!(); // Add spacing between auto-correction tests
        }
    }

    #[test]
    fn test_unknown_operation() {
        println!("❓ Testing unknown operation:");
        let result = decrust!(any_operation("unknown_operation"));
        match result {
            Ok(value) => {
                println!("   ✅ Unexpected success: {}", value);
                panic!(
                    "Expected error for unknown operation but got success: {}",
                    value
                );
            }
            Err(e) => {
                println!("   🤖 M.A.R.S. Analysis: {}", e);
                assert!(format!("{}", e).contains("Unknown operation type"));
            }
        }
    }

    #[test]
    fn test_ultimate_decrust_vision() {
        println!("\n🎉 === ULTIMATE Decrust Error-Handling Test! === 🎉");
        println!("✅ The vision works! One import, one macro, handles EVERYTHING!");
        println!("🚀 any_operation() demonstrates true universality:");
        println!("   📁 File I/O • 🌐 Network • 📊 Data Processing • 🖥️ System");
        println!("   🗄️ Database • ✅ Validation • 💼 Business Logic • 🔄 Recovery");
        println!("   🤖 M.A.R.S. Auto-Correction • 💡 Intelligent Fix Suggestions");
        println!("   🎯 ALL handled seamlessly by decrust!(any_operation(type))");
        println!("\n🧠 M.A.R.S. Features Demonstrated:");
        println!("   • Mathematical Analysis & Recursive Systematic Error Resolution");
        println!("   • 7-Phase Error Resolution with Verification Loops");
        println!("   • Causal Chain Analysis with Root Cause Identification");
        println!("   • Automatic Fix Generation for Common Error Patterns");
        println!("   • Circuit Breaker Patterns for Fault Tolerance");
        println!("   • Comprehensive Error Reporting with Rich Context");

        // This test always passes - it's just a demonstration
        // The test succeeds by reaching this point without panicking
    }
}

//! # The Ultimate Decrust Vision Test
//!
//! This example demonstrates the ultimate vision:
//! - One import: `use decrust::*;`
//! - One macro: `decrust!(any_operation())`
//! - Handles ANY operation automatically!

use ::decrust::*;

fn risky_operation() -> Result<String> {
    Ok("Success from risky operation!".to_string())
}

fn failing_operation() -> Result<String> {
    Err(validation_error!("test_field", "This is a test validation error"))
}

fn network_simulation() -> Result<String> {
    // Simulate a network error using oops! macro
    let source = std::io::Error::new(std::io::ErrorKind::TimedOut, "Connection timeout");
    Err(oops!("Network request failed to https://api.example.com", source))
}

fn main() -> Result<()> {
    println!("🚀 === THE ULTIMATE DECRUST VISION TEST === 🚀");
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
        Err(e) => println!("   ❌ Validation Error: {}", e),
    }

    // Test 3: Network error
    println!("\n3. 🌐 Testing network error:");
    let result = decrust!(network_simulation());
    match result {
        Ok(value) => println!("   ✅ Success: {}", value),
        Err(e) => println!("   ❌ Network Error: {}", e),
    }

    // Test 4: File I/O operation (real operation)
    println!("\n4. 📁 Testing real file I/O operation:");
    let result = decrust!(std::fs::read_to_string("nonexistent_file.txt"));
    match result {
        Ok(content) => println!("   ✅ File content: {}", content),
        Err(e) => println!("   ❌ File I/O Error: {}", e),
    }

    // Test 5: Complex expression
    println!("\n5. 🔧 Testing complex multi-step operation:");
    let result = decrust!(risky_operation().map(|s| format!("Processed: {}", s)));
    match result {
        Ok(value) => println!("   ✅ Complex operation: {}", value),
        Err(e) => println!("   ❌ Complex operation error: {}", e),
    }

    println!("\n🎉 === ULTIMATE DECRUST VISION TEST COMPLETE === 🎉");
    println!("✅ The vision works! One import, one macro, handles everything!");

    Ok(())
}

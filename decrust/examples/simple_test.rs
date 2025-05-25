//! Simple test of the ultimate decrust vision
//!
//! One import: use decrust::*;
//! One macro: decrust!(any_operation())

use ::decrust::*;

fn simple_operation() -> Result<String> {
    Ok("Hello from Decrust!".to_string())
}

fn main() -> Result<()> {
    println!("🚀 Testing Ultimate Decrust Vision!");

    // Test the decrust! macro with a simple operation
    let result = decrust!(simple_operation());

    match result {
        Ok(value) => println!("✅ Success: {}", value),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Test with a file operation
    let result = decrust!(std::fs::read_to_string("README.md"));
    match result {
        Ok(content) => println!("✅ File read successfully! Length: {}", content.len()),
        Err(e) => println!("❌ File error: {}", e),
    }

    println!("🎉 Ultimate Decrust Vision works!");
    Ok(())
}

//! Test fixtures and mock data for decrust-crates tests
//! Provides reusable test data and helper functions

/// Mock Cargo.toml configurations for testing
#[allow(dead_code)]
pub struct CargoTomlFixtures;

#[allow(dead_code)]
impl CargoTomlFixtures {
    /// Simple project with basic dependencies
    pub fn simple_project() -> &'static str {
        r#"
[package]
name = "simple-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["macros"] }
"#
    }

    /// Over-configured project with many unused features
    pub fn over_configured_project() -> &'static str {
        r#"
[package]
name = "over-configured-project"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
syn = { version = "2.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "rustls-tls", "stream", "multipart"] }
clap = { version = "4.0", features = ["derive", "env", "unicode", "wrap_help"] }
"#
    }

    /// Under-configured project missing required features
    pub fn under_configured_project() -> &'static str {
        r#"
[package]
name = "under-configured-project"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["macros"] }
serde = "1.0"
"#
    }

    /// Well-optimized project (should have no recommendations)
    pub fn well_optimized_project() -> &'static str {
        r#"
[package]
name = "well-optimized-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["macros", "time"] }
"#
    }

    /// Workspace project with multiple packages
    pub fn workspace_project() -> (&'static str, Vec<(&'static str, &'static str)>) {
        let workspace_toml = r#"
[workspace]
members = ["app", "lib"]

[workspace.dependencies]
tokio = { version = "1.0", features = ["macros"] }
serde = { version = "1.0", features = ["derive"] }
"#;

        let member_tomls = vec![
            (
                "app/Cargo.toml",
                r#"
[package]
name = "app"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true, features = ["time"] }
lib = { path = "../lib" }
"#,
            ),
            (
                "lib/Cargo.toml",
                r#"
[package]
name = "lib"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { workspace = true }
"#,
            ),
        ];

        (workspace_toml, member_tomls)
    }

    /// Project with dev dependencies
    pub fn project_with_dev_deps() -> &'static str {
        r#"
[package]
name = "project-with-dev-deps"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"

[dev-dependencies]
tokio-test = "0.4"
tempfile = "3.0"
criterion = { version = "0.5", features = ["html_reports"] }
"#
    }
}

/// Mock Rust source code for testing
#[allow(dead_code)]
pub struct RustCodeFixtures;

#[allow(dead_code)]
impl RustCodeFixtures {
    /// Simple main.rs with tokio main
    pub fn tokio_main_only() -> &'static str {
        r#"
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
"#
    }

    /// Code using tokio time features
    pub fn tokio_with_time() -> &'static str {
        r#"
use tokio::time;

#[tokio::main]
async fn main() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Hello after delay!");
}
"#
    }

    /// Code using serde derives
    pub fn serde_with_derives() -> &'static str {
        r#"
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person { name: "Alice".to_string(), age: 30 };
    let json = serde_json::to_string(&person).unwrap();
    println!("{}", json);
}
"#
    }

    /// Code using syn for parsing
    pub fn syn_basic_usage() -> &'static str {
        r#"
use syn::parse_str;
use quote::quote;

fn main() {
    let expr: syn::Expr = parse_str("1 + 2").unwrap();
    let tokens = quote! { #expr };
    println!("{}", tokens);
}
"#
    }

    /// Code with no actual feature usage (should trigger optimizations)
    pub fn minimal_usage() -> &'static str {
        r#"
fn main() {
    println!("Hello, world!");
}
"#
    }

    /// Complex code using multiple features
    pub fn complex_usage() -> &'static str {
        r#"
use serde::{Serialize, Deserialize};
use tokio::time;
use tokio::fs;

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_str = tokio::fs::read_to_string("config.json").await?;
    let config: Config = serde_json::from_str(&config_str)?;

    tokio::time::sleep(tokio::time::Duration::from_secs(config.timeout)).await;

    println!("Config loaded: {}", config.name);
    Ok(())
}
"#
    }

    /// Code with conditional compilation
    pub fn conditional_compilation() -> &'static str {
        r#"
#[cfg(feature = "tokio")]
use tokio::time;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct Data {
    value: i32,
}

fn main() {
    #[cfg(feature = "tokio")]
    {
        // This would require tokio runtime
        println!("Tokio feature enabled");
    }

    #[cfg(feature = "serde")]
    {
        let data = Data { value: 42 };
        println!("Data: {:?}", data);
    }
}
"#
    }
}

/// Expected analysis results for test cases
#[allow(dead_code)]
pub struct ExpectedResults;

#[allow(dead_code)]
impl ExpectedResults {
    /// Expected result for over-configured project
    pub fn over_configured_expectations() -> Vec<OptimizationExpectation> {
        vec![
            OptimizationExpectation {
                crate_name: "tokio".to_string(),
                current_features: vec!["full".to_string()],
                recommended_features: vec!["macros".to_string()],
                optimization_type: OptimizationType::RemoveUnusedFeatures,
                confidence: 0.9,
                auto_applicable: true,
                estimated_savings_kb: 100,
            },
            OptimizationExpectation {
                crate_name: "serde".to_string(),
                current_features: vec!["derive".to_string()],
                recommended_features: vec![],
                optimization_type: OptimizationType::RemoveUnusedFeatures,
                confidence: 0.8,
                auto_applicable: true,
                estimated_savings_kb: 30,
            },
            OptimizationExpectation {
                crate_name: "syn".to_string(),
                current_features: vec!["full".to_string()],
                recommended_features: vec![],
                optimization_type: OptimizationType::RemoveUnusedFeatures,
                confidence: 0.85,
                auto_applicable: true,
                estimated_savings_kb: 50,
            },
        ]
    }

    /// Expected result for under-configured project
    pub fn under_configured_expectations() -> Vec<OptimizationExpectation> {
        vec![OptimizationExpectation {
            crate_name: "tokio".to_string(),
            current_features: vec!["macros".to_string()],
            recommended_features: vec!["macros".to_string(), "time".to_string()],
            optimization_type: OptimizationType::AddMissingFeatures,
            confidence: 0.8,
            auto_applicable: true,
            estimated_savings_kb: 0, // Adding features doesn't save space
        }]
    }

    /// Expected result for well-optimized project (no recommendations)
    pub fn well_optimized_expectations() -> Vec<OptimizationExpectation> {
        vec![] // No optimizations needed
    }
}

/// Structure representing an expected optimization recommendation
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OptimizationExpectation {
    pub crate_name: String,
    pub current_features: Vec<String>,
    pub recommended_features: Vec<String>,
    pub optimization_type: OptimizationType,
    pub confidence: f64,
    pub auto_applicable: bool,
    pub estimated_savings_kb: u32,
}

/// Types of optimizations
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum OptimizationType {
    RemoveUnusedFeatures,
    AddMissingFeatures,
    ReplaceWithAlternative,
}

/// Helper functions for test setup
#[allow(dead_code)]
pub struct TestHelpers;

#[allow(dead_code)]
impl TestHelpers {
    /// Creates a temporary directory with given files
    pub fn create_temp_project(
        cargo_toml: &str,
        rust_files: &[(&str, &str)],
    ) -> Result<tempfile::TempDir, std::io::Error> {
        use std::fs;

        let temp_dir = tempfile::TempDir::new()?;

        // Write Cargo.toml
        fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)?;

        // Create src directory
        fs::create_dir_all(temp_dir.path().join("src"))?;

        // Write Rust source files
        for (filename, content) in rust_files {
            let file_path = if filename.starts_with("../") {
                // Handle files outside src directory (like tests)
                temp_dir.path().join(filename.trim_start_matches("../"))
            } else {
                temp_dir.path().join("src").join(filename)
            };

            // Create parent directories if needed
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(file_path, content)?;
        }

        Ok(temp_dir)
    }

    /// Validates that analysis results match expectations
    pub fn validate_expectations(
        _actual_results: &[OptimizationExpectation], // TODO: Replace with actual result type
        expected_results: &[OptimizationExpectation],
    ) -> bool {
        // TODO: Implement validation logic
        println!("Validating {} expected results", expected_results.len());
        true // Placeholder
    }
}

//! End-to-end tests for decrust-crates
//! Tests the complete CLI workflow and output validation

use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use std::fs;
use tempfile::TempDir;

mod test_fixtures;
use test_fixtures::*;

/// Test the basic CLI help output
#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.arg("--help");

    cmd.assert().success().stdout(predicate::str::contains(
        "Comprehensive dependency analyzer",
    ));
}

/// Test analysis of a simple project
#[test]
#[serial]
fn test_simple_project_analysis() {
    let temp_dir = TestHelpers::create_temp_project(
        CargoTomlFixtures::simple_project(),
        &[("main.rs", RustCodeFixtures::tokio_main_only())],
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.current_dir(temp_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "🔍 Decrust Crates Dependency Optimizer",
        ))
        .stdout(predicate::str::contains("📁 Project Analysis"))
        .stdout(predicate::str::contains("Dependencies in Cargo.toml"));
}

/// Test analysis of over-configured project (should find optimizations)
#[test]
#[serial]
fn test_over_configured_project() {
    let temp_dir = TestHelpers::create_temp_project(
        CargoTomlFixtures::over_configured_project(),
        &[("main.rs", RustCodeFixtures::minimal_usage())],
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.current_dir(temp_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("📁 Project Analysis"));
}

/// Test analysis of well-optimized project (should find no issues)
#[test]
#[serial]
fn test_well_optimized_project() {
    let temp_dir = TestHelpers::create_temp_project(
        CargoTomlFixtures::well_optimized_project(),
        &[("main.rs", RustCodeFixtures::serde_with_derives())],
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.current_dir(temp_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("📁 Project Analysis"));
}

/// Test under-configured project (should suggest adding features)
#[test]
#[serial]
fn test_under_configured_project() {
    let temp_dir = TestHelpers::create_temp_project(
        CargoTomlFixtures::under_configured_project(),
        &[("main.rs", RustCodeFixtures::tokio_with_time())],
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.current_dir(temp_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("📁 Project Analysis"));
}

/// Test verbose mode
#[test]
#[serial]
fn test_verbose_mode() {
    let temp_dir = TestHelpers::create_temp_project(
        CargoTomlFixtures::simple_project(),
        &[("main.rs", RustCodeFixtures::tokio_main_only())],
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.current_dir(temp_dir.path()).arg("--verbose");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Analyzing").or(predicate::str::contains("Scanning")));
}

/// Test error handling for invalid project
#[test]
fn test_invalid_project_error() {
    let temp_dir = TempDir::new().unwrap();
    // Create directory without Cargo.toml

    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.current_dir(temp_dir.path());

    cmd.assert().failure().stderr(
        predicate::str::contains("Error:")
            .or(predicate::str::contains("NotFound"))
            .or(predicate::str::contains("cannot find")),
    );
}

/// Test error handling for malformed Cargo.toml
#[test]
#[serial]
fn test_malformed_cargo_toml() {
    let temp_dir = TempDir::new().unwrap();

    // Write malformed Cargo.toml
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"
[package
name = "test"
version = "0.1.0"
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.current_dir(temp_dir.path());

    cmd.assert().failure().stderr(
        predicate::str::contains("parse")
            .or(predicate::str::contains("error"))
            .or(predicate::str::contains("invalid")),
    );
}

/// Test workspace project analysis
#[test]
#[serial]
fn test_workspace_analysis() {
    let temp_dir = TempDir::new().unwrap();
    let (workspace_toml, member_tomls) = CargoTomlFixtures::workspace_project();

    // Write workspace Cargo.toml
    fs::write(temp_dir.path().join("Cargo.toml"), workspace_toml).unwrap();

    // Create member packages
    for (path, content) in member_tomls {
        let full_path = temp_dir.path().join(path);
        fs::create_dir_all(full_path.parent().unwrap()).unwrap();
        fs::write(full_path, content).unwrap();

        // Create src directories and main.rs files
        let src_dir = temp_dir.path().join(path).parent().unwrap().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("main.rs"), RustCodeFixtures::tokio_main_only()).unwrap();
    }

    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.current_dir(temp_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("📁 Project Analysis"));
}

/// Test performance with larger project
#[test]
#[serial]
fn test_performance_large_project() {
    let temp_dir = TempDir::new().unwrap();

    // Create a project with many dependencies
    let large_cargo_toml = r#"
[package]
name = "large-project"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
syn = { version = "2.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
"#;

    fs::write(temp_dir.path().join("Cargo.toml"), large_cargo_toml).unwrap();
    fs::create_dir_all(temp_dir.path().join("src")).unwrap();

    // Create multiple source files
    for i in 0..10 {
        fs::write(
            temp_dir.path().join("src").join(format!("module_{}.rs", i)),
            RustCodeFixtures::minimal_usage(),
        )
        .unwrap();
    }

    fs::write(
        temp_dir.path().join("src/main.rs"),
        RustCodeFixtures::minimal_usage(),
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.current_dir(temp_dir.path());

    // Should complete within reasonable time (30 seconds)
    cmd.timeout(std::time::Duration::from_secs(30))
        .assert()
        .success()
        .stdout(predicate::str::contains("📁 Project Analysis"));
}

/// Test that the tool handles missing source files gracefully
#[test]
#[serial]
fn test_missing_source_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create Cargo.toml but no source files
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        CargoTomlFixtures::simple_project(),
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("decrates").unwrap();
    cmd.current_dir(temp_dir.path());

    // Should handle gracefully, not crash
    cmd.assert()
        .success() // Should still succeed, just with limited analysis
        .stdout(predicate::str::contains("📁 Project Analysis"));
}

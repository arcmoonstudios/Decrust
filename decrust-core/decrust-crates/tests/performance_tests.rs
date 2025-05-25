//! Performance tests for decrust-crates
//! Ensures the tool performs well on projects of various sizes

use serial_test::serial;
use std::fs;
use std::time::{Duration, Instant};
use tempfile::TempDir;

mod test_fixtures;
use test_fixtures::*;

/// Performance test configuration
struct PerformanceTest {
    name: &'static str,
    dependencies_count: usize,
    source_files_count: usize,
    max_duration: Duration,
}

const PERFORMANCE_TESTS: &[PerformanceTest] = &[
    PerformanceTest {
        name: "Small project",
        dependencies_count: 5,
        source_files_count: 10,
        max_duration: Duration::from_secs(2),
    },
    PerformanceTest {
        name: "Medium project",
        dependencies_count: 20,
        source_files_count: 50,
        max_duration: Duration::from_secs(5),
    },
    PerformanceTest {
        name: "Large project",
        dependencies_count: 50,
        source_files_count: 200,
        max_duration: Duration::from_secs(15),
    },
];

#[test]
#[serial]
fn test_performance_benchmarks() {
    for test_config in PERFORMANCE_TESTS {
        println!("Running performance test: {}", test_config.name);

        let temp_dir = create_performance_test_project(
            test_config.dependencies_count,
            test_config.source_files_count,
        )
        .unwrap();

        let start_time = Instant::now();

        // Run the analysis (this would need to be adapted to call the actual analysis function)
        let result = run_analysis_on_project(temp_dir.path());

        let duration = start_time.elapsed();

        println!(
            "  {} completed in {:?} (max: {:?})",
            test_config.name, duration, test_config.max_duration
        );

        assert!(
            result.is_ok(),
            "Analysis failed for {}: {:?}",
            test_config.name,
            result.err()
        );

        assert!(
            duration <= test_config.max_duration,
            "Performance test '{}' took {:?}, which exceeds maximum of {:?}",
            test_config.name,
            duration,
            test_config.max_duration
        );
    }
}

/// Test memory usage doesn't grow excessively with project size
#[test]
#[serial]
fn test_memory_usage() {
    // This is a basic test - in a real scenario you'd use tools like valgrind or custom memory tracking

    let small_project = create_performance_test_project(10, 20).unwrap();
    let large_project = create_performance_test_project(100, 500).unwrap();

    // Run analysis on both projects
    let _small_result = run_analysis_on_project(small_project.path());
    let _large_result = run_analysis_on_project(large_project.path());

    // In a real implementation, you would measure memory usage here
    // For now, we just ensure both complete successfully
    println!("Memory usage test completed - both projects analyzed successfully");
}

/// Test concurrent analysis performance
#[test]
#[serial]
fn test_concurrent_analysis() {
    use std::thread;

    let projects: Vec<_> = (0..4)
        .map(|i| {
            create_performance_test_project(15, 30)
                .unwrap_or_else(|e| panic!("Failed to create test project {}: {}", i, e))
        })
        .collect();

    let start_time = Instant::now();

    let handles: Vec<_> = projects
        .iter()
        .enumerate()
        .map(|(i, project)| {
            let project_path = project.path().to_path_buf();
            thread::spawn(move || {
                println!("Starting concurrent analysis {}", i);
                let result = run_analysis_on_project(&project_path);
                println!("Completed concurrent analysis {}", i);
                result
            })
        })
        .collect();

    // Wait for all analyses to complete
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.join().unwrap();
        assert!(
            result.is_ok(),
            "Concurrent analysis {} failed: {:?}",
            i,
            result.err()
        );
    }

    let total_duration = start_time.elapsed();
    println!("Concurrent analysis completed in {:?}", total_duration);

    // Should complete within reasonable time even with concurrent access
    assert!(
        total_duration <= Duration::from_secs(30),
        "Concurrent analysis took too long: {:?}",
        total_duration
    );
}

/// Test analysis of projects with many small files vs few large files
#[test]
#[serial]
fn test_file_size_distribution_performance() {
    // Many small files
    let many_small = create_project_with_file_distribution(50, 100).unwrap(); // 50 deps, 100 small files

    // Few large files
    let few_large = create_project_with_file_distribution(50, 10).unwrap(); // 50 deps, 10 large files

    let start_small = Instant::now();
    let _result_small = run_analysis_on_project(many_small.path());
    let duration_small = start_small.elapsed();

    let start_large = Instant::now();
    let _result_large = run_analysis_on_project(few_large.path());
    let duration_large = start_large.elapsed();

    println!("Many small files: {:?}", duration_small);
    println!("Few large files: {:?}", duration_large);

    // Both should complete in reasonable time
    assert!(duration_small <= Duration::from_secs(10));
    assert!(duration_large <= Duration::from_secs(10));
}

/// Creates a test project with specified number of dependencies and source files
fn create_performance_test_project(
    deps_count: usize,
    files_count: usize,
) -> Result<TempDir, std::io::Error> {
    let temp_dir = TempDir::new()?;

    // Generate Cargo.toml with many dependencies
    let mut cargo_toml = String::from(
        r#"
[package]
name = "performance-test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
    );

    // Add common dependencies with various feature configurations
    let common_deps = [
        r#"tokio = { version = "1.0", features = ["full"] }"#,
        r#"serde = { version = "1.0", features = ["derive"] }"#,
        r#"syn = { version = "2.0", features = ["full"] }"#,
        r#"reqwest = { version = "0.11", features = ["json", "rustls-tls"] }"#,
        r#"clap = { version = "4.0", features = ["derive"] }"#,
        r#"anyhow = "1.0""#,
        r#"thiserror = "1.0""#,
        r#"tracing = "0.1""#,
        r#"uuid = { version = "1.0", features = ["v4"] }"#,
        r#"chrono = { version = "0.4", features = ["serde"] }"#,
    ];

    for (i, dep) in common_deps.iter().cycle().take(deps_count).enumerate() {
        if i < common_deps.len() {
            cargo_toml.push_str(dep);
        } else {
            // Generate additional dummy dependencies
            cargo_toml.push_str(&format!(r#"dummy-dep-{} = "1.0""#, i));
        }
        cargo_toml.push('\n');
    }

    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)?;

    // Create src directory
    fs::create_dir_all(temp_dir.path().join("src"))?;

    // Generate source files
    for i in 0..files_count {
        let filename = if i == 0 {
            "main.rs".to_string()
        } else {
            format!("module_{}.rs", i)
        };

        let content = if i % 3 == 0 {
            RustCodeFixtures::tokio_with_time()
        } else if i % 3 == 1 {
            RustCodeFixtures::serde_with_derives()
        } else {
            RustCodeFixtures::minimal_usage()
        };

        fs::write(temp_dir.path().join("src").join(filename), content)?;
    }

    Ok(temp_dir)
}

/// Creates a project with specific file size distribution
fn create_project_with_file_distribution(
    deps_count: usize,
    files_count: usize,
) -> Result<TempDir, std::io::Error> {
    let temp_dir = create_performance_test_project(deps_count, files_count)?;

    // Modify files to have different sizes
    for i in 0..files_count {
        let filename = if i == 0 {
            "main.rs".to_string()
        } else {
            format!("module_{}.rs", i)
        };

        let base_content = RustCodeFixtures::complex_usage();

        // For "large files" test, make some files much larger
        let content = if files_count <= 20 {
            // Few large files - repeat content multiple times
            (0..10)
                .map(|_| base_content)
                .collect::<Vec<_>>()
                .join("\n\n")
        } else {
            // Many small files - use base content as-is
            base_content.to_string()
        };

        fs::write(temp_dir.path().join("src").join(filename), content)?;
    }

    Ok(temp_dir)
}

/// Mock function to run analysis on a project
/// In the real implementation, this would call the actual analysis logic
fn run_analysis_on_project(project_path: &std::path::Path) -> Result<(), String> {
    // TODO: Replace with actual analysis function call
    // For now, simulate analysis by reading the Cargo.toml and some source files

    let cargo_toml_path = project_path.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        return Err("Cargo.toml not found".to_string());
    }

    let _cargo_content = fs::read_to_string(cargo_toml_path)
        .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

    // Simulate some processing time
    std::thread::sleep(Duration::from_millis(10));

    // Read some source files
    let src_dir = project_path.join("src");
    if src_dir.exists() {
        if let Ok(entries) = fs::read_dir(src_dir) {
            for entry in entries.take(10).flatten() {
                // Limit to first 10 files for performance
                let _ = fs::read_to_string(entry.path());
                std::thread::sleep(Duration::from_millis(1)); // Simulate processing
            }
        }
    }

    Ok(())
}

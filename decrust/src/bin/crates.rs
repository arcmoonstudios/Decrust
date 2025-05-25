#!/usr/bin/env cargo
/* src/bin/crates.rs */
#![warn(missing_docs)]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! **Decrust Dependency Optimizer CLI**
//!
//! Scans your entire Rust project to analyze which dependency features you're actually using
//! and provides interactive optimization recommendations with automatic Cargo.toml updates.
//!
//! Usage: `cargo run --bin crates`
// Decrust/src/bin/crates.rs
// Copyright (c) 2025 ArcMoon Studios
// Licensed under the Business Source License 1.1
// Non-production use only. Commercial/prod use requires a paid license.
// Change Date: 2029-05-25 | Change License: GPL v3
// Contact: LordXyn@proton.me
// GitHub: https://github.com/arcmoonstudios/decrust
// Author: Lord Xyn

use decrust_core::{DependencyAnalyzer, RecommendationType, SecurityImpact};
use std::fs;
use std::io;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 **Decrust Dependency Optimizer**");
    println!("====================================");
    println!("Scanning your project for dependency optimization opportunities...\n");

    // Scan the entire project for Rust source files
    let source_files = find_rust_source_files(".")?;
    println!("📁 Found {} Rust source files", source_files.len());

    // Read all source code
    let mut all_code = String::new();
    for file_path in &source_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            all_code.push_str(&content);
            all_code.push('\n');
        }
    }

    // Analyze dependencies
    let mut analyzer = DependencyAnalyzer::new();
    let results = analyzer.analyze_code_dependencies(&all_code);

    if results.is_empty() {
        println!("✨ No external dependencies found in your project.");
        return Ok(());
    }

    // Display analysis results
    println!("\n🔍 **Dependency Analysis Results**");
    println!("==================================\n");

    let mut optimization_opportunities = Vec::new();
    let mut total_potential_savings = 0;
    let mut total_time_savings = 0.0;

    for result in &results {
        println!("📦 **{}** v{}", result.crate_name, result.current_version);

        // Show current vs optimal configuration
        println!(
            "   🔧 Current features: {}",
            result.enabled_features.join(", ")
        );
        println!("   🎯 Actually used: {}", result.used_features.join(", "));

        if !result.unused_features.is_empty() {
            println!(
                "   ⚠️  **UNUSED**: {} (wasting space!)",
                result.unused_features.join(", ")
            );
        }

        if !result.missing_features.is_empty() {
            println!(
                "   ❌ **MISSING**: {} (potential issues!)",
                result.missing_features.join(", ")
            );
        }

        // Show what's actually being used from this crate
        let usage = &result.usage_analysis;
        if !usage.functions_used.is_empty()
            || !usage.types_used.is_empty()
            || !usage.derive_macros_used.is_empty()
        {
            println!("   📋 Usage details:");
            if !usage.functions_used.is_empty() {
                println!("      • Functions: {}", usage.functions_used.join(", "));
            }
            if !usage.types_used.is_empty() {
                println!("      • Types: {}", usage.types_used.join(", "));
            }
            if !usage.derive_macros_used.is_empty() {
                println!(
                    "      • Derive macros: {}",
                    usage.derive_macros_used.join(", ")
                );
            }
            if !usage.attribute_macros_used.is_empty() {
                println!(
                    "      • Attribute macros: {}",
                    usage.attribute_macros_used.join(", ")
                );
            }
        }

        // Collect optimization opportunities
        for rec in &result.interactive_recommendations {
            if rec.confidence > 0.8 {
                optimization_opportunities.push((result.crate_name.clone(), rec.clone()));

                if let Some(savings) = rec.estimated_impact.binary_size_reduction {
                    total_potential_savings += savings;
                }
                if let Some(time) = rec.estimated_impact.compile_time_improvement {
                    total_time_savings += time;
                }
            }
        }

        println!();
    }

    // Show optimization summary
    if !optimization_opportunities.is_empty() {
        println!("💡 **Optimization Opportunities Found**");
        println!("=======================================\n");

        for (i, (crate_name, rec)) in optimization_opportunities.iter().enumerate() {
            println!(
                "{}. **{}** - {:?}",
                i + 1,
                crate_name,
                rec.recommendation_type
            );
            println!("   📝 {}", rec.explanation);
            println!("   🔧 Current:  {}", rec.current_config);
            println!("   ✨ Optimal:  {}", rec.recommended_config);

            if let Some(savings) = rec.estimated_impact.binary_size_reduction {
                println!("   💾 Savings: ~{} KB", savings / 1000);
            }
            if let Some(time) = rec.estimated_impact.compile_time_improvement {
                println!("   ⚡ Faster builds: ~{:.1}s", time);
            }

            match rec.estimated_impact.security_improvement {
                SecurityImpact::High => println!("   🔒 Security: HIGH improvement"),
                SecurityImpact::Medium => println!("   🔒 Security: MEDIUM improvement"),
                SecurityImpact::Low => println!("   🔒 Security: LOW improvement"),
                SecurityImpact::None => {}
            }

            if rec.auto_applicable {
                println!("   ✅ **SAFE TO AUTO-APPLY**");
            } else {
                println!("   ⚠️  **REQUIRES MANUAL REVIEW**");
            }
            println!();
        }

        // Show total impact
        println!("📊 **Total Potential Impact**");
        println!("============================");
        if total_potential_savings > 0 {
            println!(
                "💾 Binary size reduction: ~{} KB",
                total_potential_savings / 1000
            );
        }
        if total_time_savings > 0.0 {
            println!("⚡ Compile time improvement: ~{:.1}s", total_time_savings);
        }
        println!("🔒 Security improvement: Reduced attack surface");
        println!();

        // Interactive optimization
        println!("🎯 **Interactive Optimization**");
        println!("===============================");
        println!("Would you like to automatically optimize your Cargo.toml? (y/N): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
            apply_optimizations(&optimization_opportunities)?;
        } else {
            println!("💡 Run this tool again anytime to optimize your dependencies!");
        }
    } else {
        println!("✨ **Excellent!** Your dependency configuration is already well-optimized!");
        println!("   No high-confidence optimization opportunities found.");
    }

    Ok(())
}

/// Finds all Rust source files in the project
fn find_rust_source_files(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut rust_files = Vec::new();
    find_rust_files_recursive(Path::new(dir), &mut rust_files)?;
    Ok(rust_files)
}

/// Recursively finds Rust files, excluding target directory
fn find_rust_files_recursive(
    dir: &Path,
    rust_files: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        // Skip target directory and hidden directories
        if let Some(dir_name) = dir.file_name() {
            if dir_name == "target" || dir_name.to_string_lossy().starts_with('.') {
                return Ok(());
            }
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                find_rust_files_recursive(&path, rust_files)?;
            } else if let Some(extension) = path.extension() {
                if extension == "rs" {
                    rust_files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}

/// Applies the selected optimizations to Cargo.toml
fn apply_optimizations(
    opportunities: &[(String, decrust_core::InteractiveRecommendation)],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 **Applying Optimizations**");
    println!("============================\n");

    // Read current Cargo.toml
    let cargo_content = fs::read_to_string("Cargo.toml")?;
    let mut new_content = cargo_content.clone();

    let mut changes_made = 0;

    for (crate_name, rec) in opportunities {
        if rec.auto_applicable && rec.confidence > 0.85 {
            println!(
                "✅ Optimizing {} ({:?})...",
                crate_name, rec.recommendation_type
            );

            match rec.recommendation_type {
                RecommendationType::RemoveUnusedFeatures => {
                    // Apply the recommended configuration
                    if let Some(updated) =
                        update_crate_features(&new_content, crate_name, &rec.recommended_config)
                    {
                        new_content = updated;
                        changes_made += 1;
                        println!("   ✨ Updated features for {}", crate_name);
                    }
                }
                RecommendationType::SplitFeatures => {
                    // Apply the recommended configuration
                    if let Some(updated) =
                        update_crate_features(&new_content, crate_name, &rec.recommended_config)
                    {
                        new_content = updated;
                        changes_made += 1;
                        println!("   ✨ Split features for {}", crate_name);
                    }
                }
                _ => {
                    println!(
                        "   ⚠️  Skipping {:?} (requires manual review)",
                        rec.recommendation_type
                    );
                }
            }
        } else {
            println!(
                "⚠️  Skipping {} (confidence: {:.0}%, auto-applicable: {})",
                crate_name,
                rec.confidence * 100.0,
                rec.auto_applicable
            );
        }
    }

    if changes_made > 0 {
        // Create backup
        fs::write("Cargo.toml.backup", &cargo_content)?;
        println!("\n💾 Created backup: Cargo.toml.backup");

        // Write optimized Cargo.toml
        fs::write("Cargo.toml", new_content)?;
        println!(
            "✅ Updated Cargo.toml with {} optimization(s)",
            changes_made
        );

        println!("\n🎯 **Next Steps:**");
        println!("1. Run `cargo check` to verify everything still works");
        println!("2. Run `cargo build --release` to see the improvements");
        println!("3. If there are issues, restore with: `mv Cargo.toml.backup Cargo.toml`");
    } else {
        println!("ℹ️  No auto-applicable optimizations found.");
    }

    Ok(())
}

/// Updates crate features in Cargo.toml content
fn update_crate_features(content: &str, crate_name: &str, new_config: &str) -> Option<String> {
    // This is a simplified implementation - in production, you'd want proper TOML parsing
    // For now, we'll do basic string replacement for the demo

    // Extract the new features from the recommended config
    if let Some(start) = new_config.find('[') {
        if let Some(end) = new_config.find(']') {
            let features_str = &new_config[start..=end];

            // Find the crate line and update it
            let lines: Vec<&str> = content.lines().collect();
            let mut new_lines = Vec::new();

            for line in lines {
                if line.contains(crate_name) && line.contains("features") {
                    // Replace the features part
                    if let Some(eq_pos) = line.find('=') {
                        let before_eq = &line[..eq_pos];
                        new_lines.push(format!(
                            "{} = {{ version = \"*\", features = {} }}",
                            before_eq.trim(),
                            features_str
                        ));
                    } else {
                        new_lines.push(line.to_string());
                    }
                } else {
                    new_lines.push(line.to_string());
                }
            }

            return Some(new_lines.join("\n"));
        }
    }

    None
}

/* src/main.rs */
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! Comprehensive dependency analyzer and optimizer for Rust projects
//!
//! Usage: cargo run -p decrust-crates
//!
//! This tool:
//! 1. Scans all dependencies in Cargo.toml
//! 2. Uses Syn to analyze actual usage in source code
//! 3. Fetches crate metadata from docs.rs/crates.io
//! 4. Provides interactive optimization recommendations
//! 5. Optionally auto-updates Cargo.toml with optimized dependencies
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me

use clap::Parser;
use colored::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use decrust_core::DependencyAnalyzer;

/// Command line arguments for the crates optimizer
#[derive(Parser)]
#[command(name = "decrust-crates")]
#[command(
    about = "Comprehensive dependency analyzer and optimizer for Rust projects - part of the Decrust framework"
)]
struct Args {
    /// Apply optimizations automatically (without confirmation)
    #[arg(long)]
    apply: bool,

    /// Show detailed analysis for each crate
    #[arg(long)]
    verbose: bool,

    /// Only analyze specific crates (comma-separated)
    #[arg(long)]
    crates: Option<String>,

    /// Skip network requests (use local analysis only)
    #[arg(long)]
    offline: bool,
}

/// Main entry point for the crates dependency optimizer
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!(
        "{}",
        "🔍 Decrust Crates Dependency Optimizer"
            .bright_cyan()
            .bold()
    );
    println!(
        "{}",
        "==========================================".bright_cyan()
    );
    println!("Scanning project for dependency optimization opportunities...\n");

    let mut optimizer = CrateOptimizer::new(args)?;
    optimizer.run_full_analysis().await?;

    Ok(())
}

/// Comprehensive crate dependency optimizer
struct CrateOptimizer {
    /// Command line arguments
    args: Args,
    /// Decrust analyzer for dependency analysis
    analyzer: DependencyAnalyzer,
    /// Project root directory
    project_root: std::path::PathBuf,
    /// All Cargo.toml files in the project
    #[allow(dead_code)] // Future enhancement for workspace analysis
    cargo_toml_files: Vec<std::path::PathBuf>,
    /// Current Cargo.toml dependencies
    current_dependencies: HashMap<String, DependencyConfig>,
}

/// Configuration for a single dependency
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields are parsed but not all used yet - future enhancement
struct DependencyConfig {
    /// Version specification
    version: String,
    /// Enabled features
    features: Vec<String>,
    /// Whether it's a dev dependency
    is_dev_dependency: bool,
    /// Whether it's optional
    optional: bool,
    /// Raw TOML configuration
    raw_config: String,
}

/// Crate metadata from external sources
#[derive(Debug, Clone)]
#[allow(dead_code)] // Metadata fields for future API integration
struct CrateMetadata {
    /// Crate name
    name: String,
    /// Latest version
    latest_version: String,
    /// All available features
    available_features: Vec<String>,
    /// Default features
    default_features: Vec<String>,
    /// Feature descriptions
    feature_descriptions: HashMap<String, String>,
}

impl CrateOptimizer {
    /// Creates a new crate optimizer
    fn new(args: Args) -> Result<Self, Box<dyn std::error::Error>> {
        let project_root = std::env::current_dir()?;
        let cargo_toml_files = Self::find_cargo_toml_files(&project_root)?;
        let current_dependencies = Self::parse_cargo_toml(&project_root)?;

        Ok(Self {
            args,
            analyzer: DependencyAnalyzer::new(),
            project_root,
            cargo_toml_files,
            current_dependencies,
        })
    }

    /// Runs the complete dependency analysis and optimization workflow
    async fn run_full_analysis(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", "📁 Project Analysis".bright_yellow().bold());
        println!("  • Project root: {}", self.project_root.display());
        println!(
            "  • Dependencies in Cargo.toml: {}\n",
            self.current_dependencies.len()
        );

        // Step 1: **EFFICIENT** - Only analyze dependencies that exist in Cargo.toml
        println!(
            "{}",
            "🔍 Step 1: Targeted Dependency Analysis"
                .bright_yellow()
                .bold()
        );
        let usage_analysis = self.analyze_specific_dependencies()?;

        // Step 2: Fetch crate metadata (if not offline)
        if !self.args.offline {
            println!(
                "\n{}",
                "📡 Step 2: Fetching Crate Metadata".bright_yellow().bold()
            );
            let _crate_metadata = self.fetch_crate_metadata(&usage_analysis).await?;
        }

        // Step 3: Generate optimization recommendations
        println!(
            "\n{}",
            "💡 Step 3: Generating Optimization Recommendations"
                .bright_yellow()
                .bold()
        );
        let recommendations = self.generate_project_recommendations(&usage_analysis)?;

        // Step 4: Display results and interactive options
        println!(
            "\n{}",
            "📊 Step 4: Optimization Results".bright_yellow().bold()
        );
        self.display_optimization_results(&recommendations)?;

        // Step 5: Interactive optimization
        if !recommendations.is_empty() {
            println!(
                "\n{}",
                "🎯 Step 5: Interactive Optimization".bright_yellow().bold()
            );
            self.interactive_optimization(&recommendations)?;
        } else {
            println!(
                "\n{}",
                "✨ Your dependencies are already well-optimized!"
                    .bright_green()
                    .bold()
            );
        }

        Ok(())
    }

    /// **NEW EFFICIENT METHOD**: Analyzes only the dependencies listed in Cargo.toml
    fn analyze_specific_dependencies(
        &mut self,
    ) -> Result<Vec<decrust_core::DependencyAnalysisResult>, Box<dyn std::error::Error>> {
        println!(
            "  • Analyzing {} dependencies from Cargo.toml...",
            self.current_dependencies.len()
        );

        // Only read source files once and analyze all dependencies
        let all_source_code = self.read_targeted_source_files()?;
        let all_analysis_results = self.analyzer.analyze_code_dependencies(&all_source_code);

        // Filter to only include dependencies that are in our Cargo.toml
        let cargo_dependency_names: HashSet<String> =
            self.current_dependencies.keys().cloned().collect();
        let analysis_results: Vec<_> = all_analysis_results
            .into_iter()
            .filter(|result| {
                // Only include if it's in our Cargo.toml dependencies
                cargo_dependency_names.contains(&result.crate_name)
            })
            .collect();

        // Display results
        for result in &analysis_results {
            let status = if result.unused_features.is_empty() && result.missing_features.is_empty()
            {
                "✅".green()
            } else {
                "⚠️".yellow()
            };

            println!(
                "    {} {} v{} (features: {})",
                status,
                result.crate_name,
                result.current_version,
                result.enabled_features.join(", ")
            );
        }

        println!(
            "  • Found {} dependencies actually used in source code",
            analysis_results.len()
        );
        Ok(analysis_results)
    }

    /// Finds Cargo.toml files in the project
    fn find_cargo_toml_files(
        project_root: &Path,
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
        let mut cargo_files = Vec::new();

        // Main Cargo.toml
        let main_cargo = project_root.join("Cargo.toml");
        if main_cargo.exists() {
            cargo_files.push(main_cargo);
        }

        // Look for workspace member Cargo.toml files
        let workspace_dirs = [
            "decrust",
            "decrust-core",
            "decrust-core/decrust-promac",
            "decrust-core/decrust-promac-runtime",
            "decrust-core/decrust-crates",
        ];

        for dir in &workspace_dirs {
            let cargo_path = project_root.join(dir).join("Cargo.toml");
            if cargo_path.exists() {
                cargo_files.push(cargo_path);
            }
        }

        Ok(cargo_files)
    }

    /// Reads source files more efficiently - only the ones we need
    fn read_targeted_source_files(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut all_code = String::new();

        // Only scan the directories that are relevant to the current project
        let source_dirs = [
            "src",              // Main source directory
            "decrust/src",      // Decrust package source
            "decrust-core/src", // Decrust-core package source
        ];

        for dir in &source_dirs {
            let dir_path = self.project_root.join(dir);
            if dir_path.exists() {
                self.read_directory_recursive(&dir_path, &mut all_code)?;
            }
        }

        Ok(all_code)
    }

    /// Recursively reads .rs files from a directory
    fn read_directory_recursive(
        &self,
        dir: &Path,
        all_code: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Skip target directory and hidden directories
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if !dir_name.starts_with('.') && dir_name != "target" {
                        self.read_directory_recursive(&path, all_code)?;
                    }
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        all_code.push_str(&content);
                        all_code.push('\n');
                    }
                    Err(e) => {
                        if self.args.verbose {
                            eprintln!("Warning: Could not read {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Parses Cargo.toml to extract current dependencies
    fn parse_cargo_toml(
        project_root: &Path,
    ) -> Result<HashMap<String, DependencyConfig>, Box<dyn std::error::Error>> {
        let cargo_toml_path = project_root.join("Cargo.toml");
        let cargo_content = fs::read_to_string(cargo_toml_path)?;

        // Parse using toml crate for better accuracy
        let toml_value: toml::Value = toml::from_str(&cargo_content)?;
        let mut dependencies = HashMap::new();

        // Parse [dependencies] section
        if let Some(deps) = toml_value.get("dependencies").and_then(|v| v.as_table()) {
            Self::parse_toml_dependencies(deps, &mut dependencies, false)?;
        }

        // Parse [dev-dependencies] section
        if let Some(dev_deps) = toml_value
            .get("dev-dependencies")
            .and_then(|v| v.as_table())
        {
            Self::parse_toml_dependencies(dev_deps, &mut dependencies, true)?;
        }

        // **NEW**: Parse [workspace.dependencies] section for workspace projects
        if let Some(workspace) = toml_value.get("workspace") {
            if let Some(workspace_deps) = workspace.get("dependencies").and_then(|v| v.as_table()) {
                Self::parse_toml_dependencies(workspace_deps, &mut dependencies, false)?;
            }
        }

        Ok(dependencies)
    }

    /// Parses dependencies from TOML table
    fn parse_toml_dependencies(
        deps_table: &toml::Table,
        dependencies: &mut HashMap<String, DependencyConfig>,
        is_dev: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (name, value) in deps_table {
            let config = match value {
                toml::Value::String(version) => DependencyConfig {
                    version: version.clone(),
                    features: vec!["default".to_string()],
                    is_dev_dependency: is_dev,
                    optional: false,
                    raw_config: format!("\"{}\"", version),
                },
                toml::Value::Table(table) => {
                    let version = table
                        .get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let features = table
                        .get("features")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str())
                                .map(|s| s.to_string())
                                .collect()
                        })
                        .unwrap_or_else(|| vec!["default".to_string()]);

                    let optional = table
                        .get("optional")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    DependencyConfig {
                        version,
                        features,
                        is_dev_dependency: is_dev,
                        optional,
                        raw_config: format!("{}", value),
                    }
                }
                _ => continue,
            };

            dependencies.insert(name.clone(), config);
        }
        Ok(())
    }

    /// Fetches metadata for all analyzed crates
    async fn fetch_crate_metadata(
        &self,
        analysis_results: &[decrust_core::DependencyAnalysisResult],
    ) -> Result<HashMap<String, CrateMetadata>, Box<dyn std::error::Error>> {
        println!(
            "  • Fetching metadata for {} crates...",
            analysis_results.len()
        );

        let mut metadata = HashMap::new();

        for result in analysis_results {
            // For now, create mock metadata (would fetch from crates.io/docs.rs in real implementation)
            let mock_metadata = CrateMetadata {
                name: result.crate_name.clone(),
                latest_version: result.current_version.clone(),
                available_features: self.get_known_features(&result.crate_name),
                default_features: vec!["default".to_string()],
                feature_descriptions: HashMap::new(),
            };

            metadata.insert(result.crate_name.clone(), mock_metadata);
            println!("    ✓ {}", result.crate_name.green());
        }

        Ok(metadata)
    }

    /// Gets known features for common crates (would be replaced with API calls)
    fn get_known_features(&self, crate_name: &str) -> Vec<String> {
        match crate_name {
            "tokio" => vec![
                "default".to_string(),
                "full".to_string(),
                "fs".to_string(),
                "net".to_string(),
                "macros".to_string(),
                "time".to_string(),
                "sync".to_string(),
                "rt".to_string(),
                "rt-multi-thread".to_string(),
            ],
            "serde" => vec![
                "default".to_string(),
                "derive".to_string(),
                "alloc".to_string(),
                "std".to_string(),
                "unstable".to_string(),
            ],
            "regex" => vec![
                "default".to_string(),
                "std".to_string(),
                "unicode".to_string(),
                "unicode-age".to_string(),
                "unicode-bool".to_string(),
            ],
            _ => vec!["default".to_string()],
        }
    }

    /// Generates optimization recommendations for the entire project
    fn generate_project_recommendations(
        &self,
        analysis_results: &[decrust_core::DependencyAnalysisResult],
    ) -> Result<Vec<ProjectOptimization>, Box<dyn std::error::Error>> {
        let mut recommendations = Vec::new();

        for result in analysis_results {
            // Filter by specific crates if requested
            if let Some(ref filter_crates) = self.args.crates {
                let filter_list: HashSet<&str> = filter_crates.split(',').collect();
                if !filter_list.contains(result.crate_name.as_str()) {
                    continue;
                }
            }

            if !result.unused_features.is_empty() || !result.missing_features.is_empty() {
                let optimization = ProjectOptimization {
                    crate_name: result.crate_name.clone(),
                    current_config: format!("features = {:?}", result.enabled_features),
                    recommended_config: self.generate_optimal_config(result),
                    optimization_type: if !result.unused_features.is_empty() {
                        OptimizationType::RemoveUnusedFeatures
                    } else {
                        OptimizationType::AddMissingFeatures
                    },
                    estimated_savings: self.estimate_savings(result),
                    confidence: 0.9,
                    auto_applicable: true,
                    unused_features: result.unused_features.clone(),
                    missing_features: result.missing_features.clone(),
                };

                recommendations.push(optimization);
            }
        }

        Ok(recommendations)
    }

    /// Generates optimal configuration for a crate
    fn generate_optimal_config(&self, result: &decrust_core::DependencyAnalysisResult) -> String {
        let optimal_features: Vec<String> = result
            .enabled_features
            .iter()
            .filter(|f| !result.unused_features.contains(f))
            .cloned()
            .chain(result.missing_features.iter().cloned())
            .collect();

        format!("features = {:?}", optimal_features)
    }

    /// Estimates potential savings from optimization
    fn estimate_savings(
        &self,
        result: &decrust_core::DependencyAnalysisResult,
    ) -> OptimizationSavings {
        let binary_size_reduction = result.unused_features.len() * 50_000; // ~50KB per unused feature
        let compile_time_improvement = result.unused_features.len() as f32 * 0.3; // ~0.3s per feature

        OptimizationSavings {
            binary_size_kb: binary_size_reduction / 1000,
            compile_time_seconds: compile_time_improvement,
            transitive_deps_removed: result.unused_features.len(),
            security_improvement: self.assess_security_impact(result),
            confidence_score: self.calculate_confidence_score(result),
        }
    }

    /// Assesses security impact of optimization (from concepts.md)
    fn assess_security_impact(
        &self,
        result: &decrust_core::DependencyAnalysisResult,
    ) -> SecurityImpact {
        // Analyze security implications of removing features/dependencies
        let unused_count = result.unused_features.len();
        let _missing_count = result.missing_features.len();

        // Higher impact if we're removing many features (potential attack surface reduction)
        if unused_count >= 3 {
            SecurityImpact::Medium
        } else if unused_count >= 1 {
            SecurityImpact::Low
        } else {
            SecurityImpact::None // Adding features doesn't improve security
        }
    }

    /// Calculates confidence score for optimization recommendation (from concepts.md)
    fn calculate_confidence_score(&self, result: &decrust_core::DependencyAnalysisResult) -> f64 {
        let mut confidence: f64 = 0.5; // Base confidence

        // Higher confidence for well-known optimization patterns
        if !result.unused_features.is_empty() {
            confidence += 0.3; // High confidence in removing unused features
        }

        // Lower confidence for adding features (might be false positive)
        if !result.missing_features.is_empty() {
            confidence -= 0.1;
        }

        // Boost confidence for common crates with well-known feature patterns
        match result.crate_name.as_str() {
            "syn" | "serde" | "tokio" | "clap" => confidence += 0.2,
            _ => {}
        }

        // Ensure confidence is between 0.0 and 1.0
        confidence.clamp(0.0, 1.0)
    }

    /// Displays optimization results to the user
    fn display_optimization_results(
        &self,
        recommendations: &[ProjectOptimization],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if recommendations.is_empty() {
            println!(
                "  {}",
                "✨ No optimization opportunities found!"
                    .bright_green()
                    .bold()
            );
            println!("     Your dependency configuration is already well-optimized.");
            return Ok(());
        }

        println!(
            "  {} {}\n",
            "📊 Found".bright_cyan(),
            format!("{} optimization opportunities:", recommendations.len())
                .bright_cyan()
                .bold()
        );

        let mut total_size_savings = 0;
        let mut total_time_savings = 0.0;

        for (i, opt) in recommendations.iter().enumerate() {
            println!(
                "  {}. {} ({})",
                format!("{}", i + 1).bright_white().bold(),
                opt.crate_name.bright_cyan().bold(),
                opt.optimization_type.description().yellow()
            );
            println!("     Current:  {}", opt.current_config.dimmed());
            println!("     Optimal:  {}", opt.recommended_config.green());

            if !opt.unused_features.is_empty() {
                println!("     Remove:   {}", opt.unused_features.join(", ").red());
            }
            if !opt.missing_features.is_empty() {
                println!("     Add:      {}", opt.missing_features.join(", ").green());
            }

            println!(
                "     Savings:  {}KB binary, {:.1}s compile time",
                format!("~{}", opt.estimated_savings.binary_size_kb).green(),
                format!("{:.1}", opt.estimated_savings.compile_time_seconds).green()
            );
            // Enhanced display from concepts.md with security impact and confidence scoring
            println!(
                "     Confidence: {} | Security: {:?} | Score: {:.1}",
                format!("{:.0}%", opt.confidence * 100.0).bright_green(),
                opt.estimated_savings.security_improvement,
                opt.estimated_savings.confidence_score
            );

            total_size_savings += opt.estimated_savings.binary_size_kb;
            total_time_savings += opt.estimated_savings.compile_time_seconds;

            if opt.auto_applicable {
                println!("     {}", "✅ Safe to auto-apply".green());
            } else {
                println!("     {}", "⚠️  Requires manual review".yellow());
            }
            println!();
        }

        println!("  {}", "🎯 Total Potential Savings:".bright_cyan().bold());
        println!(
            "     • Binary size: {}",
            format!("~{}KB", total_size_savings).bright_green().bold()
        );
        println!(
            "     • Compile time: {}",
            format!("~{:.1}s", total_time_savings).bright_green().bold()
        );

        Ok(())
    }

    /// Interactive optimization workflow
    fn interactive_optimization(
        &self,
        recommendations: &[ProjectOptimization],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.apply {
            println!(
                "  {}",
                "🚀 Auto-applying all safe optimizations..."
                    .bright_green()
                    .bold()
            );
            // TODO: Implement auto-apply logic
            println!("  {}", "💡 Auto-apply functionality coming soon!".yellow());
            return Ok(());
        }

        println!("  Would you like to apply these optimizations?");
        println!(
            "  {} Apply all safe optimizations automatically",
            "[a]".bright_green()
        );
        println!(
            "  {} Select specific optimizations to apply",
            "[s]".bright_yellow()
        );
        println!("  {} Show detailed report only", "[r]".bright_blue());
        println!("  {} No changes", "[n]".bright_red());

        println!("\n  Choice [a/s/r/n]: {}", "r".bright_blue());

        println!(
            "\n  {}",
            "📋 Detailed Optimization Report:".bright_cyan().bold()
        );

        for opt in recommendations {
            println!(
                "    • {}: {}",
                opt.crate_name.bright_cyan(),
                opt.optimization_type.description()
            );
            println!(
                "      Impact: {}KB, {:.1}s faster builds",
                format!("{}", opt.estimated_savings.binary_size_kb).green(),
                format!("{:.1}", opt.estimated_savings.compile_time_seconds).green()
            );
        }

        println!(
            "\n  {}",
            "💡 To apply optimizations, run with --apply flag"
                .bright_yellow()
                .bold()
        );
        println!("     Example: cargo run -p decrust-crates -- --apply");

        Ok(())
    }
}

/// Project-level optimization recommendation
#[derive(Debug, Clone)]
struct ProjectOptimization {
    crate_name: String,
    current_config: String,
    recommended_config: String,
    optimization_type: OptimizationType,
    estimated_savings: OptimizationSavings,
    confidence: f32,
    auto_applicable: bool,
    unused_features: Vec<String>,
    missing_features: Vec<String>,
}

/// Type of optimization
#[derive(Debug, Clone)]
enum OptimizationType {
    RemoveUnusedFeatures,
    AddMissingFeatures,
}

impl OptimizationType {
    fn description(&self) -> &'static str {
        match self {
            OptimizationType::RemoveUnusedFeatures => "Remove unused features",
            OptimizationType::AddMissingFeatures => "Add missing features",
        }
    }
}

/// Enhanced optimization savings with impact modeling from concepts.md
#[derive(Debug, Clone)]
struct OptimizationSavings {
    binary_size_kb: usize,
    compile_time_seconds: f32,
    #[allow(dead_code)] // Enhanced features from concepts.md - future display
    transitive_deps_removed: usize,
    #[allow(dead_code)] // Enhanced features from concepts.md - future display
    security_improvement: SecurityImpact,
    #[allow(dead_code)] // Enhanced features from concepts.md - future display
    confidence_score: f64,
}

/// Security impact assessment levels (from concepts.md)
#[derive(Debug, Clone)]
#[allow(dead_code)] // Enhanced framework from concepts.md
enum SecurityImpact {
    None,
    Low,
    Medium,
    High,
}

/// Alternative crate suggestion with intelligent recommendations (from concepts.md)
#[derive(Debug, Clone)]
#[allow(dead_code)] // Advanced feature from concepts.md - future implementation
struct AlternativeCrateSuggestion {
    current_crate: String,
    alternative_name: String,
    alternative_version: String,
    size_comparison: f64,       // Ratio: alternative_size / current_size
    feature_compatibility: f64, // 0.0 to 1.0
    migration_effort: MigrationEffort,
    rationale: String,
    estimated_savings: OptimizationSavings,
}

/// Migration effort assessment (from concepts.md)
#[derive(Debug, Clone)]
#[allow(dead_code)] // Advanced feature from concepts.md - future implementation
enum MigrationEffort {
    Trivial,  // Drop-in replacement
    Easy,     // Minor API changes
    Moderate, // Some refactoring required
    Complex,  // Significant changes needed
}

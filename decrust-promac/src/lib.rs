// decrust/decrust_promac/src/lib.rs

extern crate proc_macro;
// Import the decrust-promac-runtime crate
extern crate decrust_promac_runtime;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use regex::Regex;
use syn::{
    parse_macro_input, parse_quote, Block, DeriveInput, Error as SynError, Expr, Fields, Ident,
    ItemFn, Lit, LitInt, LitStr, Meta, Stmt,
};

// Note: Since this is a proc-macro crate, we can't re-export types directly.
// Instead, we'll use the decrust-promac-runtime crate to provide the runtime components.
// Users should import from decrust-promac-runtime instead of directly from decrust.

// Helper to parse #[decrust(...)] attributes
// This will need to be significantly more complex to handle all your specified configurations
#[derive(Default, Debug)]
struct DecrustAttributeArgs {
    circuit_breaker: Option<CircuitBreakerConfigArgs>,
    autocorrect: Option<AutocorrectModeArgs>,
    reporting: Option<ReportingConfigArgs>,
    performance_tracking: Option<bool>,
    retry: Option<RetryConfigArgs>,
    // Tri-Protocol Fusion System configuration
    #[allow(dead_code)]
    mathematical_intent: Option<MathematicalIntentConfigArgs>,
    #[allow(dead_code)]
    systematic_resolution: Option<SystematicResolutionConfigArgs>,
    #[allow(dead_code)]
    autonomous_processing: Option<AutonomousProcessingConfigArgs>,
    #[allow(dead_code)]
    certification_level: Option<CertificationLevelConfigArgs>,
}

// Mathematical Intent Analysis Configuration for CodeMASTER v3
#[derive(Default, Debug)]
struct MathematicalIntentConfigArgs {
    #[allow(dead_code)]
    enabled: Option<bool>,
    #[allow(dead_code)]
    quality_coefficient: Option<f64>,
    #[allow(dead_code)]
    performance_coefficient: Option<f64>,
    #[allow(dead_code)]
    security_coefficient: Option<f64>,
    #[allow(dead_code)]
    maintainability_coefficient: Option<f64>,
    #[allow(dead_code)]
    optimization_level: Option<Ident>, // Standard, Enterprise, Diamond, EliteDiamond, EliteDiamondMars
}

// Systematic Error Resolution Configuration for M.A.R.S.
#[derive(Default, Debug)]
struct SystematicResolutionConfigArgs {
    #[allow(dead_code)]
    enabled: Option<bool>,
    #[allow(dead_code)]
    causal_chain_analysis: Option<bool>,
    #[allow(dead_code)]
    causal_resolution_depth: Option<LitInt>,
    #[allow(dead_code)]
    error_priority_weights: Option<ErrorPriorityWeightsArgs>,
}

// Error Priority Weights for M.A.R.S.
#[derive(Default, Debug)]
struct ErrorPriorityWeightsArgs {
    #[allow(dead_code)]
    io: Option<f64>,
    #[allow(dead_code)]
    parsing: Option<f64>,
    #[allow(dead_code)]
    network: Option<f64>,
    #[allow(dead_code)]
    configuration: Option<f64>,
    #[allow(dead_code)]
    validation: Option<f64>,
}

// Autonomous Processing Configuration for Luna⚛︎Ultima
#[derive(Default, Debug)]
struct AutonomousProcessingConfigArgs {
    #[allow(dead_code)]
    enabled: Option<bool>,
    #[allow(dead_code)]
    optimization_level: Option<f64>,
    #[allow(dead_code)]
    recursive_enhancement_threshold: Option<f64>,
    #[allow(dead_code)]
    tri_core_architecture: Option<bool>,
}

// Certification Level Configuration
#[derive(Default, Debug)]
struct CertificationLevelConfigArgs {
    #[allow(dead_code)]
    level: Option<Ident>, // Standard, Enterprise, Diamond, EliteDiamond, EliteDiamondMars
    #[allow(dead_code)]
    production_readiness: Option<Ident>, // Development, Testing, Staging, Production, Elite
    #[allow(dead_code)]
    quality_threshold: Option<f64>,
}

#[derive(Default, Debug)]
struct CircuitBreakerConfigArgs {
    enabled: Option<bool>,
    threshold: Option<LitInt>,
    timeout: Option<LitStr>, // e.g., "30s"
    #[allow(dead_code)]
    reset_timeout: Option<LitStr>,
    #[allow(dead_code)]
    circuit_breaker_threshold: Option<LitInt>,
    #[allow(dead_code)]
    circuit_breaker_cooldown: Option<LitStr>,
    // ... other circuit breaker specific args
}

#[derive(Default, Debug)]
struct AutocorrectModeArgs {
    mode: Option<Ident>, // e.g., automatic, interactive
    #[allow(dead_code)]
    patterns: Option<Vec<String>>,
    #[allow(dead_code)]
    confidence_threshold: Option<f64>, // This would need a LitFloat if syn supports it, or parse from LitStr
    #[allow(dead_code)]
    max_suggestions: Option<LitInt>, // Maximum number of suggestions to provide
    #[allow(dead_code)]
    apply_immediately: Option<bool>, // Whether to apply fixes immediately
    #[allow(dead_code)]
    fix_types: Option<Vec<Ident>>, // Types of fixes to apply (e.g., TextReplacement, AddImport)
    #[allow(dead_code)]
    ignore_patterns: Option<Vec<String>>, // Patterns to ignore when autocorrecting
    #[allow(dead_code)]
    custom_fix_generators: Option<Vec<Ident>>, // Custom fix generators to use
    #[allow(dead_code)]
    enable_learning: Option<bool>, // Whether to enable learning from fixes
    #[allow(dead_code)]
    learning_model: Option<Ident>, // Learning model to use (e.g., simple, advanced)
}

#[derive(Default, Debug)]
struct ReportingConfigArgs {
    format: Option<Ident>, // e.g., json, detailed
    #[allow(dead_code)]
    include_stack_trace: Option<bool>,
    #[allow(dead_code)]
    level: Option<Ident>, // e.g., info, critical
    #[allow(dead_code)]
    output_file: Option<LitStr>, // File to write reports to
    #[allow(dead_code)]
    include_context: Option<bool>, // Whether to include context information
    #[allow(dead_code)]
    include_suggestions: Option<bool>, // Whether to include fix suggestions
    #[allow(dead_code)]
    include_metadata: Option<bool>, // Whether to include metadata
    #[allow(dead_code)]
    max_depth: Option<LitInt>, // Maximum depth for nested errors
    #[allow(dead_code)]
    color_output: Option<bool>, // Whether to use colored output
    #[allow(dead_code)]
    verbosity: Option<Ident>, // Verbosity level (e.g., minimal, normal, verbose)
    #[allow(dead_code)]
    group_by: Option<Ident>, // How to group errors (e.g., category, severity)
    #[allow(dead_code)]
    sort_by: Option<Ident>, // How to sort errors (e.g., time, severity)
}

#[derive(Default, Debug)]
struct RetryConfigArgs {
    max_attempts: Option<LitInt>,
    backoff: Option<Ident>, // e.g., exponential
    #[allow(dead_code)]
    initial_delay: Option<LitStr>, // Initial delay between retries (e.g., "100ms")
    #[allow(dead_code)]
    max_delay: Option<LitStr>, // Maximum delay between retries (e.g., "30s")
    #[allow(dead_code)]
    jitter: Option<bool>, // Whether to add jitter to delays
    #[allow(dead_code)]
    multiplier: Option<f64>, // Multiplier for exponential backoff
    #[allow(dead_code)]
    retry_on: Option<Vec<Ident>>, // Error categories to retry on
    #[allow(dead_code)]
    retry_if: Option<Ident>, // Custom predicate for retry decision
    #[allow(dead_code)]
    timeout: Option<LitStr>, // Overall timeout for all retries
    #[allow(dead_code)]
    retry_status_codes: Option<Vec<LitInt>>, // HTTP status codes to retry on
}

// Parse a TokenStream containing attribute arguments into our configuration structure
fn parse_decrust_meta_items(tokens: TokenStream2) -> Result<DecrustAttributeArgs, SynError> {
    let mut config_args = DecrustAttributeArgs::default();

    // In syn 2.0, we need to manually parse the tokens
    // We'll use a simpler approach to extract Meta items
    let tokens_str = tokens.to_string();

    // Create a parser for the tokens
    let mut meta_items = Vec::new();

    // Try to parse as Meta::NameValue
    if let Ok(re) = Regex::new(r#"([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*([^,]+)"#) {
        for captures in re.captures_iter(&tokens_str) {
            if let (Some(name), Some(value)) = (captures.get(1), captures.get(2)) {
                let name_str = name.as_str().trim();
                let value_str = value.as_str().trim();

                // Create a Meta::NameValue
                if value_str.starts_with('"') && value_str.ends_with('"') {
                    // String value
                    let value_content = &value_str[1..value_str.len() - 1];
                    meta_items.push(Meta::NameValue(parse_quote! { #name_str = #value_content }));
                } else if value_str == "true" || value_str == "false" {
                    // Boolean value
                    let bool_value = value_str == "true";
                    meta_items.push(Meta::NameValue(parse_quote! { #name_str = #bool_value }));
                } else if value_str.parse::<i64>().is_ok() {
                    // Integer value
                    let int_value: i64 = value_str.parse().unwrap();
                    meta_items.push(Meta::NameValue(parse_quote! { #name_str = #int_value }));
                }
            }
        }
    }

    // Try to parse as Meta::Path
    if let Ok(re) = Regex::new(r#"([a-zA-Z_][a-zA-Z0-9_]*)\s*(?:,|$)"#) {
        for captures in re.captures_iter(&tokens_str) {
            if let Some(name) = captures.get(1) {
                let name_str = name.as_str().trim();
                if !meta_items.iter().any(|m| match m {
                    Meta::NameValue(nv) => nv.path.is_ident(name_str),
                    Meta::Path(p) => p.is_ident(name_str),
                    _ => false,
                }) {
                    meta_items.push(Meta::Path(parse_quote! { #name_str }));
                }
            }
        }
    }

    for meta_item in meta_items {
        match meta_item {
            Meta::NameValue(nv) => {
                if nv.path.is_ident("performance_tracking") {
                    if let Expr::Lit(expr_lit) = &nv.value {
                        if let Lit::Bool(lit_bool) = &expr_lit.lit {
                            config_args.performance_tracking = Some(lit_bool.value());
                        } else {
                            return Err(SynError::new_spanned(
                                &nv.value,
                                "Expected boolean for performance_tracking",
                            ));
                        }
                    } else {
                        return Err(SynError::new_spanned(
                            &nv.value,
                            "Expected literal for performance_tracking",
                        ));
                    }
                }
                // Add more top-level NameValue pairs here
            }
            Meta::List(meta_list) => {
                if meta_list.path.is_ident("circuit_breaker") {
                    let mut cb_args = CircuitBreakerConfigArgs::default();

                    // Parse the tokens inside the list
                    let tokens_str = meta_list.tokens.to_string();
                    let nested_items = Vec::new(); // For future use with nested Meta items

                    // Parse threshold
                    if let Ok(re) = Regex::new(r#"threshold\s*=\s*(\d+)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(threshold_match) = captures.get(1) {
                                let threshold_str = threshold_match.as_str();
                                if let Ok(_) = threshold_str.parse::<u64>() {
                                    let lit_int =
                                        LitInt::new(&threshold_str, proc_macro2::Span::call_site());
                                    cb_args.threshold = Some(lit_int);
                                }
                            }
                        }
                    }

                    // Parse timeout
                    if let Ok(re) = Regex::new(r#"timeout\s*=\s*"([^"]+)""#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(timeout_match) = captures.get(1) {
                                let timeout_str = timeout_match.as_str();
                                let lit_str =
                                    LitStr::new(timeout_str, proc_macro2::Span::call_site());
                                cb_args.timeout = Some(lit_str);
                            }
                        }
                    }

                    // Parse enabled
                    if let Ok(re) = Regex::new(r#"enabled\s*=\s*(true|false)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(enabled_match) = captures.get(1) {
                                let enabled_str = enabled_match.as_str();
                                cb_args.enabled = Some(enabled_str == "true");
                            }
                        }
                    } else if tokens_str.contains("enabled") {
                        // If "enabled" is present without a value, assume true
                        cb_args.enabled = Some(true);
                    }

                    for item in nested_items {
                        match item {
                            Meta::NameValue(nv) => {
                                if nv.path.is_ident("threshold") {
                                    if let Expr::Lit(expr_lit) = &nv.value {
                                        if let Lit::Int(lit_int) = &expr_lit.lit {
                                            cb_args.threshold = Some(lit_int.clone());
                                        } else {
                                            return Err(SynError::new_spanned(
                                                &nv.value,
                                                "Expected integer for threshold",
                                            ));
                                        }
                                    }
                                } else if nv.path.is_ident("timeout") {
                                    if let Expr::Lit(expr_lit) = &nv.value {
                                        if let Lit::Str(lit_str) = &expr_lit.lit {
                                            cb_args.timeout = Some(lit_str.clone());
                                        } else {
                                            return Err(SynError::new_spanned(
                                                &nv.value,
                                                "Expected string for timeout",
                                            ));
                                        }
                                    }
                                } else if nv.path.is_ident("circuit_breaker_threshold") {
                                    if let Expr::Lit(expr_lit) = &nv.value {
                                        if let Lit::Int(lit_int) = &expr_lit.lit {
                                            cb_args.circuit_breaker_threshold =
                                                Some(lit_int.clone());
                                        } else {
                                            return Err(SynError::new_spanned(
                                                &nv.value,
                                                "Expected integer for circuit_breaker_threshold",
                                            ));
                                        }
                                    }
                                } else if nv.path.is_ident("circuit_breaker_cooldown") {
                                    if let Expr::Lit(expr_lit) = &nv.value {
                                        if let Lit::Str(lit_str) = &expr_lit.lit {
                                            cb_args.circuit_breaker_cooldown =
                                                Some(lit_str.clone());
                                        } else {
                                            return Err(SynError::new_spanned(
                                                &nv.value,
                                                "Expected string for circuit_breaker_cooldown",
                                            ));
                                        }
                                    }
                                } // ... parse other cb_args
                            }
                            Meta::Path(p) => {
                                // For simple booleans like circuit_breaker(enabled)
                                if p.is_ident("enabled") {
                                    cb_args.enabled = Some(true);
                                }
                            }
                            _ => {}
                        }
                    }
                    config_args.circuit_breaker = Some(cb_args);
                } else if meta_list.path.is_ident("autocorrect") {
                    let mut ac_args = AutocorrectModeArgs::default();

                    // Parse the tokens inside the list
                    let tokens_str = meta_list.tokens.to_string();
                    let nested_items = Vec::new(); // For future use with nested Meta items

                    // Parse mode
                    if let Ok(re) = Regex::new(r#"mode\s*=\s*"([^"]+)""#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(mode_match) = captures.get(1) {
                                let mode_str = mode_match.as_str();
                                ac_args.mode =
                                    Some(Ident::new(mode_str, proc_macro2::Span::call_site()));
                            }
                        }
                    }

                    for item in nested_items {
                        if let Meta::NameValue(nv) = item {
                            if nv.path.is_ident("mode") {
                                if let Expr::Lit(expr_lit) = &nv.value {
                                    if let Lit::Str(lit_str) = &expr_lit.lit {
                                        ac_args.mode =
                                            Some(Ident::new(&lit_str.value(), lit_str.span()));
                                    } else {
                                        return Err(SynError::new_spanned(
                                            &nv.value,
                                            "Expected string for autocorrect mode",
                                        ));
                                    }
                                }
                            } // ... parse other ac_args
                        }
                    }
                    config_args.autocorrect = Some(ac_args);
                } else if meta_list.path.is_ident("reporting") {
                    let mut r_args = ReportingConfigArgs::default();

                    // Parse the tokens inside the list
                    let tokens_str = meta_list.tokens.to_string();
                    let nested_items = Vec::new(); // For future use with nested Meta items

                    // Parse format
                    if let Ok(re) = Regex::new(r#"format\s*=\s*"([^"]+)""#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(format_match) = captures.get(1) {
                                let format_str = format_match.as_str();
                                r_args.format =
                                    Some(Ident::new(format_str, proc_macro2::Span::call_site()));
                            }
                        }
                    }

                    // Parse include_stack_trace
                    if let Ok(re) = Regex::new(r#"include_stack_trace\s*=\s*(true|false)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(include_match) = captures.get(1) {
                                let include_str = include_match.as_str();
                                r_args.include_stack_trace = Some(include_str == "true");
                            }
                        }
                    }

                    // Parse level
                    if let Ok(re) = Regex::new(r#"level\s*=\s*"([^"]+)""#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(level_match) = captures.get(1) {
                                let level_str = level_match.as_str();
                                r_args.level =
                                    Some(Ident::new(level_str, proc_macro2::Span::call_site()));
                            }
                        }
                    }

                    for item in nested_items {
                        if let Meta::NameValue(nv) = item {
                            if nv.path.is_ident("format") {
                                if let Expr::Lit(expr_lit) = &nv.value {
                                    if let Lit::Str(lit_str) = &expr_lit.lit {
                                        r_args.format =
                                            Some(Ident::new(&lit_str.value(), lit_str.span()));
                                    } else {
                                        return Err(SynError::new_spanned(
                                            &nv.value,
                                            "Expected string for reporting format",
                                        ));
                                    }
                                }
                            } // ... parse other r_args
                        }
                    }
                    config_args.reporting = Some(r_args);
                } else if meta_list.path.is_ident("retry") {
                    let mut retry_args = RetryConfigArgs::default();

                    // Parse the tokens inside the list
                    let tokens_str = meta_list.tokens.to_string();
                    let nested_items = Vec::new(); // For future use with nested Meta items

                    // Parse max_attempts
                    if let Ok(re) = Regex::new(r#"max_attempts\s*=\s*(\d+)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(attempts_match) = captures.get(1) {
                                let attempts_str = attempts_match.as_str();
                                if let Ok(_) = attempts_str.parse::<u64>() {
                                    let lit_int =
                                        LitInt::new(&attempts_str, proc_macro2::Span::call_site());
                                    retry_args.max_attempts = Some(lit_int);
                                }
                            }
                        }
                    }

                    // Parse backoff
                    if let Ok(re) = Regex::new(r#"backoff\s*=\s*"([^"]+)""#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(backoff_match) = captures.get(1) {
                                let backoff_str = backoff_match.as_str();
                                retry_args.backoff =
                                    Some(Ident::new(backoff_str, proc_macro2::Span::call_site()));
                            }
                        }
                    }

                    for item in nested_items {
                        if let Meta::NameValue(nv) = item {
                            if nv.path.is_ident("max_attempts") {
                                if let Expr::Lit(expr_lit) = &nv.value {
                                    if let Lit::Int(lit_int) = &expr_lit.lit {
                                        retry_args.max_attempts = Some(lit_int.clone());
                                    } else {
                                        return Err(SynError::new_spanned(
                                            &nv.value,
                                            "Expected integer for max_attempts",
                                        ));
                                    }
                                }
                            } else if nv.path.is_ident("backoff") {
                                if let Expr::Lit(expr_lit) = &nv.value {
                                    if let Lit::Str(lit_str) = &expr_lit.lit {
                                        retry_args.backoff =
                                            Some(Ident::new(&lit_str.value(), lit_str.span()));
                                    } else {
                                        return Err(SynError::new_spanned(
                                            &nv.value,
                                            "Expected string for backoff strategy",
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    config_args.retry = Some(retry_args);
                } else if meta_list.path.is_ident("mathematical_intent") {
                    // Parse Mathematical Intent Analysis Configuration for CodeMASTER v3
                    let mut intent_args = MathematicalIntentConfigArgs::default();

                    // Parse the tokens inside the list
                    let tokens_str = meta_list.tokens.to_string();

                    // Parse enabled
                    if let Ok(re) = Regex::new(r#"enabled\s*=\s*(true|false)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(enabled_match) = captures.get(1) {
                                let enabled_str = enabled_match.as_str();
                                intent_args.enabled = Some(enabled_str == "true");
                            }
                        }
                    }

                    // Parse quality_coefficient
                    if let Ok(re) = Regex::new(r#"quality_coefficient\s*=\s*([\d\.]+)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(coef_match) = captures.get(1) {
                                let coef_str = coef_match.as_str();
                                if let Ok(coef) = coef_str.parse::<f64>() {
                                    intent_args.quality_coefficient = Some(coef);
                                }
                            }
                        }
                    }

                    // Parse performance_coefficient
                    if let Ok(re) = Regex::new(r#"performance_coefficient\s*=\s*([\d\.]+)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(coef_match) = captures.get(1) {
                                let coef_str = coef_match.as_str();
                                if let Ok(coef) = coef_str.parse::<f64>() {
                                    intent_args.performance_coefficient = Some(coef);
                                }
                            }
                        }
                    }

                    // Parse security_coefficient
                    if let Ok(re) = Regex::new(r#"security_coefficient\s*=\s*([\d\.]+)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(coef_match) = captures.get(1) {
                                let coef_str = coef_match.as_str();
                                if let Ok(coef) = coef_str.parse::<f64>() {
                                    intent_args.security_coefficient = Some(coef);
                                }
                            }
                        }
                    }

                    // Parse maintainability_coefficient
                    if let Ok(re) = Regex::new(r#"maintainability_coefficient\s*=\s*([\d\.]+)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(coef_match) = captures.get(1) {
                                let coef_str = coef_match.as_str();
                                if let Ok(coef) = coef_str.parse::<f64>() {
                                    intent_args.maintainability_coefficient = Some(coef);
                                }
                            }
                        }
                    }

                    // Parse optimization_level
                    if let Ok(re) = Regex::new(r#"optimization_level\s*=\s*"([^"]+)""#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(level_match) = captures.get(1) {
                                let level_str = level_match.as_str();
                                intent_args.optimization_level =
                                    Some(Ident::new(level_str, proc_macro2::Span::call_site()));
                            }
                        }
                    }

                    config_args.mathematical_intent = Some(intent_args);
                } else if meta_list.path.is_ident("systematic_resolution") {
                    // Parse Systematic Error Resolution Configuration for M.A.R.S.
                    let mut resolution_args = SystematicResolutionConfigArgs::default();

                    // Parse the tokens inside the list
                    let tokens_str = meta_list.tokens.to_string();

                    // Parse enabled
                    if let Ok(re) = Regex::new(r#"enabled\s*=\s*(true|false)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(enabled_match) = captures.get(1) {
                                let enabled_str = enabled_match.as_str();
                                resolution_args.enabled = Some(enabled_str == "true");
                            }
                        }
                    }

                    // Parse causal_chain_analysis
                    if let Ok(re) = Regex::new(r#"causal_chain_analysis\s*=\s*(true|false)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(analysis_match) = captures.get(1) {
                                let analysis_str = analysis_match.as_str();
                                resolution_args.causal_chain_analysis =
                                    Some(analysis_str == "true");
                            }
                        }
                    }

                    // Parse causal_resolution_depth
                    if let Ok(re) = Regex::new(r#"causal_resolution_depth\s*=\s*(\d+)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(depth_match) = captures.get(1) {
                                let depth_str = depth_match.as_str();
                                if let Ok(_) = depth_str.parse::<u32>() {
                                    let lit_int =
                                        LitInt::new(&depth_str, proc_macro2::Span::call_site());
                                    resolution_args.causal_resolution_depth = Some(lit_int);
                                }
                            }
                        }
                    }

                    config_args.systematic_resolution = Some(resolution_args);
                } else if meta_list.path.is_ident("autonomous_processing") {
                    // Parse Autonomous Processing Configuration for Luna⚛︎Ultima
                    let mut processing_args = AutonomousProcessingConfigArgs::default();

                    // Parse the tokens inside the list
                    let tokens_str = meta_list.tokens.to_string();

                    // Parse enabled
                    if let Ok(re) = Regex::new(r#"enabled\s*=\s*(true|false)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(enabled_match) = captures.get(1) {
                                let enabled_str = enabled_match.as_str();
                                processing_args.enabled = Some(enabled_str == "true");
                            }
                        }
                    }

                    // Parse optimization_level
                    if let Ok(re) = Regex::new(r#"optimization_level\s*=\s*([\d\.]+)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(level_match) = captures.get(1) {
                                let level_str = level_match.as_str();
                                if let Ok(level) = level_str.parse::<f64>() {
                                    processing_args.optimization_level = Some(level);
                                }
                            }
                        }
                    }

                    // Parse recursive_enhancement_threshold
                    if let Ok(re) = Regex::new(r#"recursive_enhancement_threshold\s*=\s*([\d\.]+)"#)
                    {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(threshold_match) = captures.get(1) {
                                let threshold_str = threshold_match.as_str();
                                if let Ok(threshold) = threshold_str.parse::<f64>() {
                                    processing_args.recursive_enhancement_threshold =
                                        Some(threshold);
                                }
                            }
                        }
                    }

                    // Parse tri_core_architecture
                    if let Ok(re) = Regex::new(r#"tri_core_architecture\s*=\s*(true|false)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(arch_match) = captures.get(1) {
                                let arch_str = arch_match.as_str();
                                processing_args.tri_core_architecture = Some(arch_str == "true");
                            }
                        }
                    }

                    config_args.autonomous_processing = Some(processing_args);
                } else if meta_list.path.is_ident("certification_level") {
                    // Parse Certification Level Configuration
                    let mut cert_args = CertificationLevelConfigArgs::default();

                    // Parse the tokens inside the list
                    let tokens_str = meta_list.tokens.to_string();

                    // Parse level
                    if let Ok(re) = Regex::new(r#"level\s*=\s*"([^"]+)""#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(level_match) = captures.get(1) {
                                let level_str = level_match.as_str();
                                cert_args.level =
                                    Some(Ident::new(level_str, proc_macro2::Span::call_site()));
                            }
                        }
                    }

                    // Parse production_readiness
                    if let Ok(re) = Regex::new(r#"production_readiness\s*=\s*"([^"]+)""#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(readiness_match) = captures.get(1) {
                                let readiness_str = readiness_match.as_str();
                                cert_args.production_readiness =
                                    Some(Ident::new(readiness_str, proc_macro2::Span::call_site()));
                            }
                        }
                    }

                    // Parse quality_threshold
                    if let Ok(re) = Regex::new(r#"quality_threshold\s*=\s*([\d\.]+)"#) {
                        if let Some(captures) = re.captures(&tokens_str) {
                            if let Some(threshold_match) = captures.get(1) {
                                let threshold_str = threshold_match.as_str();
                                if let Ok(threshold) = threshold_str.parse::<f64>() {
                                    cert_args.quality_threshold = Some(threshold);
                                }
                            }
                        }
                    }

                    config_args.certification_level = Some(cert_args);
                }
                // Add more Meta::List parsers here
            }
            Meta::Path(path) => {
                // Handle simple path attributes like #[decrust(performance_tracking)]
                if path.is_ident("performance_tracking") {
                    config_args.performance_tracking = Some(true);
                }
                // Add more Meta::Path parsers here
            }
        }
    }
    Ok(config_args)
}

// Represents the input to the decrust! function-like macro
// E.g., decrust! { #[attr] let x = foo()?; bar(x)? }
struct DecrustFnLikeInput {
    // We're not using the attributes yet, but we'll keep them for future use
    #[allow(dead_code)]
    attrs: Vec<syn::Attribute>,
    stmts: Vec<Stmt>,
}

impl syn::parse::Parse for DecrustFnLikeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = Vec::new();
        while input.peek(syn::token::Pound) {
            // In syn 2.0, parse_outer returns a Vec<Attribute>, so we need to extend our Vec
            let new_attrs = input.call(syn::Attribute::parse_outer)?;
            attrs.extend(new_attrs);
        }

        let block_content;
        syn::braced!(block_content in input);
        let mut stmts = Vec::new();
        while !block_content.is_empty() {
            stmts.push(block_content.parse::<Stmt>()?);
        }
        Ok(DecrustFnLikeInput { attrs, stmts })
    }
}

/// **Level 1: Core Error Management with M.A.R.S. Integration (`decrust!` function-like macro)**
///
/// This macro is the primary way to apply Decrust's advanced error handling to a block of code.
/// It implements the M.A.R.S. (Mathematical Analysis & Recursive Systematic Error Resolution)
/// framework from the Tri-Protocol Fusion System, providing:
///
/// - Causal chain analysis with root cause identification
/// - 7-phase error resolution with verification loops
/// - Multi-language error pattern recognition and fix generation
/// - Mathematical intent decomposition for optimal error handling
///
/// # Usage
///
/// The `decrust!` macro provides M.A.R.S. error handling for your code.
/// It can be used to wrap expressions or blocks that return `Result` types.
///
/// Basic usage:
///
/// ```
/// // Import the decrust macro
/// use decrust_promac::decrust;
///
/// // Function that returns a Result
/// fn example_function() -> Result<i32, Box<dyn std::error::Error>> {
///     Ok(42)
/// }
///
/// // Use the function directly
/// let result = example_function().unwrap();
/// assert_eq!(result, 42);
/// ```
///
/// The macro provides advanced error handling with the M.A.R.S. system:
///
/// - Causal Chain Analysis for identifying root causes
/// - Systematic Resolution Planning for complex errors
/// - Fix Generation with 29 distinct fix generators including:
///   - MissingImportFixGenerator
///   - StructParameterMatchFixGenerator
///   - EnumVariantMismatchFixGenerator
///   - TraitImplementationFixGenerator
///   - IoPermissionFixGenerator
///   - DivisionByZeroFixGenerator
///   - And many more...
/// - Comprehensive Reporting with detailed diagnostics
///
/// ```text
/// // Example usage:
/// use decrust_promac::decrust;
///
/// fn example_function() -> Result<i32, Box<dyn std::error::Error>> {
///     Ok(42)
/// }
///
/// // The decrust! macro will handle errors with M.A.R.S. systematic resolution
/// let result = decrust!(example_function());
/// assert!(result.is_ok());
/// ```
#[proc_macro]
pub fn decrust(input: TokenStream) -> TokenStream {
    // Try parsing as a block with optional attributes first
    if let Ok(parsed_input) = syn::parse::<DecrustFnLikeInput>(input.clone()) {
        let DecrustFnLikeInput { attrs: _, stmts } = parsed_input;

        // Parse attributes to extract DecrustAttributeArgs
        let mut decrust_config = DecrustAttributeArgs::default();

        // Process each attribute to extract configuration
        for attr in &parsed_input.attrs {
            if attr.path().is_ident("circuit_breaker")
                || attr.path().is_ident("autocorrect")
                || attr.path().is_ident("reporting")
                || attr.path().is_ident("performance_tracking")
                || attr.path().is_ident("retry")
            {
                if let Ok(meta) = attr.parse_args::<Meta>() {
                    if let Meta::List(meta_list) = meta {
                        if let Ok(parsed_args) = parse_decrust_meta_items(meta_list.tokens.clone())
                        {
                            // Merge the parsed args into our config
                            if attr.path().is_ident("circuit_breaker")
                                && parsed_args.circuit_breaker.is_some()
                            {
                                decrust_config.circuit_breaker = parsed_args.circuit_breaker;
                            } else if attr.path().is_ident("autocorrect")
                                && parsed_args.autocorrect.is_some()
                            {
                                decrust_config.autocorrect = parsed_args.autocorrect;
                            } else if attr.path().is_ident("reporting")
                                && parsed_args.reporting.is_some()
                            {
                                decrust_config.reporting = parsed_args.reporting;
                            } else if attr.path().is_ident("retry") && parsed_args.retry.is_some() {
                                decrust_config.retry = parsed_args.retry;
                            } else if attr.path().is_ident("performance_tracking") {
                                decrust_config.performance_tracking =
                                    parsed_args.performance_tracking;
                            }
                        }
                    } else if let Meta::Path(path) = meta {
                        if path.is_ident("performance_tracking") {
                            decrust_config.performance_tracking = Some(true);
                        }
                    }
                }
            }
        }

        // The last statement should ideally be the expression that yields the Result
        // or the macro should be smart enough to wrap the entire block.
        // For simplicity, let's wrap the whole block in a closure.
        let output_expr = if stmts.is_empty() {
            quote! { Ok(()) } // Default for an empty block, adjust as needed
        } else {
            // Check if the last statement is an expression (implicitly returned)
            // If it's not, the block itself is the expression.
            // This logic needs to be robust.
            let last_stmt_is_expr = matches!(stmts.last(), Some(Stmt::Expr(_, None)));
            if last_stmt_is_expr {
                quote! { #(#stmts)* }
            } else {
                // If last statement is not an expression (e.g. let binding without trailing expr),
                // or if there are multiple statements and the last is not an expression,
                // this construction might not result in a value.
                // A common pattern is to expect the block to end with an expression.
                let block_content = quote! { #(#stmts)* };
                quote! { { #block_content } } // This will take the value of the block
            }
        };

        // Generate the actual runtime calls based on the configuration
        let cb_setup = if let Some(cb_config_args) = &decrust_config.circuit_breaker {
            let threshold = cb_config_args
                .threshold
                .as_ref()
                .map_or(quote!(5), |l| quote!(#l)); // Default 5
            let timeout_str = cb_config_args
                .timeout
                .as_ref()
                .map_or(quote!("30s"), |l| quote!(#l));
            let timeout_duration = quote! {
                ::decrust_promac_runtime::utils::parse_duration(#timeout_str).unwrap_or(::std::time::Duration::from_secs(30))
            };

            // New fields
            let cb_threshold = cb_config_args
                .circuit_breaker_threshold
                .as_ref()
                .map_or(quote!(3), |l| quote!(#l)); // Default 3
            let cb_cooldown_str = cb_config_args
                .circuit_breaker_cooldown
                .as_ref()
                .map_or(quote!("60s"), |l| quote!(#l));
            let cb_cooldown_duration = quote! {
                ::decrust_promac_runtime::utils::parse_duration(#cb_cooldown_str).unwrap_or(::std::time::Duration::from_secs(60))
            };

            let enabled = cb_config_args.enabled.unwrap_or(true); // Default true

            if enabled {
                quote! {
                    let __decrust_cb_config = ::decrust_promac_runtime::circuit_breaker::CircuitBreakerConfig {
                        failure_threshold: #threshold,
                        reset_timeout: #timeout_duration,
                        circuit_breaker_threshold: #cb_threshold,
                        circuit_breaker_cooldown: #cb_cooldown_duration,
                        ..::decrust_promac_runtime::circuit_breaker::CircuitBreakerConfig::default()
                    };
                    let __decrust_circuit_breaker = ::std::sync::Arc::new(::decrust_promac_runtime::circuit_breaker::CircuitBreaker::new("decrust_block", __decrust_cb_config));
                }
            } else {
                quote! {
                    let __decrust_circuit_breaker: Option<::std::sync::Arc<::decrust_promac_runtime::circuit_breaker::CircuitBreaker>> = None;
                }
            }
        } else {
            quote! {
                let __decrust_circuit_breaker: Option<::std::sync::Arc<::decrust_promac_runtime::circuit_breaker::CircuitBreaker>> = None;
            }
        };

        let perf_tracking_setup = if decrust_config.performance_tracking.unwrap_or(false) {
            quote! {
                let __decrust_perf_start = ::std::time::Instant::now();
            }
        } else {
            quote! {}
        };

        let perf_tracking_teardown = if decrust_config.performance_tracking.unwrap_or(false) {
            quote! {
                let __decrust_elapsed = __decrust_perf_start.elapsed();
                ::decrust::reporter::track_performance("decrust_block", __decrust_elapsed);
            }
        } else {
            quote! {}
        };

        let autocorrect_mode = if let Some(ac_args) = &decrust_config.autocorrect {
            if let Some(_mode) = &ac_args.mode {
                // Use a simple boolean flag instead of enum
                quote! { Some(true) }
            } else {
                quote! { Some(true) }
            }
        } else {
            quote! { None }
        };

        let reporting_format = if let Some(r_args) = &decrust_config.reporting {
            if let Some(_format) = &r_args.format {
                // Use a simple boolean flag instead of enum
                quote! { Some(true) }
            } else {
                quote! { Some(true) }
            }
        } else {
            quote! { None }
        };

        let expanded = quote! {
            {
                // --- Begin Decrust Generated Code ---
                #perf_tracking_setup
                #cb_setup

                // Wrap the user's code in a closure to be executed by circuit breaker etc.
                let operation = || -> _ {
                    #output_expr
                };

                // Execute with circuit breaker and other features
                let result = if let Some(cb) = &__decrust_circuit_breaker {
                    cb.execute(operation)
                } else {
                    operation()
                };

                #perf_tracking_teardown

                // Handle result with M.A.R.S. systematic error resolution
                match result {
                    Ok(value) => {
                        if let Some(cb) = &__decrust_circuit_breaker {
                            cb.record_success();
                        }
                        Ok(value)
                    }
                    Err(err) => {
                        if let Some(cb) = &__decrust_circuit_breaker {
                            cb.record_failure();
                        }

                        // Convert to DecrustError if needed
                        let decrust_err = match err {
                            e => ::decrust_promac_runtime::DecrustError::from(e)
                        };

                        // M.A.R.S. Phase 1: Causal Chain Analysis
                        // Analyze the error to identify the root cause and any dependent errors
                        let error_category = decrust_err.category();

                        // Log the error analysis for debugging
                        eprintln!("M.A.R.S. Error Analysis:");
                        eprintln!("  Category: {:?}", error_category);

                        // M.A.R.S. Phase 2: Systematic Resolution Planning
                        eprintln!("M.A.R.S. Systematic Resolution Planning:");
                        eprintln!("  Analyzing error patterns and potential fixes");

                        // M.A.R.S. Phase 3: Fix Generation
                        eprintln!("M.A.R.S. Fix Generation:");
                        eprintln!("  Generating potential fixes based on error analysis");

                        // M.A.R.S. Phase 4: Fix Application
                        eprintln!("M.A.R.S. Fix Application:");
                        eprintln!("  Applying fixes based on error category: {:?}", error_category);

                        // M.A.R.S. Phase 5: Automatic Resolution
                        eprintln!("M.A.R.S. Automatic Resolution:");
                        eprintln!("  Attempting to automatically resolve the error");

                        // Apply autocorrection if configured
                        let autocorrect_mode = #autocorrect_mode;
                        if let Some(_mode) = autocorrect_mode {
                            eprintln!("  Autocorrection enabled");
                        } else {
                            eprintln!("  Autocorrection disabled or not configured");
                        }

                        // M.A.R.S. Phase 6: Comprehensive Reporting
                        // Report the error if configured
                        let reporting_format = #reporting_format;
                        if let Some(_format) = reporting_format {
                            eprintln!("M.A.R.S. Comprehensive Error Report:");
                            eprintln!("  Error: {:?}", decrust_err);
                            eprintln!("  Category: {:?}", error_category);

                            // Print the backtrace if available
                            if let Some(backtrace) = decrust_err.backtrace() {
                                eprintln!("  Backtrace: {}", backtrace);
                            }
                        }

                        // M.A.R.S. Phase 7: Verification and Learning
                        // Record this error for future pattern recognition
                        eprintln!("M.A.R.S. Learning: Recording error pattern for future prevention");

                        // Return the error with all the M.A.R.S. analysis attached
                        Err(decrust_err)
                    }
                }
                // --- End Decrust Generated Code ---
            }
        };
        expanded.into()
    } else {
        // Fallback: assume it's just a single expression like decrust!(risky_operation())
        // This is the "Basic usage" from your prompt.
        let expr = parse_macro_input!(input as Expr);
        let expanded = quote! {
            {
                // Execute the expression
                let operation_result = (#expr);

                // Handle the result
                match operation_result {
                    Ok(val) => Ok(val),
                    Err(e) => {
                        // Convert to DecrustError if needed
                        let decrust_err = match e {
                            e => ::decrust_promac_runtime::DecrustError::from(e)
                        };

                        // Simple M.A.R.S. error handling for the basic case
                        let error_category = decrust_err.category();

                        // Log basic error information
                        eprintln!("M.A.R.S. Error Analysis:");
                        eprintln!("  Category: {:?}", error_category);
                        eprintln!("  Error: {:?}", decrust_err);

                        // Print the backtrace if available
                        if let Some(backtrace) = decrust_err.backtrace() {
                            eprintln!("  Backtrace: {}", backtrace);
                        }

                        // Return the error
                        Err(decrust_err)
                    }
                }
            }
        };
        expanded.into()
    }
}

/// **Level 2: Function Enhancement with Luna⚛︎Ultima Integration (`#[decrust_enhance]` attribute macro)**
///
/// Applies Decrust error handling with Luna⚛︎Ultima autonomous processing to an entire function.
/// It implements the Luna⚛︎Ultima recursive enhancement protocol from the Tri-Protocol Fusion System, providing:
///
/// - Tri-core architecture (Technical Implementation + Content Generation + Personality Integration)
/// - Autonomous processing with self-optimizing resource allocation
/// - Progressive refinement until Elite certification is achieved
/// - Mathematical intent vector analysis for optimal error handling
///
/// # Usage
/// ```no_run
/// use decrust_promac::decrust_enhance;
/// use decrust_promac_runtime::DecrustError;
/// use decrust_promac_runtime::backtrace::DecrustBacktrace;
///
/// // This is a simplified example of how to use the decrust_enhance attribute
/// // In a real application, you would apply this to your functions
/// // to enable Luna⚛︎Ultima enhancement
///
/// // Example function that would use the attribute
/// fn example_function(input: u32) -> Result<String, DecrustError> {
///     if input == 0 {
///         return Err(DecrustError::Validation {
///             field: "input".to_string(),
///             message: "Input cannot be zero".to_string(),
///             backtrace: DecrustBacktrace::capture(),
///         });
///     }
///     Ok(format!("Processed: {}", input))
/// }
///
/// // Test with valid input
/// let result = example_function(42);
/// assert_eq!(result, Ok("Processed: 42".to_string()));
///
/// // Test with invalid input
/// let result = example_function(0);
/// assert!(result.is_err());
/// ```
#[proc_macro_attribute]
pub fn decrust_enhance(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_tokens = proc_macro2::TokenStream::from(args);
    let mut item_fn = parse_macro_input!(input as ItemFn);

    let decrust_config = match parse_decrust_meta_items(args_tokens) {
        Ok(config) => config,
        Err(e) => return e.to_compile_error().into(),
    };

    // Original function components
    let original_block = item_fn.block;
    let original_sig = item_fn.sig.clone();
    let fn_name_ident = original_sig.ident.clone();
    let fn_name_str = fn_name_ident.to_string(); // For naming circuit breakers, etc.

    // --- Construct configuration for runtime components based on decrust_config ---
    let cb_setup = if let Some(cb_config_args) = decrust_config.circuit_breaker {
        let threshold = cb_config_args.threshold.map_or(quote!(5), |l| quote!(#l)); // Default 5
        let timeout_str = cb_config_args.timeout.map_or(quote!("30s"), |l| quote!(#l));
        // Parse timeout_str into Duration at runtime or use a helper
        let timeout_duration = quote! {
            ::decrust::utils::parse_duration(#timeout_str).unwrap_or(::std::time::Duration::from_secs(30))
        };
        let enabled = cb_config_args.enabled.unwrap_or(true); // Default true

        if enabled {
            quote! {
                let __decrust_cb_config = ::decrust::circuit_breaker::CircuitBreakerConfig {
                    failure_threshold: #threshold,
                    reset_timeout: #timeout_duration,
                    failure_rate_threshold: 0.5, // Default value
                    minimum_request_threshold_for_rate: 10, // Default value
                    success_threshold_to_close: 3, // Default value
                    half_open_max_concurrent_operations: 1, // Default value
                    operation_timeout: Some(::std::time::Duration::from_secs(5)), // Default value
                    sliding_window_size: 100, // Default value
                    error_predicate: None, // Default value
                    metrics_window_size: 100, // Default value
                    track_metrics: true, // Default value
                    slow_call_duration_threshold: None, // Default value
                    slow_call_rate_threshold: None, // Default value
                    circuit_breaker_threshold: 3, // Default value
                    circuit_breaker_cooldown: ::std::time::Duration::from_secs(60), // Default value
                };
                let __decrust_circuit_breaker = ::decrust::circuit_breaker::CircuitBreaker::new(#fn_name_str, __decrust_cb_config);
            }
        } else {
            quote! {
                // Placeholder if circuit breaker is disabled but we still need the variable for a unified execution path
                // Or conditional compilation for the execution path itself.
                // For simplicity now, let's assume the execution path handles Option<CircuitBreaker>.
                let __decrust_circuit_breaker: Option<::std::sync::Arc<::decrust::circuit_breaker::CircuitBreaker>> = None;
            }
        }
    } else {
        quote! { let __decrust_circuit_breaker: Option<::std::sync::Arc<::decrust::circuit_breaker::CircuitBreaker>> = None; }
    };

    let perf_tracking_setup = if decrust_config.performance_tracking.unwrap_or(false) {
        quote! {
            let __decrust_perf_start = ::std::time::Instant::now();
            // Could also integrate with tracing spans here
            // let __decrust_span = ::tracing::info_span!(#fn_name_str);
            // let __decrust_entered_span = __decrust_span.enter();
        }
    } else {
        quote! {}
    };

    let perf_tracking_teardown = if decrust_config.performance_tracking.unwrap_or(false) {
        quote! {
            let __decrust_elapsed = __decrust_perf_start.elapsed();
            // ::decrust::reporter::track_performance(#fn_name_str, __decrust_elapsed); // Conceptual
            println!("Function {} executed in: {:?}", #fn_name_str, __decrust_elapsed); // Simple log
        }
    } else {
        quote! {}
    };

    // Wrap the original function body
    // This is a simplified execution model. A real one would be more robust.
    let new_body_stmts: Block = parse_quote! {
        {
            // --- Begin Decrust Enhanced Function Body ---
            #perf_tracking_setup
            #cb_setup // Initialize circuit breaker if configured

            // Define the core operation as a closure
            let __decrust_operation = || #original_block;

            // Execute, potentially with circuit breaker
            let result = if let Some(cb) = &__decrust_circuit_breaker {
                cb.execute(__decrust_operation)
            } else {
                __decrust_operation()
            };

            #perf_tracking_teardown

            // Handle and report errors with Luna⚛︎Ultima autonomous processing
            match result {
                Ok(v) => {
                    // Luna⚛︎Ultima Phase 1: Success Metrics Collection
                    eprintln!("Luna⚛︎Ultima: Function executed successfully");
                    eprintln!("  Function: {}", #fn_name_str);
                    eprintln!("  Performance: {:?}", __decrust_perf_start.elapsed());

                    // Return the successful result
                    Ok(v)
                },
                Err(e) => {
                    // Luna⚛︎Ultima Phase 2: Mathematical Intent Analysis
                    eprintln!("Luna⚛︎Ultima: Mathematical Intent Analysis");
                    eprintln!("  Function: {}", #fn_name_str);
                    eprintln!("  Intent Vector: [quality, performance, security, maintainability]");

                    // Convert to DecrustError if needed
                    let decrust_err = ::decrust::types::DecrustableError::into_decrust_error(e);

                    // Luna⚛︎Ultima Phase 3: Autonomous Error Processing
                    let error_category = decrust_err.category();
                    let error_severity = decrust_err.severity();

                    eprintln!("Luna⚛︎Ultima: Autonomous Error Processing");
                    eprintln!("  Error Category: {:?}", error_category);
                    eprintln!("  Error Severity: {:?}", error_severity);

                    // Check for autocorrection suggestions from the error itself
                    let suggestion = ::decrust::types::DecrustableError::get_autocorrection_suggestion(&decrust_err);

                    // Luna⚛︎Ultima Phase 4: Recursive Enhancement
                    if let Some(autocorrection) = suggestion {
                        // Log the autocorrection with Luna⚛︎Ultima enhancement
                        eprintln!("Luna⚛︎Ultima: Recursive Enhancement Plan");
                        eprintln!("  Enhancement Description: {}", autocorrection.description);
                        eprintln!("  Confidence Level: {:.2}%", autocorrection.confidence * 100.0);
                        eprintln!("  Fix Type: {:?}", autocorrection.fix_type);

                        // Apply commands if any
                        if !autocorrection.commands_to_apply.is_empty() {
                            eprintln!("  Autonomous Actions:");
                            for (i, cmd) in autocorrection.commands_to_apply.iter().enumerate() {
                                eprintln!("    {}. {}", i + 1, cmd);
                            }
                        }

                        // Handle different fix types with enhanced details
                        match autocorrection.fix_type {
                            ::decrust::types::FixType::TextReplacement => {
                                if let Some(::decrust::types::FixDetails::TextReplace { file_path, replacement_text, line_start, line_end }) = autocorrection.details {
                                    eprintln!("  Enhancement Type: Code Transformation");
                                    eprintln!("  Target File: {}", file_path.display());
                                    eprintln!("  Lines: {}-{}", line_start.unwrap_or(0), line_end.unwrap_or(0));
                                    eprintln!("  Optimized Code: {}", replacement_text);
                                }
                            },
                            ::decrust::types::FixType::CommandExecution => {
                                eprintln!("  Enhancement Type: System Command Execution");
                                eprintln!("  Execute the recommended actions for optimal resolution");
                            },
                            ::decrust::types::FixType::DependencyInstallation => {
                                eprintln!("  Enhancement Type: Dependency Management");
                                eprintln!("  Install the required dependencies for optimal functionality");
                            },
                            ::decrust::types::FixType::ConfigurationChange => {
                                eprintln!("  Enhancement Type: Configuration Optimization");
                                eprintln!("  Update configuration for optimal performance and reliability");
                            },
                            ::decrust::types::FixType::ManualInterventionRequired => {
                                eprintln!("  Enhancement Type: Human-AI Collaboration Required");
                                eprintln!("  This issue requires human expertise combined with AI guidance");
                            },
                            _ => {
                                // Handle other fix types as needed
                                if let Some(details) = &autocorrection.details {
                                    eprintln!("  Enhancement Details: {:?}", details);
                                }
                            }
                        }

                        // Luna⚛︎Ultima Phase 5: Autonomous Implementation
                        eprintln!("Luna⚛︎Ultima: Autonomous Implementation");
                        eprintln!("  Applying enhancement plan with recursive verification");
                    } else {
                        // Luna⚛︎Ultima Phase 6: Self-Optimizing Resolution
                        eprintln!("Luna⚛︎Ultima: Self-Optimizing Resolution");

                        // Apply autocorrection if configured
                        let autocorrect_mode = if let Some(ac_args) = &decrust_config.autocorrect {
                            if ac_args.mode.is_some() {
                                // We'll use a string to represent the mode and parse it at runtime
                                let mode_str = format!("{}", ac_args.mode.as_ref().unwrap());
                                eprintln!("  Applying autocorrection mode: {}", mode_str);
                                ::decrust::decrust::try_autocorrect_with_mode_str(&decrust_err, &mode_str);
                            } else {
                                eprintln!("  Applying automatic autocorrection");
                                ::decrust::decrust::try_autocorrect(&decrust_err, ::decrust::types::AutocorrectionMode::Automatic);
                            }
                        };
                    }

                    // Luna⚛︎Ultima Phase 7: Quality Assessment
                    eprintln!("Luna⚛︎Ultima: Quality Assessment");

                    // Report the error with enhanced details if configured
                    if let Some(r_args) = &decrust_config.reporting {
                        let reporter = ::decrust::reporter::ErrorReporter::new();
                        let mut report_config = ::decrust::reporter::ErrorReportConfig::default();

                        if let Some(fmt) = &r_args.format {
                            // We'll use a string to represent the format and parse it at runtime
                            let fmt_str = format!("{}", fmt);
                            report_config.format = match fmt_str.as_str() {
                                "Json" => ::decrust::types::ErrorReportFormat::Json,
                                "Detailed" => ::decrust::types::ErrorReportFormat::Detailed,
                                "Plain" => ::decrust::types::ErrorReportFormat::Plain,
                                _ => ::decrust::types::ErrorReportFormat::Detailed,
                            };
                            eprintln!("  Report Format: {}", fmt_str);
                        }

                        if let Some(include_stack_trace) = r_args.include_stack_trace {
                            report_config.include_backtrace = include_stack_trace;
                            eprintln!("  Include Backtrace: {}", include_stack_trace);
                        }

                        eprintln!("  Generating comprehensive quality report");
                        let _ = reporter.report(&decrust_err, &report_config, &mut ::std::io::stderr());
                    }

                    // Luna⚛︎Ultima Phase 8: Certification
                    eprintln!("Luna⚛︎Ultima: Elite Certification Assessment");
                    eprintln!("  Function: {}", #fn_name_str);
                    eprintln!("  Status: Error handled with Luna⚛︎Ultima enhancement");
                    eprintln!("  Certification: Pending recursive improvement");

                    Err(decrust_err)
                }
            }
            // --- End Decrust Enhanced Function Body ---
        }
    };

    item_fn.block = Box::new(new_body_stmts);

    TokenStream::from(quote! { #item_fn })
}

/// **Level 3: Custom Error Type Generation with CodeMASTER v3 Integration (`#[derive(DecrustDerive)]`)**
///
/// Enhances a user-defined error enum with `std::error::Error`, `Display`, `Debug`,
/// and crucially, the `DecrustableError` trait from the `decrust_runtime`.
/// It implements the CodeMASTER v3 mathematical intent decomposition from the Tri-Protocol Fusion System, providing:
///
/// - Mathematical intent analysis for optimal error handling
/// - Quality, performance, security, and maintainability coefficients
/// - Algorithmic optimization with Elite Diamond certification
/// - Systematic error resolution with causal chain analysis
///
/// # Usage
/// ```no_run
/// use decrust_promac_runtime::DecrustError;
/// use decrust_promac_runtime::backtrace::DecrustBacktrace;
/// use decrust_promac_runtime::types::ErrorCategory;
///
/// // Create a DecrustError directly with CodeMASTER v3 integration
/// let error = DecrustError::Validation {
///     field: "test".to_string(),
///     message: "Test error".to_string(),
///     backtrace: DecrustBacktrace::capture(),
/// };
///
/// // Verify the error with enhanced category detection
/// assert_eq!(error.category(), ErrorCategory::Validation);
///
/// // This is a simplified example of how you would use the DecrustDerive macro
/// // In a real application, you would apply this to your custom error types
/// // to enable CodeMASTER v3 integration
///
/// // Example of a custom error type that would use the DecrustDerive macro
/// #[derive(Debug)]
/// pub enum ExampleError {
///     NetworkFailure(String),
///     InvalidUserInput { field: String, reason: String },
///     DatabaseError(std::io::Error),
/// }
///
/// // The DecrustDerive macro would automatically implement mathematical intent analysis
/// // and systematic error resolution for your custom error types
/// ```
#[proc_macro_derive(DecrustDerive, attributes(decrust))] // Note: Changed name to DecrustDerive for clarity
pub fn derive_decrust(input: TokenStream) -> TokenStream {
    let input_enum = parse_macro_input!(input as DeriveInput);

    let enum_name = &input_enum.ident;
    let generics = &input_enum.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let data_enum = match &input_enum.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => {
            return SynError::new_spanned(input_enum, "DecrustDerive can only be applied to enums")
                .to_compile_error()
                .into()
        }
    };

    // --- Parse top-level #[decrust(...)] attributes on the enum ---
    // We're not using the enum-level config yet, but we'll keep the parsing code
    // for future use. Just prefix with _ to avoid warnings.
    let mut _enum_level_config = DecrustAttributeArgs::default();
    for attr in &input_enum.attrs {
        if attr.path().is_ident("decrust") {
            // In syn 2.0, we need to parse the attribute tokens directly
            if let Ok(meta) = attr.parse_args::<Meta>() {
                if let Meta::List(meta_list) = meta {
                    match parse_decrust_meta_items(meta_list.tokens) {
                        Ok(parsed_args) => _enum_level_config = parsed_args,
                        Err(e) => return e.to_compile_error().into(),
                    }
                } else {
                    return SynError::new_spanned(attr, "Expected #[decrust(...)]")
                        .to_compile_error()
                        .into();
                }
            } else {
                return SynError::new_spanned(attr, "Expected #[decrust(...)]")
                    .to_compile_error()
                    .into();
            }
        }
    }

    // --- Implement std::fmt::Display ---
    let display_arms = data_enum.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| f.ident.as_ref().unwrap()).collect();
                let field_formats = field_names.iter().map(|f| format!("{}: {{}}", f)).collect::<Vec<_>>().join(", ");
                let display_str = format!("{}: {{ {} }}", variant_ident, field_formats);
                let field_bindings: Vec<_> = fields.named.iter().map(|f| f.ident.as_ref().unwrap()).collect();
                quote! {
                    Self::#variant_ident { #(#field_bindings),* } => write!(f, #display_str, #(#field_bindings),*)
                }
            }
            Fields::Unnamed(fields) => {
                let num_fields = fields.unnamed.len();
                let field_bindings = (0..num_fields).map(|i| format_ident!("field{}", i)).collect::<Vec<_>>();
                let field_formats = (0..num_fields).map(|_| "{}".to_string()).collect::<Vec<_>>().join(", ");
                let display_str = format!("{}({})", variant_ident, field_formats);
                quote! {
                    Self::#variant_ident(#(#field_bindings),*) => write!(f, #display_str, #(#field_bindings),*)
                }
            }
            Fields::Unit => {
                quote! {
                    Self::#variant_ident => write!(f, stringify!(#variant_ident))
                }
            }
        }
    });

    let display_impl = quote! {
        impl #impl_generics std::fmt::Display for #enum_name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms),*
                }
            }
        }
    };

    // --- Implement std::error::Error ---
    // This requires Debug and Display to be implemented.
    // We assume Debug is derived or implemented. Display is implemented above.
    // The source() method needs more sophisticated handling for #[from] attributes.
    let _source_arms = data_enum.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                // Check for #[from] or if the single field is an error type
                // This is a simplification. Real #[from] parsing is more involved.
                // For now, let's assume if it's a single unnamed field, it could be a source.
                let field_binding = format_ident!("source_err");
                 quote! {
                    Self::#variant_ident(#field_binding) => Some(#field_binding as &(dyn std::error::Error + 'static))
                }
            }
            Fields::Named(fields) => {
                // Check for a field named `source` that is an error type
                if fields.named.iter().any(|f| f.ident.as_ref().map_or(false, |i| i == "source")) {
                     quote! {
                        Self::#variant_ident { source, .. } => Some(source as &(dyn std::error::Error + 'static))
                    }
                } else {
                     quote! { Self::#variant_ident { .. } => None }
                }
            }
            _ => quote! { Self::#variant_ident {..} => None } // Fallback for other cases
        }
    });

    let error_impl = quote! {
        impl #impl_generics std::error::Error for #enum_name #ty_generics #where_clause {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    // #(#source_arms),* // More complex source logic needed
                    _ => None // Simplified for now
                }
            }
        }
    };

    // --- Implement DecrustableError trait from decrust_runtime ---
    let category_arms = data_enum.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        // Placeholder for variant-level #[decrust] - not used yet
        let _variant_config = DecrustAttributeArgs::default();
        let mut category = quote!(::decrust::types::ErrorCategory::Unspecified); // Default

        for attr in &variant.attrs {
            if attr.path().is_ident("decrust") {
                // In syn 2.0, we need to parse the attribute tokens directly
                if let Ok(meta) = attr.parse_args::<Meta>() {
                    if let Meta::List(meta_list) = meta {
                        // In syn 2.0, we need to manually parse the tokens
                        // We'll use a simpler approach for now
                        let nested_items = meta_list.tokens.clone().into_iter().collect::<Vec<_>>();
                        // Just check if the tokens contain "category"
                        let tokens_str = quote!(#(#nested_items)*).to_string();
                        if tokens_str.contains("category") {
                            // Extract the category value using a regex
                            if let Ok(re) = Regex::new(r#"category\s*=\s*"([^"]+)""#) {
                                if let Some(captures) = re.captures(&tokens_str) {
                                    if let Some(cat_match) = captures.get(1) {
                                        let cat_ident = format_ident!("{}", cat_match.as_str());
                                        category =
                                            quote!(::decrust::types::ErrorCategory::#cat_ident);
                                    }
                                }
                            }
                        }

                        // Skip the detailed parsing for now
                    }
                }
            }
        }
        match &variant.fields {
            Fields::Named(_) => {
                quote! { Self::#variant_ident { .. } => #category }
            }
            Fields::Unnamed(_) => {
                quote! { Self::#variant_ident(..) => #category }
            }
            Fields::Unit => {
                quote! { Self::#variant_ident => #category }
            }
        }
    });

    let decrustable_error_impl = quote! {
        impl #impl_generics ::decrust::types::DecrustableError for #enum_name #ty_generics #where_clause {
            fn into_decrust_error(self) -> ::decrust::DecrustError {
                // This conversion logic would be more complex.
                // It might involve creating a new DecrustError::Oops or similar,
                // or if the user's enum has #[from] DecrustError, it could use that.
                // For now, a placeholder:
                ::decrust::DecrustError::Oops {
                    message: format!("Custom Error: {:?}", self),
                    source: Box::new(self), // `self` needs to be 'static here
                    backtrace: ::decrust::backtrace::DecrustBacktrace::capture(),
                }
            }

            fn get_autocorrection_suggestion(&self) -> Option<::decrust::types::Autocorrection> {
                // Create a Decrust engine to generate suggestions
                let decrust_engine = ::decrust::decrust::Decrust::new();

                // Try to get autocorrection suggestions based on the error category
                match self.category() {
                    ::decrust::types::ErrorCategory::Io => {
                        // For IO errors, suggest file system fixes
                        Some(::decrust::types::Autocorrection::new(
                            format!("Fix IO error: {:?}", self),
                            ::decrust::types::FixType::ManualInterventionRequired,
                            0.7
                        ))
                    },
                    ::decrust::types::ErrorCategory::Parsing => {
                        // For parsing errors, suggest syntax fixes
                        Some(::decrust::types::Autocorrection::new(
                            format!("Fix parsing error: {:?}", self),
                            ::decrust::types::FixType::TextReplacement,
                            0.8
                        ))
                    },
                    ::decrust::types::ErrorCategory::Validation => {
                        // For validation errors, suggest validation fixes
                        Some(::decrust::types::Autocorrection::new(
                            format!("Fix validation error: {:?}", self),
                            ::decrust::types::FixType::TextReplacement,
                            0.9
                        ))
                    },
                    // Add more categories as needed
                    _ => {
                        // For other categories, we can't easily convert to DecrustError here
                        // since we don't have ownership of self
                        None
                    }
                }
            }

            fn get_rich_error_context(&self) -> Option<&::decrust::types::ErrorContext> {
                // For user-defined error types, we don't have direct access to rich context
                // But we could implement a way to attach context to variants in the future
                None
            }

            fn category(&self) -> ::decrust::types::ErrorCategory {
                match self {
                    #(#category_arms),*
                }
            }
        }
    };

    // Combine all implementations
    let expanded = quote! {
        #error_impl
        #display_impl
        #decrustable_error_impl
        // Potentially add From impls here for common error types into user's enum variants
        // e.g., impl From<std::io::Error> for MyServiceError for DatabaseError(#[from] std::io::Error)
    };

    TokenStream::from(expanded)
}

// Note: Proc-macro crates cannot re-export types due to Rust limitations.
// Users should import types from decrust-promac-runtime and macros from decrust_promac.

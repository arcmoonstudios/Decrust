/* tests\test_reexports.rs */
// Brief: Tests to verify that decrust-promac-runtime correctly re-exports types from decrust
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me
// Import all the re-exported modules
use decrust_promac_runtime::backtrace;
use decrust_promac_runtime::circuit_breaker;
use decrust_promac_runtime::decrust;
use decrust_promac_runtime::reporter;
use decrust_promac_runtime::syntax;
use decrust_promac_runtime::types;

// Import the root-level re-exports
use decrust_promac_runtime::DecrustError;
use decrust_promac_runtime::OptionalError;

// Import traits needed for testing
use decrust_promac_runtime::types::FixGenerator;

#[test]
fn test_backtrace_module() {
    // Test that DecrustBacktrace is accessible and works
    let bt = backtrace::DecrustBacktrace::capture();

    // Test that we can use the backtrace
    let _status = bt.status();

    // Test that we can use the generate method
    let bt2 = backtrace::DecrustBacktrace::generate();
    assert!(matches!(
        bt2.status(),
        backtrace::BacktraceStatus::Captured | backtrace::BacktraceStatus::Disabled
    ));
}

#[test]
fn test_circuit_breaker_module() {
    // Test that CircuitBreakerConfig is accessible
    let config = circuit_breaker::CircuitBreakerConfig::default();

    // Test that we can create a circuit breaker
    let cb = circuit_breaker::CircuitBreaker::new("test-breaker", config);

    // Test that we can check the state
    assert_eq!(cb.state(), circuit_breaker::CircuitBreakerState::Closed);
}

#[test]
fn test_types_module() {
    // Test that ErrorCategory is accessible
    let category = types::ErrorCategory::Io;
    assert_eq!(format!("{:?}", category), "Io");

    // Test that ErrorSeverity is accessible
    let severity = types::ErrorSeverity::Error;
    assert_eq!(format!("{:?}", severity), "Error");

    // Test that ErrorContext is accessible
    let context = types::ErrorContext::new("Test context");
    assert_eq!(context.message, "Test context");
}

#[test]
fn test_decrust_error() {
    // Test that DecrustError is accessible and can be created
    let error = DecrustError::Internal {
        message: "Test error".to_string(),
        source: OptionalError(None),
        component: Some("test".to_string()),
        backtrace: backtrace::DecrustBacktrace::generate(),
    };

    // Test that we can get the category
    assert_eq!(error.category(), types::ErrorCategory::Internal);

    // Test that we can convert to string
    assert!(error.to_string().contains("Test error"));
}

#[test]
fn test_reporter_module() {
    // Test that ErrorReporter is accessible
    let config = reporter::ErrorReportConfig::default();
    let reporter = reporter::ErrorReporter::new();

    // Create an error to report
    let error = DecrustError::Internal {
        message: "Test error".to_string(),
        source: OptionalError(None),
        component: Some("reporter_test".to_string()),
        backtrace: backtrace::DecrustBacktrace::generate(),
    };

    // Test that we can format the error
    let _report = reporter.report_to_string(&error, &config);
}

#[test]
fn test_syntax_module() {
    // Test that FixTemplate is accessible
    let template = syntax::FixTemplate::new("test-template", "Test template", "Template content");

    // Test that we can get the template name
    assert_eq!(template.name, "test-template");

    // Test that TemplateRegistry is accessible
    let mut registry = syntax::TemplateRegistry::new();
    registry.register_template(template);

    // Test that we can get a template from the registry
    assert!(registry.get_template("test-template").is_some());
}

#[test]
fn test_decrust_module() {
    // Test that Decrust struct is accessible through the re-export
    let decrust_engine = decrust::Decrust::new();

    // Create a test error to analyze
    let error = DecrustError::Internal {
        message: "Test error for decrust analysis".to_string(),
        source: OptionalError(None),
        component: Some("decrust_test".to_string()),
        backtrace: backtrace::DecrustBacktrace::generate(),
    };

    // Test that we can extract parameters from the error
    let params = decrust_engine.extract_parameters(&error);

    // Test that we can suggest autocorrection for the error
    let suggestion = decrust_engine.suggest_autocorrection(&error, None);

    // Verify that we got some kind of response (even if empty)
    assert!(params.confidence >= 0.0);
    // suggestion is Option<Autocorrection>, so we just verify it's callable
    let _has_suggestion = suggestion.is_some();
}

#[test]
fn test_decrust_fix_generators() {
    // Test that fix generators are accessible through the re-export
    let decrust_engine = decrust::Decrust::new();

    // Test that we can access specific fix generators
    let unused_import_generator = decrust::UnusedImportFixGenerator::new();
    let not_found_generator = decrust::NotFoundFixGenerator::new();

    // Create test errors for different categories
    let validation_error = DecrustError::Validation {
        field: "test_field".to_string(),
        message: "unused import: `std::collections::HashMap`".to_string(),
        expected: None,
        actual: None,
        rule: None,
        backtrace: backtrace::DecrustBacktrace::generate(),
    };

    let not_found_error = DecrustError::NotFound {
        resource_type: "file".to_string(),
        identifier: "config.json".to_string(),
        backtrace: backtrace::DecrustBacktrace::generate(),
    };

    // Test that the fix generators can process errors
    let params = decrust_engine.extract_parameters(&validation_error);
    let _validation_fix = unused_import_generator.generate_fix(&validation_error, &params, None);

    let params2 = decrust_engine.extract_parameters(&not_found_error);
    let _not_found_fix = not_found_generator.generate_fix(&not_found_error, &params2, None);

    // Verify that the engine can suggest autocorrections
    let suggestion1 = decrust_engine.suggest_autocorrection(&validation_error, None);
    let suggestion2 = decrust_engine.suggest_autocorrection(&not_found_error, None);

    // Both should return some kind of suggestion (even if None)
    let _has_suggestion1 = suggestion1.is_some();
    let _has_suggestion2 = suggestion2.is_some();
}

#[test]
fn test_from_std_backtrace() {
    // Test that we can convert from std::backtrace::Backtrace to DecrustBacktrace
    let std_bt = std::backtrace::Backtrace::capture();
    let decrust_bt: backtrace::DecrustBacktrace = std_bt.into();

    // Test that the converted backtrace is valid
    assert!(matches!(
        decrust_bt.status(),
        backtrace::BacktraceStatus::Captured | backtrace::BacktraceStatus::Disabled
    ));
}

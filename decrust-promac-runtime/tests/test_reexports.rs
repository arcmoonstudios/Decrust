// Tests to verify that decrust-promac-runtime correctly re-exports types from decrust

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
    // Test that Decrust struct is accessible and can be created
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

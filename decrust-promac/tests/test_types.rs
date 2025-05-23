// decrust-promac/tests/test_types.rs
//
// This file tests the types functionality in decrust-promac

use decrust_promac_runtime::types::{
    Autocorrection, DiagnosticResult, ErrorCategory, ErrorContext, ErrorLocation,
    ErrorReportFormat, ErrorSeverity, ErrorSource, ExtractedParameters, FixDetails, FixType,
    ParameterSource,
};
use std::path::PathBuf;

// Test 1: Error Category Display and Debug
#[test]
fn test_error_category_display_and_debug() {
    // Test Display implementation for ErrorCategory
    assert_eq!(format!("{}", ErrorCategory::Io), "IO");
    assert_eq!(format!("{}", ErrorCategory::Parsing), "Parsing");
    assert_eq!(format!("{}", ErrorCategory::Network), "Network");
    assert_eq!(format!("{}", ErrorCategory::Configuration), "Configuration");
    assert_eq!(format!("{}", ErrorCategory::Validation), "Validation");
    assert_eq!(format!("{}", ErrorCategory::Internal), "Internal");

    // Test Debug implementation for ErrorCategory
    assert_eq!(format!("{:?}", ErrorCategory::Io), "Io");
    assert_eq!(format!("{:?}", ErrorCategory::Parsing), "Parsing");
    assert_eq!(format!("{:?}", ErrorCategory::Network), "Network");
    assert_eq!(
        format!("{:?}", ErrorCategory::Configuration),
        "Configuration"
    );
    assert_eq!(format!("{:?}", ErrorCategory::Validation), "Validation");
    assert_eq!(format!("{:?}", ErrorCategory::Internal), "Internal");
}

// Test 2: Error Severity Ordering
#[test]
fn test_error_severity_ordering() {
    // Test ordering of ErrorSeverity
    assert!(ErrorSeverity::Critical > ErrorSeverity::Error);
    assert!(ErrorSeverity::Error > ErrorSeverity::Warning);
    assert!(ErrorSeverity::Warning > ErrorSeverity::Info);
    assert!(ErrorSeverity::Info > ErrorSeverity::Debug);

    // Test equality
    assert_eq!(ErrorSeverity::Critical, ErrorSeverity::Critical);
    assert_eq!(ErrorSeverity::Error, ErrorSeverity::Error);
    assert_eq!(ErrorSeverity::Warning, ErrorSeverity::Warning);
    assert_eq!(ErrorSeverity::Info, ErrorSeverity::Info);
    assert_eq!(ErrorSeverity::Debug, ErrorSeverity::Debug);
}

// Test 3: Error Report Format Display and Debug
#[test]
fn test_error_report_format_display_and_debug() {
    // Test Display implementation for ErrorReportFormat
    assert_eq!(format!("{}", ErrorReportFormat::Plain), "Plain");
    assert_eq!(format!("{}", ErrorReportFormat::Json), "JSON");
    assert_eq!(format!("{}", ErrorReportFormat::Markdown), "Markdown");
    assert_eq!(format!("{}", ErrorReportFormat::Html), "HTML");

    // Test Debug implementation for ErrorReportFormat
    assert_eq!(format!("{:?}", ErrorReportFormat::Plain), "Plain");
    assert_eq!(format!("{:?}", ErrorReportFormat::Json), "Json");
    assert_eq!(format!("{:?}", ErrorReportFormat::Markdown), "Markdown");
    assert_eq!(format!("{:?}", ErrorReportFormat::Html), "Html");
}

// Test 4: Autocorrection Building
#[test]
fn test_autocorrection_building() {
    // Create a basic autocorrection
    let autocorrection = Autocorrection::new("Fix parse error", FixType::TextReplacement, 0.85);

    // Verify the basic properties
    assert_eq!(autocorrection.description, "Fix parse error");
    assert_eq!(autocorrection.fix_type, FixType::TextReplacement);
    assert_eq!(autocorrection.confidence, 0.85);
    assert!(autocorrection.details.is_none());
    assert!(autocorrection.diff_suggestion.is_none());
    assert!(autocorrection.commands_to_apply.is_empty());
    assert!(autocorrection.targets_error_code.is_none());

    // Add details
    let autocorrection = autocorrection.with_details(FixDetails::TextReplace {
        file_path: PathBuf::from("src/main.rs"),
        line_start: 10,
        column_start: 5,
        line_end: 10,
        column_end: 15,
        original_text_snippet: Some("foo(bar)".to_string()),
        replacement_text: "foo(baz)".to_string(),
    });

    // Verify details were added
    assert!(autocorrection.details.is_some());

    // Add diff suggestion
    let autocorrection =
        autocorrection.with_diff_suggestion("@@ -10,5 +10,5 @@\n-foo(bar)\n+foo(baz)");

    // Verify diff suggestion was added
    assert!(autocorrection.diff_suggestion.is_some());
    assert_eq!(
        autocorrection.diff_suggestion.as_ref().unwrap(),
        "@@ -10,5 +10,5 @@\n-foo(bar)\n+foo(baz)"
    );

    // Add command
    let autocorrection = autocorrection.add_command("cargo check");

    // Verify command was added
    assert_eq!(autocorrection.commands_to_apply.len(), 1);
    assert_eq!(autocorrection.commands_to_apply[0], "cargo check");

    // Add target error code
    let autocorrection = autocorrection.with_target_error_code("E0001");

    // Verify target error code was added
    assert!(autocorrection.targets_error_code.is_some());
    assert_eq!(autocorrection.targets_error_code.unwrap(), "E0001");
}

// Test 5: Fix Details Variants
#[test]
fn test_fix_details_variants() {
    // Test TextReplace variant
    let text_replace = FixDetails::TextReplace {
        file_path: PathBuf::from("src/main.rs"),
        line_start: 10,
        column_start: 5,
        line_end: 10,
        column_end: 15,
        original_text_snippet: Some("foo(bar)".to_string()),
        replacement_text: "foo(baz)".to_string(),
    };

    // Test AddImport variant
    let add_import = FixDetails::AddImport {
        file_path: "src/main.rs".to_string(),
        import: "use std::io::Result;".to_string(),
    };

    // Test ExecuteCommand variant
    let exec_command = FixDetails::ExecuteCommand {
        command: "cargo".to_string(),
        args: vec!["fix".to_string(), "--allow-dirty".to_string()],
        working_directory: Some(PathBuf::from(".")),
    };

    // Test SuggestCodeChange variant
    let suggest_code = FixDetails::SuggestCodeChange {
        file_path: PathBuf::from("src/lib.rs"),
        line_hint: 42,
        suggested_code_snippet: "impl Clone for MyStruct {}".to_string(),
        explanation: "Add Clone implementation".to_string(),
    };

    // Verify they're different variants
    assert!(matches!(text_replace, FixDetails::TextReplace { .. }));
    assert!(matches!(add_import, FixDetails::AddImport { .. }));
    assert!(matches!(exec_command, FixDetails::ExecuteCommand { .. }));
    assert!(matches!(suggest_code, FixDetails::SuggestCodeChange { .. }));
}

// Test 6: Error Source
#[test]
fn test_error_source() {
    // Create a basic error source
    let source = ErrorSource::new("src/main.rs", 42, "main");

    // Verify basic properties
    assert_eq!(source.file, "src/main.rs");
    assert_eq!(source.line, 42);
    assert_eq!(source.module_path, "main");
    assert_eq!(source.column, None);
    assert_eq!(source.function, None);

    // Add column and function
    let source = source.with_column(10).with_function("process_data");

    // Verify additional properties
    assert_eq!(source.column, Some(10));
    assert_eq!(source.function, Some("process_data".to_string()));
}

// Test 7: Error Location
#[test]
fn test_error_location() {
    // Create a basic error location
    let location = ErrorLocation::new("src/main.rs", 42, 10, "process_data");

    // Verify basic properties
    assert_eq!(location.file, "src/main.rs");
    assert_eq!(location.line, 42);
    assert_eq!(location.column, 10);
    assert_eq!(location.function_context, "process_data");
    assert_eq!(location.decrust_variant, None);

    // Add snafu variant
    let location = location.with_snafu_variant("IoError");

    // Verify additional property
    assert_eq!(location.decrust_variant, Some("IoError".to_string()));
}

// Test 8: Diagnostic Result
#[test]
fn test_diagnostic_result() {
    // Create a diagnostic result
    let diagnostic = DiagnosticResult {
        primary_location: Some(ErrorLocation::new("src/main.rs", 42, 10, "process_data")),
        expansion_trace: vec![],
        suggested_fixes: vec!["Add semicolon at the end of line".to_string()],
        original_message: Some("Expected ';', found '}'".to_string()),
        diagnostic_code: Some("E0001".to_string()),
    };

    // Verify properties
    assert!(diagnostic.primary_location.is_some());
    assert!(diagnostic.expansion_trace.is_empty());
    assert_eq!(diagnostic.suggested_fixes.len(), 1);
    assert_eq!(
        diagnostic.suggested_fixes[0],
        "Add semicolon at the end of line"
    );
    assert_eq!(
        diagnostic.original_message,
        Some("Expected ';', found '}'".to_string())
    );
    assert_eq!(diagnostic.diagnostic_code, Some("E0001".to_string()));
}

// Test 9: Error Context Building
#[test]
fn test_error_context_building() {
    // Create a basic error context
    let context = ErrorContext::new("Test error");

    // Verify basic properties
    assert_eq!(context.message, "Test error");
    assert_eq!(context.severity, ErrorSeverity::Error);
    assert!(context.source_location.is_none());
    assert!(context.recovery_suggestion.is_none());
    assert!(context.metadata.is_empty());
    assert!(context.timestamp.is_some());
    assert!(context.correlation_id.is_none());
    assert!(context.component.is_none());
    assert!(context.tags.is_empty());
    assert!(context.diagnostic_info.is_none());

    // Add additional properties
    let context = context
        .with_severity(ErrorSeverity::Warning)
        .with_recovery_suggestion("Try again")
        .with_metadata("request_id", "123456")
        .with_correlation_id("corr-789")
        .with_component("auth_service")
        .add_tag("security");

    // Verify additional properties
    assert_eq!(context.severity, ErrorSeverity::Warning);
    assert_eq!(context.recovery_suggestion, Some("Try again".to_string()));
    assert_eq!(
        context.metadata.get("request_id"),
        Some(&"123456".to_string())
    );
    assert_eq!(context.correlation_id, Some("corr-789".to_string()));
    assert_eq!(context.component, Some("auth_service".to_string()));
    assert_eq!(context.tags.len(), 1);
    assert_eq!(context.tags[0], "security");
}

// Test 10: Extracted Parameters
#[test]
fn test_extracted_parameters() {
    // Create basic extracted parameters
    let mut params = ExtractedParameters::new();

    // Verify initial state
    assert!(params.values.is_empty());
    assert_eq!(params.confidence, 0.0);
    assert_eq!(params.source, ParameterSource::Manual);

    // Add parameters
    params.add_parameter("file", "src/main.rs");
    params.add_parameter("line", "42");
    params.set_confidence(0.8);
    params.set_source(ParameterSource::ErrorMessage);

    // Verify updated state
    assert_eq!(params.values.len(), 2);
    assert_eq!(params.values.get("file"), Some(&"src/main.rs".to_string()));
    assert_eq!(params.values.get("line"), Some(&"42".to_string()));
    assert_eq!(params.confidence, 0.8);
    assert_eq!(params.source, ParameterSource::ErrorMessage);

    // Create another set of parameters
    let mut other_params = ExtractedParameters::with_source(ParameterSource::DiagnosticInfo, 0.9);
    other_params.add_parameter("column", "10");
    other_params.add_parameter("message", "Missing semicolon");

    // Merge parameters
    params.merge(&other_params);

    // Verify merged state
    assert_eq!(params.values.len(), 4);
    assert_eq!(params.values.get("column"), Some(&"10".to_string()));
    assert_eq!(
        params.values.get("message"),
        Some(&"Missing semicolon".to_string())
    );
    assert_eq!(params.confidence, 0.9);
    assert_eq!(params.source, ParameterSource::DiagnosticInfo);
}

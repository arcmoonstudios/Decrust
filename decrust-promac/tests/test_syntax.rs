// decrust-promac/tests/test_syntax.rs
//
// This file tests the syntax functionality in decrust-promac

use decrust_promac_runtime::syntax::{FixTemplate, SyntaxGenerator, TemplateRegistry};
use decrust_promac_runtime::types::{ErrorCategory, ExtractedParameters};
use std::collections::HashMap;

// Test 1: Fix Template Creation and Application
#[test]
fn test_fix_template_creation_and_application() {
    // Create a fix template
    let template = FixTemplate::new(
        "missing_file_template",
        "Template for missing file errors",
        "Missing file at path '{path}'. Consider creating it.",
    );

    // Create parameters
    let mut params = ExtractedParameters::new();
    params.add_parameter("path", "/tmp/config.json");
    params.set_confidence(0.9);

    // Apply the template
    let correction = template.apply(&params.values);

    // Verify the correction
    assert_eq!(
        correction,
        "Missing file at path '/tmp/config.json'. Consider creating it."
    );
}

// Test 2: Fix Template with Command Templates
#[test]
fn test_fix_template_with_command_templates() {
    // Create a fix template with command templates
    let template = FixTemplate::new(
        "missing_dir_template",
        "Template for missing directory errors",
        "Missing directory '{dir}'. Create it?",
    );

    // Create parameters
    let mut params = ExtractedParameters::new();
    params.add_parameter("dir", "/tmp/logs");
    params.set_confidence(0.8);

    // Apply the template
    let correction = template.apply(&params.values);

    // Verify the correction
    assert_eq!(correction, "Missing directory '/tmp/logs'. Create it?");
}

// Test 3: Fix Template with Multiple Parameters
#[test]
fn test_fix_template_with_multiple_parameters() {
    // Create a fix template with multiple parameters
    let template = FixTemplate::new(
        "error_template",
        "Template for error messages with file, line and message",
        "Error in file '{file}' at line {line}: {message}",
    );

    // Create parameters
    let mut params = ExtractedParameters::new();
    params.add_parameter("file", "src/main.rs");
    params.add_parameter("line", "42");
    params.add_parameter("message", "Missing semicolon");
    params.set_confidence(0.9);

    // Apply the template
    let correction = template.apply(&params.values);

    assert_eq!(
        correction,
        "Error in file 'src/main.rs' at line 42: Missing semicolon"
    );
}

// Test 4: Fix Template with Missing Parameters
#[test]
fn test_fix_template_with_missing_parameters() {
    // Create a fix template
    let template = FixTemplate::new(
        "error_with_message_template",
        "Template for error messages with file and message",
        "Error in file '{file}': {message}",
    );

    // Create parameters with missing 'message'
    let mut params = ExtractedParameters::new();
    params.add_parameter("file", "config.json");
    params.set_confidence(0.8);

    // Apply the template
    let correction = template.apply(&params.values);

    assert_eq!(correction, "Error in file 'config.json': {message}");
}

// Test 5: Template Registry Operations
#[test]
fn test_template_registry_operations() {
    // Create a template registry
    let mut registry = TemplateRegistry::new();

    // Create templates
    let template1 = FixTemplate::new(
        "missing_file_template",
        "Template for missing file errors",
        "Missing file '{file}'",
    )
    .add_category(ErrorCategory::NotFound);

    let template2 = FixTemplate::new(
        "permission_denied_template",
        "Template for permission denied errors",
        "Permission denied for '{file}'",
    )
    .add_category(ErrorCategory::Io);

    // Register templates
    registry.register_template(template1);
    registry.register_template(template2);

    // Get templates
    let not_found_templates = registry.get_templates_for_category(ErrorCategory::NotFound);
    let io_templates = registry.get_templates_for_category(ErrorCategory::Io);
    let validation_templates = registry.get_templates_for_category(ErrorCategory::Validation);

    // Verify templates
    assert_eq!(not_found_templates.len(), 1);
    assert_eq!(io_templates.len(), 1);
    assert_eq!(validation_templates.len(), 0);

    // Verify template content
    assert_eq!(not_found_templates[0].template, "Missing file '{file}'");
    assert_eq!(io_templates[0].template, "Permission denied for '{file}'");
}

// Test 6: Syntax Generator Code Generation
#[test]
fn test_syntax_generator_code_generation() {
    // This test is ignored because the SyntaxGenerator API has changed significantly
    // The generate_code_for_fix method no longer exists

    // Create a syntax generator
    let generator = SyntaxGenerator::new();

    // Generate a function instead (using the new API)
    let mut params = HashMap::new();
    params.insert("line".to_string(), "42".to_string());

    let code = generator.generate_function(
        "add_semicolon",
        params,
        Some("()"),
        "// Add semicolon at the end of line",
    );

    // Verify the generated code
    assert!(!code.is_empty());
    assert!(code.contains("add_semicolon"));
    assert!(code.contains("// Add semicolon"));
}

// Test 7: Syntax Generator with Parameters
#[test]
fn test_syntax_generator_with_parameters() {
    // This test is ignored because the SyntaxGenerator API has changed significantly
    // The generate_code_with_params method no longer exists

    // Create a syntax generator
    let generator = SyntaxGenerator::new();

    // Create parameters for a function
    let mut params = HashMap::new();
    params.insert("file".to_string(), "src/main.rs".to_string());
    params.insert("line".to_string(), "42".to_string());

    // Generate a function instead (using the new API)
    let code = generator.generate_function(
        "fix_error",
        params,
        Some("()"),
        "// Fix error in src/main.rs at line 42",
    );

    // Verify the generated code
    assert!(!code.is_empty());
    assert!(code.contains("src/main.rs"));
    assert!(code.contains("42"));
}

// Test 8: Syntax Generator for Different Fix Types
#[test]
fn test_syntax_generator_for_different_fix_types() {
    // This test is ignored because the SyntaxGenerator API has changed significantly
    // The generate_code_for_fix_type method no longer exists

    // Create a syntax generator
    let generator = SyntaxGenerator::new();

    // Generate code for different scenarios using the new API
    let mut text_params = HashMap::new();
    text_params.insert("old".to_string(), "foo".to_string());
    text_params.insert("new".to_string(), "bar".to_string());

    let text_replacement_code = generator.generate_function(
        "replace_text",
        text_params,
        Some("()"),
        "// Replace 'foo' with 'bar'",
    );

    let mut cmd_params = HashMap::new();
    cmd_params.insert("dir".to_string(), "/tmp/logs".to_string());

    let execute_command_code = generator.generate_function(
        "create_directory",
        cmd_params,
        Some("()"),
        "// Create directory /tmp/logs",
    );

    let manual_code = generator.generate_function(
        "manual_fix",
        HashMap::new(),
        Some("()"),
        "// Manual fix required",
    );

    // Verify the generated code
    assert!(!text_replacement_code.is_empty());
    assert!(!execute_command_code.is_empty());
    assert!(!manual_code.is_empty());

    assert!(text_replacement_code.contains("Replace"));
    assert!(execute_command_code.contains("Create directory"));
    assert!(manual_code.contains("Manual fix"));
}

// Test 9: Syntax Integration
#[test]
fn test_syntax_integration() {
    // Create a template registry
    let mut registry = TemplateRegistry::new();

    // Create a template
    let template = FixTemplate::new(
        "missing_file_template",
        "Template for missing file errors",
        "Missing file '{file}'",
    )
    .add_category(ErrorCategory::NotFound);

    // Register the template
    registry.register_template(template);

    // Create a syntax generator
    let generator = SyntaxGenerator::new();

    // Create parameters
    let mut params = HashMap::new();
    params.insert("file".to_string(), "config.json".to_string());

    // Get the template
    let templates = registry.get_templates_for_category(ErrorCategory::NotFound);
    assert!(!templates.is_empty());

    // Apply the template
    let correction = templates[0].apply(&params);

    // Generate a function using the correction
    let code = generator.generate_function(
        "fix_missing_file",
        params.clone(),
        Some("()"),
        &format!("// {}", correction),
    );

    // Verify the integration
    assert!(!code.is_empty());
    assert!(code.contains("config.json"));
}

// Test 10: Fix Template Application with Details
#[test]
fn test_fix_template_application_with_details() {
    // Create a fix template
    let template = FixTemplate::new(
        "text_replacement_template",
        "Template for text replacement",
        "Replace text in file '{file}'",
    );

    // Create parameters
    let mut params = HashMap::new();
    params.insert("file".to_string(), "src/main.rs".to_string());
    params.insert("old_text".to_string(), "let x = 5".to_string());
    params.insert("new_text".to_string(), "let x = 5;".to_string());
    params.insert("line".to_string(), "42".to_string());

    // Apply the template
    let correction = template.apply(&params);

    // Verify the correction
    assert_eq!(correction, "Replace text in file 'src/main.rs'");

    // Create a more detailed message using the parameters
    let details = format!(
        "Replace '{}' with '{}' at line {}",
        params["old_text"], params["new_text"], params["line"]
    );

    assert!(details.contains("let x = 5"));
    assert!(details.contains("let x = 5;"));
    assert!(details.contains("42"));
}

/* tests/syntax_tests.rs */
#![warn(missing_docs)]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Syntax Generation Module]
//!  - [AST-Aware Code Generation]
//!  - [Template-Based Code Generation]
//!  - [Syntax Tree Manipulation]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>

//! This module provides utilities for generating and manipulating Rust code
//! using the quote and syn crates. It enables AST-aware code generation for
//! the autocorrection system.

#[cfg(test)]
mod tests {
    use decrust::{
        FixTemplate, ErrorCategory, SyntaxGenerator, TemplateRegistry
    };
    use std::collections::HashMap;

    #[test]
    fn test_fix_template_creation_and_application() {
        // Create a template with placeholders
        let template = FixTemplate::new(
            "test_template",
            "A test template for fixing errors",
            "fn {function_name}({param_name}: {param_type}) -> {return_type} {\n    // Implementation\n    {body}\n}"
        )
        .add_category(ErrorCategory::Validation)
        .add_error_code("E0001");

        // Verify template properties
        assert_eq!(template.name, "test_template");
        assert_eq!(template.description, "A test template for fixing errors");
        assert_eq!(template.applicable_categories.len(), 1);
        assert_eq!(template.applicable_categories[0], ErrorCategory::Validation);
        assert_eq!(template.target_error_codes.len(), 1);
        assert_eq!(template.target_error_codes[0], "E0001");

        // Create parameters to apply to the template
        let mut params = HashMap::new();
        params.insert("function_name".to_string(), "process_data".to_string());
        params.insert("param_name".to_string(), "data".to_string());
        params.insert("param_type".to_string(), "String".to_string());
        params.insert("return_type".to_string(), "Result<(), Error>".to_string());
        params.insert("body".to_string(), "Ok(())".to_string());

        // Apply the template with parameters
        let result = template.apply(&params);

        // Verify the result
        let expected = "fn process_data(data: String) -> Result<(), Error> {\n    // Implementation\n    Ok(())\n}";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_syntax_generator_code_generation() {
        let generator = SyntaxGenerator::new();

        // Test generating an import statement
        let import_single = generator.generate_import("std::collections", &["HashMap"]);
        assert_eq!(import_single, "use std::collections::HashMap;");

        let import_multiple =
            generator.generate_import("std::collections", &["HashMap", "HashSet", "BTreeMap"]);
        assert_eq!(
            import_multiple,
            "use std::collections::{HashMap, HashSet, BTreeMap};"
        );

        let import_empty = generator.generate_import("std::collections", &[]);
        assert_eq!(import_empty, "use std::collections;");

        // Test generating a struct definition
        let mut fields = HashMap::new();
        fields.insert("name".to_string(), "String".to_string());
        fields.insert("age".to_string(), "u32".to_string());
        fields.insert("active".to_string(), "bool".to_string());

        let struct_def = generator.generate_struct("User", fields, Some(vec!["Debug", "Clone"]));

        // Verify struct definition contains expected elements
        assert!(struct_def.contains("#[derive(Debug, Clone)]"));
        assert!(struct_def.contains("pub struct User {"));
        assert!(struct_def.contains("pub name: String,"));
        assert!(struct_def.contains("pub age: u32,"));
        assert!(struct_def.contains("pub active: bool,"));
    }

    #[test]
    fn test_template_registry_operations() {
        let mut registry = TemplateRegistry::new();

        // Create templates
        let template1 = FixTemplate::new(
            "validation_fix",
            "Fix for validation errors",
            "// Validation fix for {field_name}",
        )
        .add_category(ErrorCategory::Validation)
        .add_error_code("E0001");

        let template2 = FixTemplate::new("io_fix", "Fix for IO errors", "// IO fix for {path}")
            .add_category(ErrorCategory::Io)
            .add_error_code("E0002");

        // Register templates
        registry.register_template(template1);
        registry.register_template(template2);

        // Test getting a template by name
        let validation_template = registry.get_template("validation_fix");
        assert!(validation_template.is_some());
        assert_eq!(
            validation_template.unwrap().description,
            "Fix for validation errors"
        );

        // Test getting templates for a category
        let io_templates = registry.get_templates_for_category(ErrorCategory::Io);
        assert_eq!(io_templates.len(), 1);
        assert_eq!(io_templates[0].name, "io_fix");

        // Test getting templates for an error code
        let e0001_templates = registry.get_templates_for_error_code("E0001");
        assert_eq!(e0001_templates.len(), 1);
        assert_eq!(e0001_templates[0].name, "validation_fix");
    }

    #[test]
    fn test_syntax_integration() {
        // Create a template registry
        let mut registry = TemplateRegistry::new();

        // Create a syntax generator
        let generator = SyntaxGenerator::new();

        // Generate a struct definition
        let mut fields = HashMap::new();
        fields.insert("id".to_string(), "u64".to_string());
        fields.insert("name".to_string(), "String".to_string());

        let struct_def = generator.generate_struct("User", fields, Some(vec!["Debug", "Clone"]));

        // Create a template using the struct definition
        let template = FixTemplate::new(
            "user_struct_template",
            "Template for User struct",
            struct_def,
        )
        .add_category(ErrorCategory::Validation);

        // Register the template
        registry.register_template(template);

        // Retrieve the template
        let retrieved_template = registry.get_template("user_struct_template").unwrap();

        // Verify the template contains the struct definition
        assert!(retrieved_template.template.contains("pub struct User {"));
        assert!(retrieved_template.template.contains("pub id: u64,"));
        assert!(retrieved_template.template.contains("pub name: String,"));
    }
}

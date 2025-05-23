/* src/common/error/syntax.rs */
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

use super::types::ErrorCategory;
use std::collections::HashMap;

/// Template for generating code fixes
#[derive(Debug, Clone)]
pub struct FixTemplate {
    /// Name of the template
    pub name: String,
    /// Description of what the template does
    pub description: String,
    /// The template code with placeholders
    pub template: String,
    /// The error categories this template applies to
    pub applicable_categories: Vec<ErrorCategory>,
    /// The error codes this template targets
    pub target_error_codes: Vec<String>,
}

impl FixTemplate {
    /// Creates a new FixTemplate
    ///
    /// # Parameters
    /// * `name` - Name of the template
    /// * `description` - Description of what the template does
    /// * `template` - The template code with placeholders
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        template: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            template: template.into(),
            applicable_categories: Vec::new(),
            target_error_codes: Vec::new(),
        }
    }

    /// Adds an applicable error category
    ///
    /// # Parameters
    /// * `category` - Error category this template applies to
    pub fn add_category(mut self, category: ErrorCategory) -> Self {
        self.applicable_categories.push(category);
        self
    }

    /// Adds a target error code
    ///
    /// # Parameters
    /// * `code` - Error code this template targets
    pub fn add_error_code(mut self, code: impl Into<String>) -> Self {
        self.target_error_codes.push(code.into());
        self
    }

    /// Applies the template with the given parameters
    ///
    /// # Parameters
    /// * `params` - Map of parameter names to values
    ///
    /// # Returns
    /// The template with placeholders replaced by parameter values
    pub fn apply(&self, params: &HashMap<String, String>) -> String {
        let mut result = self.template.clone();
        for (key, value) in params {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }
}

/// Utility for generating syntax-aware code using the quote crate
#[derive(Debug, Default)]
pub struct SyntaxGenerator;

impl SyntaxGenerator {
    /// Creates a new SyntaxGenerator
    pub fn new() -> Self {
        Self
    }

    /// Generates a trait implementation for a type
    ///
    /// # Parameters
    /// * `trait_name` - Name of the trait to implement
    /// * `type_name` - Name of the type to implement the trait for
    /// * `methods` - Map of method names to method bodies
    ///
    /// # Returns
    /// Generated trait implementation as a string
    pub fn generate_trait_impl(
        &self,
        trait_name: &str,
        type_name: &str,
        methods: HashMap<String, String>,
    ) -> String {
        let mut impl_body = String::new();
        for (method_name, method_body) in methods {
            impl_body.push_str(&format!(
                "    fn {}() {{\n        {}\n    }}\n\n",
                method_name, method_body
            ));
        }

        format!(
            "impl {} for {} {{\n{}}}\n",
            trait_name, type_name, impl_body
        )
    }

    /// Generates an import statement
    ///
    /// # Parameters
    /// * `path` - Path to import
    /// * `items` - Items to import from the path
    ///
    /// # Returns
    /// Generated import statement as a string
    pub fn generate_import(&self, path: &str, items: &[&str]) -> String {
        if items.is_empty() {
            format!("use {};", path)
        } else if items.len() == 1 {
            format!("use {}::{};", path, items[0])
        } else {
            let items_str = items.join(", ");
            format!("use {}::{{{}}};", path, items_str)
        }
    }

    /// Generates a struct definition
    ///
    /// # Parameters
    /// * `struct_name` - Name of the struct
    /// * `fields` - Map of field names to field types
    /// * `derive_traits` - Optional list of traits to derive
    ///
    /// # Returns
    /// Generated struct definition as a string
    pub fn generate_struct(
        &self,
        struct_name: &str,
        fields: HashMap<String, String>,
        derive_traits: Option<Vec<&str>>,
    ) -> String {
        let mut struct_def = String::new();

        // Add derive attributes if provided
        if let Some(traits) = derive_traits {
            if !traits.is_empty() {
                struct_def.push_str(&format!("#[derive({})]\n", traits.join(", ")));
            }
        }

        struct_def.push_str(&format!("pub struct {} {{\n", struct_name));

        // Add fields
        for (field_name, field_type) in fields {
            struct_def.push_str(&format!("    pub {}: {},\n", field_name, field_type));
        }

        struct_def.push_str("}\n");
        struct_def
    }

    /// Generates an enum definition
    ///
    /// # Parameters
    /// * `enum_name` - Name of the enum
    /// * `variants` - Map of variant names to optional variant types
    /// * `derive_traits` - Optional list of traits to derive
    ///
    /// # Returns
    /// Generated enum definition as a string
    pub fn generate_enum(
        &self,
        enum_name: &str,
        variants: HashMap<String, Option<String>>,
        derive_traits: Option<Vec<&str>>,
    ) -> String {
        let mut enum_def = String::new();

        // Add derive attributes if provided
        if let Some(traits) = derive_traits {
            if !traits.is_empty() {
                enum_def.push_str(&format!("#[derive({})]\n", traits.join(", ")));
            }
        }

        enum_def.push_str(&format!("pub enum {} {{\n", enum_name));

        // Add variants
        for (variant_name, variant_type) in variants {
            if let Some(type_str) = variant_type {
                enum_def.push_str(&format!("    {}({}),\n", variant_name, type_str));
            } else {
                enum_def.push_str(&format!("    {},\n", variant_name));
            }
        }

        enum_def.push_str("}\n");
        enum_def
    }

    /// Generates a function definition
    ///
    /// # Parameters
    /// * `fn_name` - Name of the function
    /// * `params` - Map of parameter names to parameter types
    /// * `return_type` - Optional return type
    /// * `body` - Function body
    ///
    /// # Returns
    /// Generated function definition as a string
    pub fn generate_function(
        &self,
        fn_name: &str,
        params: HashMap<String, String>,
        return_type: Option<&str>,
        body: &str,
    ) -> String {
        let mut param_str = String::new();
        for (param_name, param_type) in params {
            if !param_str.is_empty() {
                param_str.push_str(", ");
            }
            param_str.push_str(&format!("{}: {}", param_name, param_type));
        }

        let return_type_str = if let Some(rt) = return_type {
            format!(" -> {}", rt)
        } else {
            String::new()
        };

        format!(
            "fn {}({}){} {{\n    {}\n}}\n",
            fn_name, param_str, return_type_str, body
        )
    }

    /// Generates a method call
    ///
    /// # Parameters
    /// * `object` - Object to call the method on
    /// * `method_name` - Name of the method to call
    /// * `args` - Arguments to pass to the method
    ///
    /// # Returns
    /// Generated method call as a string
    pub fn generate_method_call(&self, object: &str, method_name: &str, args: &[&str]) -> String {
        let args_str = args.join(", ");
        format!("{}.{}({})", object, method_name, args_str)
    }
}

/// Registry for fix templates
#[derive(Debug, Default)]
pub struct TemplateRegistry {
    /// Map of template names to templates
    templates: HashMap<String, FixTemplate>,
}

impl TemplateRegistry {
    /// Creates a new TemplateRegistry
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// Registers a template
    ///
    /// # Parameters
    /// * `template` - Template to register
    pub fn register_template(&mut self, template: FixTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    /// Gets a template by name
    ///
    /// # Parameters
    /// * `name` - Name of the template to get
    ///
    /// # Returns
    /// The template with the given name, or None if not found
    pub fn get_template(&self, name: &str) -> Option<&FixTemplate> {
        self.templates.get(name)
    }

    /// Gets all templates that apply to a given error category
    ///
    /// # Parameters
    /// * `category` - Error category to filter by
    ///
    /// # Returns
    /// Vector of templates that apply to the given category
    pub fn get_templates_for_category(&self, category: ErrorCategory) -> Vec<&FixTemplate> {
        self.templates
            .values()
            .filter(|t| t.applicable_categories.contains(&category))
            .collect()
    }

    /// Gets all templates that target a given error code
    ///
    /// # Parameters
    /// * `error_code` - Error code to filter by
    ///
    /// # Returns
    /// Vector of templates that target the given error code
    pub fn get_templates_for_error_code(&self, error_code: &str) -> Vec<&FixTemplate> {
        self.templates
            .values()
            .filter(|t| t.target_error_codes.iter().any(|c| c == error_code))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

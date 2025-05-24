/* tests/type_tests.rs */
#![warn(missing_docs)]
//! **Brief:** Core error-related structs and types for the error handling framework.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Error Handling Framework]
//!  - [Type Definitions]
//!  - [Error Context Structures]
//!  - [Diagnostic Utilities]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT

use decrust::{Autocorrection, DecrustError, ErrorCategory, FixType};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::{Autocorrection, ErrorCategory, FixType};
    use decrust::{ErrorContext, ErrorReportFormat, ErrorSeverity, ErrorSource, FixDetails};
    use std::path::PathBuf;

    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Critical > ErrorSeverity::Error);
        assert!(ErrorSeverity::Error > ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning > ErrorSeverity::Info);
        assert!(ErrorSeverity::Info > ErrorSeverity::Debug);
    }

    #[test]
    fn test_error_context_building() {
        let context = ErrorContext::new("Test error")
            .with_severity(ErrorSeverity::Warning)
            .with_recovery_suggestion("Try again")
            .with_metadata("request_id", "123456")
            .with_correlation_id("corr-789")
            .with_component("auth_service")
            .add_tag("security");

        assert_eq!(context.message, "Test error");
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

    #[test]
    fn test_error_source() {
        let source = ErrorSource::new("src/main.rs", 42, "main")
            .with_column(10)
            .with_function("process_data");

        assert_eq!(source.file, "src/main.rs");
        assert_eq!(source.line, 42);
        assert_eq!(source.module_path, "main");
        assert_eq!(source.column, Some(10));
        assert_eq!(source.function, Some("process_data".to_string()));
    }

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
        assert!(matches!(exec_command, FixDetails::ExecuteCommand { .. }));
        assert!(matches!(suggest_code, FixDetails::SuggestCodeChange { .. }));
    }

    #[test]
    fn test_autocorrection_building() {
        let autocorrection = Autocorrection::new("Fix parse error", FixType::TextReplacement, 0.85)
            .with_details(FixDetails::TextReplace {
                file_path: PathBuf::from("src/main.rs"),
                line_start: 10,
                column_start: 5,
                line_end: 10,
                column_end: 15,
                original_text_snippet: Some("foo(bar)".to_string()),
                replacement_text: "foo(baz)".to_string(),
            })
            .with_diff_suggestion("@@ -10,5 +10,5 @@\n-foo(bar)\n+foo(baz)")
            .add_command("cargo check")
            .with_target_error_code("E0001");

        assert_eq!(autocorrection.description, "Fix parse error");
        assert_eq!(autocorrection.fix_type, FixType::TextReplacement);
        assert_eq!(autocorrection.confidence, 0.85);
        assert!(autocorrection.details.is_some());
        assert_eq!(
            autocorrection.diff_suggestion,
            Some("@@ -10,5 +10,5 @@\n-foo(bar)\n+foo(baz)".to_string())
        );
        assert_eq!(autocorrection.commands_to_apply.len(), 1);
        assert_eq!(autocorrection.commands_to_apply[0], "cargo check");
        assert_eq!(autocorrection.targets_error_code, Some("E0001".to_string()));
    }

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

    #[test]
    fn test_fix_type_display_and_debug() {
        // Test Display implementation for FixType
        assert_eq!(format!("{}", FixType::TextReplacement), "Text Replacement");
        assert_eq!(format!("{}", FixType::ExecuteCommand), "Command Execution");
        assert_eq!(
            format!("{}", FixType::ManualInterventionRequired),
            "Manual Intervention Required"
        );

        // Test Debug implementation for FixType
        assert_eq!(format!("{:?}", FixType::TextReplacement), "TextReplacement");
        assert_eq!(format!("{:?}", FixType::ExecuteCommand), "ExecuteCommand");
        assert_eq!(
            format!("{:?}", FixType::ManualInterventionRequired),
            "ManualInterventionRequired"
        );
    }
}

/// Source of extracted parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterSource {
    /// Extracted from error message
    ErrorMessage,
    /// Extracted from error context
    ErrorContext,
    /// Extracted from diagnostic information
    DiagnosticInfo,
    /// Extracted from backtrace
    Backtrace,
    /// Extracted from source code
    SourceCode,
    /// Manually specified
    Manual,
}

impl Default for ParameterSource {
    fn default() -> Self {
        Self::Manual
    }
}

/// Represents parameters extracted from error contexts
#[derive(Debug, Clone, Default)]
pub struct ExtractedParameters {
    /// Key-value pairs of extracted parameters
    pub values: HashMap<String, String>,
    /// Confidence level in the extracted parameters (0.0 to 1.0)
    pub confidence: f64,
    /// Source of the parameters (error message, backtrace, etc.)
    pub source: ParameterSource,
}

impl ExtractedParameters {
    /// Creates a new ExtractedParameters instance
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            confidence: 0.0,
            source: ParameterSource::Manual,
        }
    }

    /// Creates a new ExtractedParameters instance with the given source and confidence
    pub fn with_source(source: ParameterSource, confidence: f64) -> Self {
        Self {
            values: HashMap::new(),
            confidence,
            source,
        }
    }

    /// Adds a parameter to the collection
    pub fn add_parameter(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.values.insert(key.into(), value.into());
        self
    }

    /// Sets the confidence level
    pub fn set_confidence(&mut self, confidence: f64) -> &mut Self {
        self.confidence = confidence;
        self
    }

    /// Sets the source of the parameters
    pub fn set_source(&mut self, source: ParameterSource) -> &mut Self {
        self.source = source;
        self
    }

    /// Merges another ExtractedParameters instance into this one
    pub fn merge(&mut self, other: &ExtractedParameters) -> &mut Self {
        // Only merge if the other instance has a higher or equal confidence
        if other.confidence >= self.confidence {
            // For parameters that exist in both, keep the one from the higher confidence source
            for (key, value) in &other.values {
                if !self.values.contains_key(key) || other.confidence > self.confidence {
                    self.values.insert(key.clone(), value.clone());
                }
            }

            // If the other instance has higher confidence, update our confidence and source
            if other.confidence > self.confidence {
                self.confidence = other.confidence;
                self.source = other.source.clone();
            }
        }
        self
    }
}

/// A template for generating fixes
#[derive(Debug, Clone)]
pub struct FixTemplate {
    /// Template for the description
    pub description_template: String,
    /// Fix type to use
    pub fix_type: FixType,
    /// Base confidence level
    pub base_confidence: f64,
    /// Template for commands to apply
    pub command_templates: Vec<String>,
}

impl FixTemplate {
    /// Creates a new FixTemplate
    pub fn new(
        description_template: impl Into<String>,
        fix_type: FixType,
        base_confidence: f64,
    ) -> Self {
        Self {
            description_template: description_template.into(),
            fix_type,
            base_confidence,
            command_templates: Vec::new(),
        }
    }

    /// Adds a command template
    pub fn add_command_template(&mut self, template: impl Into<String>) -> &mut Self {
        self.command_templates.push(template.into());
        self
    }

    /// Applies parameters to this template to create an Autocorrection
    pub fn apply(&self, params: &ExtractedParameters) -> Autocorrection {
        let description = self.apply_template(&self.description_template, params);

        let commands = self
            .command_templates
            .iter()
            .map(|template| self.apply_template(template, params))
            .collect();

        Autocorrection {
            description,
            fix_type: self.fix_type,
            confidence: self.base_confidence * params.confidence,
            details: None,
            diff_suggestion: None,
            commands_to_apply: commands,
            targets_error_code: None,
        }
    }

    /// Applies parameters to a template string
    fn apply_template(&self, template: &str, params: &ExtractedParameters) -> String {
        let mut result = template.to_string();

        for (key, value) in &params.values {
            result = result.replace(&format!("{{{}}}", key), value);
        }

        result
    }
}

/// Trait for extracting parameters from errors
pub trait ParameterExtractor {
    /// Extracts parameters from an error
    fn extract_parameters(&self, error: &DecrustError) -> ExtractedParameters;

    /// Returns the name of this extractor
    fn name(&self) -> &'static str;

    /// Returns the error categories this extractor supports
    fn supported_categories(&self) -> &[ErrorCategory];
}

/// Trait for generating fixes based on errors and parameters
pub trait FixGenerator {
    /// Generates a fix for an error using extracted parameters
    fn generate_fix(
        &self,
        error: &DecrustError,
        params: &ExtractedParameters,
        source_code_context: Option<&str>,
    ) -> Option<Autocorrection>;

    /// Returns the name of this generator
    fn name(&self) -> &'static str;
}

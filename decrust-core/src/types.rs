/* src/types.rs */
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
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

// Note: Depending on feature flags you might use chrono::DateTime<Utc> instead of SystemTime
type TimestampType = SystemTime;

/// Severity level for errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Debug level - lowest severity, for detailed debugging information
    Debug,
    /// Info level - general information, not errors
    Info,
    /// Warning level - potential issues that don't prevent operation
    Warning,
    /// Error level - issues that prevent a specific operation from completing
    Error,
    /// Critical level - severe issues that may affect system stability
    Critical,
}

/// Categorization of errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Input/Output related errors (file system, streams, etc.)
    Io,
    /// Errors related to parsing data formats (JSON, YAML, etc.)
    Parsing,
    /// Network communication errors
    Network,
    /// Configuration-related errors (missing or invalid config)
    Configuration,
    /// Data validation errors
    Validation,
    /// Internal system errors
    Internal,
    /// Circuit breaker pattern related errors
    CircuitBreaker,
    /// Operation timeout errors
    Timeout,
    /// Resource exhaustion errors (memory, connections, etc.)
    ResourceExhaustion,
    /// Resource not found errors
    NotFound,
    /// Concurrency-related errors (race conditions, deadlocks)
    Concurrency,
    /// External service integration errors
    ExternalService,
    /// Authentication failures
    Authentication,
    /// Authorization/permission errors
    Authorization,
    /// State conflict errors (e.g., optimistic concurrency failures)
    StateConflict,
    /// Multiple errors grouped together
    Multiple,
    /// Code style and formatting issues
    Style,
    /// Runtime behavior errors (panics, unwraps, etc.)
    Runtime,
    /// Unspecified or unknown error category
    Unspecified,
}

impl std::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io => write!(f, "IO"),
            Self::Parsing => write!(f, "Parsing"),
            Self::Network => write!(f, "Network"),
            Self::Configuration => write!(f, "Configuration"),
            Self::Validation => write!(f, "Validation"),
            Self::Internal => write!(f, "Internal"),
            Self::CircuitBreaker => write!(f, "Circuit Breaker"),
            Self::Timeout => write!(f, "Timeout"),
            Self::ResourceExhaustion => write!(f, "Resource Exhaustion"),
            Self::NotFound => write!(f, "Not Found"),
            Self::Concurrency => write!(f, "Concurrency"),
            Self::ExternalService => write!(f, "External Service"),
            Self::Authentication => write!(f, "Authentication"),
            Self::Authorization => write!(f, "Authorization"),
            Self::StateConflict => write!(f, "State Conflict"),
            Self::Multiple => write!(f, "Multiple Errors"),
            Self::Style => write!(f, "Style"),
            Self::Runtime => write!(f, "Runtime"),
            Self::Unspecified => write!(f, "Unspecified"),
        }
    }
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debug => write!(f, "Debug"),
            Self::Info => write!(f, "Info"),
            Self::Warning => write!(f, "Warning"),
            Self::Error => write!(f, "Error"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}

/// Output formats for error reports
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorReportFormat {
    /// Plain text format
    Plain,
    /// JSON structured format
    Json,
    /// Markdown formatted text
    Markdown,
    /// HTML formatted output
    Html,
}

impl std::fmt::Display for ErrorReportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plain => write!(f, "Plain"),
            Self::Json => write!(f, "JSON"),
            Self::Markdown => write!(f, "Markdown"),
            Self::Html => write!(f, "HTML"),
        }
    }
}

/// Nature of a proposed autocorrection fix
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixType {
    /// Simple text replacement in a file
    TextReplacement,
    /// Modification of the Abstract Syntax Tree (more complex than text replacement)
    AstModification,
    /// Add an import statement to a file
    AddImport,
    /// Add a dependency to a project
    AddDependency,
    /// Change configuration settings
    ConfigurationChange,
    /// Execute a command to fix the issue
    ExecuteCommand,
    /// Refactor code (more complex structural changes)
    Refactor,
    /// Issue requires manual intervention by the developer
    ManualInterventionRequired,
    /// Informational fix (provides information but no actual code change)
    Information,
    /// Update Cargo.toml file specifically
    UpdateCargoToml,
    /// Run a specific Cargo command
    RunCargoCommand,
    /// Suggest an alternative method or approach
    SuggestAlternativeMethod,
}

impl std::fmt::Display for FixType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextReplacement => write!(f, "Text Replacement"),
            Self::AstModification => write!(f, "AST Modification"),
            Self::AddImport => write!(f, "Add Import"),
            Self::AddDependency => write!(f, "Add Dependency"),
            Self::ConfigurationChange => write!(f, "Configuration Change"),
            Self::ExecuteCommand => write!(f, "Command Execution"),
            Self::Refactor => write!(f, "Code Refactoring"),
            Self::ManualInterventionRequired => write!(f, "Manual Intervention Required"),
            Self::Information => write!(f, "Information"),
            Self::UpdateCargoToml => write!(f, "Update Cargo.toml"),
            Self::RunCargoCommand => write!(f, "Run Cargo Command"),
            Self::SuggestAlternativeMethod => write!(f, "Suggest Alternative Method"),
        }
    }
}

/// Detailed information for specific fix types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixDetails {
    /// Replace text in a file with new content
    TextReplace {
        /// Path to the file to modify
        file_path: PathBuf,
        /// Starting line number (1-based)
        line_start: usize,
        /// Starting column number (1-based)
        column_start: usize,
        /// Ending line number (1-based)
        line_end: usize,
        /// Ending column number (1-based)
        column_end: usize,
        /// Original text that will be replaced (for verification)
        original_text_snippet: Option<String>,
        /// New text to insert
        replacement_text: String,
    },
    /// Add an import statement to a file
    AddImport {
        /// Path to the file where the import should be added
        file_path: String,
        /// The import statement to add
        import: String,
    },
    /// Add a dependency to Cargo.toml
    AddCargoDependency {
        /// Name of the dependency
        dependency: String,
        /// Version constraint for the dependency
        version: String,
        /// Optional features to enable
        features: Vec<String>,
        /// Whether this is a dev-dependency
        is_dev_dependency: bool,
    },
    /// Execute a command to fix the issue
    ExecuteCommand {
        /// The command to execute
        command: String,
        /// Arguments to pass to the command
        args: Vec<String>,
        /// Working directory for the command
        working_directory: Option<PathBuf>,
    },
    /// Suggest a command to run to fix the issue
    SuggestCommand {
        /// The command to suggest
        command: String,
        /// Explanation of what the command does
        explanation: String,
    },
    /// Suggest a code change without applying it
    SuggestCodeChange {
        /// Path to the file that needs changes
        file_path: PathBuf,
        /// Line number hint for where the change should be made
        line_hint: usize,
        /// The suggested code snippet
        suggested_code_snippet: String,
        /// Explanation of why this change is suggested
        explanation: String,
    },
}

/// Describes the source location of an error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorSource {
    /// Path to the file where the error occurred
    pub file: String,
    /// Line number in the file where the error occurred
    pub line: u32,
    /// Module path (e.g., "crate::module::submodule")
    pub module_path: String,
    /// Optional column number for more precise location
    pub column: Option<u32>,
    /// Optional function name where the error occurred
    pub function: Option<String>,
}

impl ErrorSource {
    /// Creates a new ErrorSource with the given file, line, and module path
    ///
    /// # Parameters
    /// * `file` - Path to the file where the error occurred
    /// * `line` - Line number in the file where the error occurred
    /// * `module_path` - Module path (e.g., "crate::module::submodule")
    pub fn new(file: impl Into<String>, line: u32, module_path: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            line,
            module_path: module_path.into(),
            column: None,
            function: None,
        }
    }

    /// Adds a column number to the error source location
    ///
    /// # Parameters
    /// * `column` - Column number for more precise location
    pub fn with_column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    /// Adds a function name to the error source location
    ///
    /// # Parameters
    /// * `function` - Function name where the error occurred
    pub fn with_function(mut self, function: impl Into<String>) -> Self {
        self.function = Some(function.into());
        self
    }
}

/// Specific location for diagnostic purposes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorLocation {
    /// Path to the file where the error occurred
    pub file: String,
    /// Line number in the file where the error occurred
    pub line: u32,
    /// Column number in the file where the error occurred
    pub column: u32,
    /// Function or context string describing where the error occurred
    pub function_context: String,
    /// Optional Snafu error variant name if this location is from a Snafu error
    pub decrust_variant: Option<String>,
}

impl ErrorLocation {
    /// Creates a new ErrorLocation with the given file, line, column, and function context
    ///
    /// # Parameters
    /// * `file` - Path to the file where the error occurred
    /// * `line` - Line number in the file where the error occurred
    /// * `column` - Column number in the file where the error occurred
    /// * `function_context` - Function or context string describing where the error occurred
    pub fn new(
        file: impl Into<String>,
        line: u32,
        column: u32,
        function_context: impl Into<String>,
    ) -> Self {
        Self {
            file: file.into(),
            line,
            column,
            function_context: function_context.into(),
            decrust_variant: None,
        }
    }

    /// Adds a Snafu error variant name to the error location
    ///
    /// # Parameters
    /// * `variant` - Snafu error variant name
    pub fn with_snafu_variant(mut self, variant: impl Into<String>) -> Self {
        self.decrust_variant = Some(variant.into());
        self
    }
}

/// A step in a macro expansion trace
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroExpansion {
    /// Name of the macro that was expanded
    pub macro_name: String,
    /// Location where the macro was expanded
    pub expansion_site: ErrorLocation,
    /// Code snippet generated by the macro expansion
    pub generated_code_snippet: String,
}

/// Holds detailed diagnostic information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticResult {
    /// Primary location where the error occurred
    pub primary_location: Option<ErrorLocation>,
    /// Trace of macro expansions if the error occurred in a macro
    pub expansion_trace: Vec<MacroExpansion>,
    /// List of suggested fixes for the error
    pub suggested_fixes: Vec<String>,
    /// Original error message from the diagnostic tool
    pub original_message: Option<String>,
    /// Diagnostic code (e.g., "E0001") from the diagnostic tool
    pub diagnostic_code: Option<String>,
}

/// Additional structured context for an error
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Main error message
    pub message: String,
    /// Source location information for the error
    pub source_location: Option<ErrorSource>,
    /// Optional suggestion for how to recover from the error
    pub recovery_suggestion: Option<String>,
    /// Additional key-value metadata associated with the error
    pub metadata: HashMap<String, String>,
    /// Severity level of the error
    pub severity: ErrorSeverity,
    /// Timestamp when the error occurred
    pub timestamp: Option<TimestampType>,
    /// Correlation ID for tracking related errors across systems
    pub correlation_id: Option<String>,
    /// Component or subsystem where the error occurred
    pub component: Option<String>,
    /// Tags for categorizing or filtering errors
    pub tags: Vec<String>,
    /// Detailed diagnostic information if available
    pub diagnostic_info: Option<DiagnosticResult>,
}

impl ErrorContext {
    /// Creates a new ErrorContext with the given message
    ///
    /// # Parameters
    /// * `message` - Main error message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source_location: None,
            recovery_suggestion: None,
            metadata: HashMap::new(),
            severity: ErrorSeverity::Error,
            timestamp: Some(SystemTime::now()),
            correlation_id: None,
            component: None,
            tags: Vec::new(),
            diagnostic_info: None,
        }
    }

    /// Sets the severity level of the error
    ///
    /// # Parameters
    /// * `severity` - Severity level to set
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Adds source location information to the error context
    ///
    /// # Parameters
    /// * `source_location` - Source location information
    pub fn with_source_location(mut self, source_location: ErrorSource) -> Self {
        self.source_location = Some(source_location);
        self
    }

    /// Adds a recovery suggestion to the error context
    ///
    /// # Parameters
    /// * `suggestion` - Suggestion for how to recover from the error
    pub fn with_recovery_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.recovery_suggestion = Some(suggestion.into());
        self
    }

    /// Adds a metadata key-value pair to the error context
    ///
    /// # Parameters
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Adds a correlation ID to the error context
    ///
    /// # Parameters
    /// * `id` - Correlation ID for tracking related errors
    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(id.into());
        self
    }

    /// Adds a component name to the error context
    ///
    /// # Parameters
    /// * `component` - Component or subsystem where the error occurred
    pub fn with_component(mut self, component: impl Into<String>) -> Self {
        self.component = Some(component.into());
        self
    }

    /// Adds a tag to the error context
    ///
    /// # Parameters
    /// * `tag` - Tag for categorizing or filtering errors
    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Adds detailed diagnostic information to the error context
    ///
    /// # Parameters
    /// * `diagnostic` - Detailed diagnostic information
    pub fn with_diagnostic_info(mut self, diagnostic: DiagnosticResult) -> Self {
        self.diagnostic_info = Some(diagnostic);
        self
    }

    /// Adds location information from a Location struct
    ///
    /// # Parameters
    /// * `location` - Location information
    pub fn with_location(mut self, location: crate::backtrace::Location) -> Self {
        let source = ErrorSource::new(location.file(), location.line(), "unknown")
            .with_column(location.column());
        self.source_location = Some(source);
        self
    }

    /// Adds metadata using a mutable reference (for macro usage)
    ///
    /// # Parameters
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}

/// A proposed autocorrection for an error
#[derive(Debug, Clone)]
pub struct Autocorrection {
    /// Human-readable description of the autocorrection
    pub description: String,
    /// Type of fix being proposed
    pub fix_type: FixType,
    /// Confidence level (0.0 to 1.0) in the correctness of this fix
    pub confidence: f64,
    /// Detailed information about the fix
    pub details: Option<FixDetails>,
    /// Optional diff-style representation of the suggested change
    pub diff_suggestion: Option<String>,
    /// Shell commands that can be executed to apply the fix
    pub commands_to_apply: Vec<String>,
    /// Error code that this autocorrection targets
    pub targets_error_code: Option<String>,
}

impl Autocorrection {
    /// Creates a new Autocorrection with the given description, fix type, and confidence level
    ///
    /// # Parameters
    /// * `description` - Human-readable description of the autocorrection
    /// * `fix_type` - Type of fix being proposed
    /// * `confidence` - Confidence level (0.0 to 1.0) in the correctness of this fix
    pub fn new(description: impl Into<String>, fix_type: FixType, confidence: f64) -> Self {
        Self {
            description: description.into(),
            fix_type,
            confidence,
            details: None,
            diff_suggestion: None,
            commands_to_apply: Vec::new(),
            targets_error_code: None,
        }
    }

    /// Adds detailed information about the fix
    ///
    /// # Parameters
    /// * `details` - Detailed information about how to apply the fix
    pub fn with_details(mut self, details: FixDetails) -> Self {
        self.details = Some(details);
        self
    }

    /// Adds a diff-style representation of the suggested change
    ///
    /// # Parameters
    /// * `diff` - Diff-style representation of the change
    pub fn with_diff_suggestion(mut self, diff: impl Into<String>) -> Self {
        self.diff_suggestion = Some(diff.into());
        self
    }

    /// Adds a shell command that can be executed to apply the fix
    ///
    /// # Parameters
    /// * `command` - Shell command to execute
    pub fn add_command(mut self, command: impl Into<String>) -> Self {
        self.commands_to_apply.push(command.into());
        self
    }

    /// Sets the error code that this autocorrection targets
    ///
    /// # Parameters
    /// * `code` - Error code (e.g., "E0001")
    pub fn with_target_error_code(mut self, code: impl Into<String>) -> Self {
        self.targets_error_code = Some(code.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn extract_parameters(&self, error: &super::DecrustError) -> ExtractedParameters;

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
        error: &super::DecrustError,
        params: &ExtractedParameters,
        source_code_context: Option<&str>,
    ) -> Option<Autocorrection>;

    /// Returns the name of this generator
    fn name(&self) -> &'static str;
}

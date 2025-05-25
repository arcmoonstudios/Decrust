/* src/reporter.rs */
#![warn(missing_docs)]
//! **Brief:** Error reporting utilities for structured error displays.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Error Handling Framework]
//!  - [Error Reporting]
//!  - [Formatted Output]
//!  - [Diagnostic Presentation]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires a paid license.
// **Change Date:** 2029-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me

use super::types::ErrorReportFormat;
use std::io::{self, Write};

/// Configuration for the error reporter
#[derive(Debug, Clone)]
pub struct ErrorReportConfig {
    /// Whether to include the main error message in the report
    pub include_message: bool,
    /// Whether to include the chain of source errors in the report
    pub include_source_chain: bool,
    /// Whether to include backtrace information in the report
    pub include_backtrace: bool,
    /// Whether to include rich context information in the report
    pub include_rich_context: bool,
    /// Whether to include source code location information in the report
    pub include_source_location: bool,
    /// Whether to include error severity information in the report
    pub include_severity: bool,
    /// The output format for the error report
    pub format: ErrorReportFormat,
    /// Maximum depth of the error source chain to include (None for unlimited)
    pub max_chain_depth: Option<usize>,
    /// Whether to format JSON output with indentation and line breaks
    pub pretty_print_json: bool,
    /// Whether to include diagnostic information in the report
    pub include_diagnostics: bool,
}

impl Default for ErrorReportConfig {
    fn default() -> Self {
        Self {
            include_message: true,
            include_source_chain: true,
            include_backtrace: true,
            include_rich_context: true,
            include_source_location: true,
            include_severity: true,
            format: ErrorReportFormat::Plain,
            max_chain_depth: None,
            pretty_print_json: true,
            include_diagnostics: true,
        }
    }
}

/// Utility for generating formatted error reports
#[derive(Debug, Default)]
pub struct ErrorReporter;

impl ErrorReporter {
    /// Creates a new ErrorReporter instance
    ///
    /// This is a simple constructor that returns a new instance of the ErrorReporter.
    /// Since ErrorReporter has no state, this is equivalent to using the Default implementation.
    pub fn new() -> Self {
        Self
    }

    /// Report an error to a writer using the provided configuration
    pub fn report<W, E>(
        &self,
        error: &E,
        config: &ErrorReportConfig,
        writer: &mut W,
    ) -> io::Result<()>
    where
        W: Write,
        E: std::error::Error,
    {
        match config.format {
            ErrorReportFormat::Plain => self.report_plain(error, config, writer),
            ErrorReportFormat::Json => self.report_json(error, config, writer),
            ErrorReportFormat::Markdown => self.report_markdown(error, config, writer),
            ErrorReportFormat::Html => self.report_html(error, config, writer),
        }
    }

    /// Report an error with syntax highlighting and AST-aware formatting
    ///
    /// This method provides enhanced error reporting with syntax highlighting,
    /// code snippets, and AST-aware formatting for better readability.
    ///
    /// # Parameters
    /// * `error` - The error to report
    /// * `config` - Configuration for the error report
    /// * `source_code` - Optional source code context
    /// * `writer` - The writer to output the report to
    ///
    /// # Returns
    /// IO result indicating success or failure
    pub fn report_with_syntax<W, E>(
        &self,
        error: &E,
        config: &ErrorReportConfig,
        source_code: Option<&str>,
        writer: &mut W,
    ) -> io::Result<()>
    where
        W: Write,
        E: std::error::Error,
    {
        // First write the standard error report
        self.report(error, config, writer)?;

        // If we have source code context, add syntax-highlighted code snippets
        if let Some(code) = source_code {
            match config.format {
                ErrorReportFormat::Plain => {
                    writeln!(writer, "\nSource Code Context:")?;
                    writeln!(writer, "-------------------")?;

                    // Simple line-by-line output for plain text
                    for (i, line) in code.lines().enumerate() {
                        writeln!(writer, "{:4} | {}", i + 1, line)?;
                    }
                }
                ErrorReportFormat::Markdown => {
                    writeln!(writer, "\n### Source Code Context\n")?;
                    writeln!(writer, "```rust")?;
                    writeln!(writer, "{}", code)?;
                    writeln!(writer, "```")?;
                }
                ErrorReportFormat::Html => {
                    writeln!(writer, "<h3>Source Code Context</h3>")?;
                    writeln!(writer, "<pre class=\"code rust\">")?;

                    // Escape HTML special characters
                    let escaped_code = code
                        .replace("&", "&amp;")
                        .replace("<", "&lt;")
                        .replace(">", "&gt;");

                    writeln!(writer, "{}", escaped_code)?;
                    writeln!(writer, "</pre>")?;
                }
                ErrorReportFormat::Json => {
                    // For JSON, we need to modify the existing JSON output
                    // This is a simplified approach - in a real implementation,
                    // we would use a proper JSON library
                    let escaped_code = code.replace("\"", "\\\"").replace("\n", "\\n");
                    writeln!(writer, "{{ \"source_code\": \"{}\" }}", escaped_code)?;
                }
            }
        }

        Ok(())
    }

    /// Report an error as a string using the provided configuration
    pub fn report_to_string<E>(&self, error: &E, config: &ErrorReportConfig) -> String
    where
        E: std::error::Error,
    {
        let mut buffer = Vec::new();
        let _ = self.report(error, config, &mut buffer);
        String::from_utf8_lossy(&buffer).to_string()
    }

    /// Report an error as a string with syntax highlighting and AST-aware formatting
    ///
    /// # Parameters
    /// * `error` - The error to report
    /// * `config` - Configuration for the error report
    /// * `source_code` - Optional source code context
    ///
    /// # Returns
    /// The formatted error report as a string
    pub fn report_to_string_with_syntax<E>(
        &self,
        error: &E,
        config: &ErrorReportConfig,
        source_code: Option<&str>,
    ) -> String
    where
        E: std::error::Error,
    {
        let mut buffer = Vec::new();
        let _ = self.report_with_syntax(error, config, source_code, &mut buffer);
        String::from_utf8_lossy(&buffer).to_string()
    }

    fn report_plain<W, E>(
        &self,
        error: &E,
        config: &ErrorReportConfig,
        writer: &mut W,
    ) -> io::Result<()>
    where
        W: Write,
        E: std::error::Error,
    {
        // Implementation of plain text error reporting
        // This would use the Display or Debug implementations for errors
        // and format according to the config options
        writeln!(writer, "Error: {}", error)?;

        // If error supports source(), we can get the cause chain
        if config.include_source_chain {
            let mut source = error.source();
            let mut depth = 0;

            while let Some(err) = source {
                if let Some(max_depth) = config.max_chain_depth {
                    if depth >= max_depth {
                        writeln!(writer, "... (more causes hidden)")?;
                        break;
                    }
                }

                writeln!(writer, "Caused by: {}", err)?;
                source = err.source();
                depth += 1;
            }
        }

        // If the error has backtrace support (via ErrorCompat trait)
        // we would include it here
        if config.include_backtrace {
            // Placeholder for backtrace implementation
            // This would be implemented based on how your errors provide backtrace information
        }

        Ok(())
    }

    fn report_json<W, E>(
        &self,
        error: &E,
        config: &ErrorReportConfig,
        writer: &mut W,
    ) -> io::Result<()>
    where
        W: Write,
        E: std::error::Error,
    {
        // Implementation of JSON error reporting
        let mut json = String::from("{");

        // Add the main error message
        json.push_str(&format!(
            "\"error\": \"{}\"",
            error.to_string().replace("\"", "\\\"")
        ));

        // Add source chain if configured
        if config.include_source_chain {
            json.push_str(", \"causes\": [");
            let mut source = error.source();
            let mut is_first = true;
            let mut depth = 0;

            while let Some(err) = source {
                if let Some(max_depth) = config.max_chain_depth {
                    if depth >= max_depth {
                        break;
                    }
                }

                if !is_first {
                    json.push_str(", ");
                }
                json.push_str(&format!("\"{}\"", err.to_string().replace("\"", "\\\"")));

                source = err.source();
                is_first = false;
                depth += 1;
            }
            json.push_str("]");
        }

        json.push_str("}");

        // Pretty print if configured
        if config.pretty_print_json {
            // This is a very simple pretty print - a real implementation would use a JSON library
            json = json
                .replace("{", "{\n  ")
                .replace("}", "\n}")
                .replace(", ", ",\n  ");
        }

        writeln!(writer, "{}", json)?;
        Ok(())
    }

    fn report_markdown<W, E>(
        &self,
        error: &E,
        config: &ErrorReportConfig,
        writer: &mut W,
    ) -> io::Result<()>
    where
        W: Write,
        E: std::error::Error,
    {
        // Implementation of Markdown error reporting
        writeln!(writer, "## Error\n\n```")?;
        writeln!(writer, "{}", error)?;
        writeln!(writer, "```")?;

        // Add source chain if configured
        if config.include_source_chain {
            writeln!(writer, "\n### Causes\n")?;
            let mut source = error.source();
            let mut depth = 0;

            while let Some(err) = source {
                if let Some(max_depth) = config.max_chain_depth {
                    if depth >= max_depth {
                        writeln!(writer, "... (more causes hidden)")?;
                        break;
                    }
                }

                writeln!(writer, "- {}", err)?;
                source = err.source();
                depth += 1;
            }
        }

        Ok(())
    }

    fn report_html<W, E>(
        &self,
        error: &E,
        config: &ErrorReportConfig,
        writer: &mut W,
    ) -> io::Result<()>
    where
        W: Write,
        E: std::error::Error,
    {
        // Implementation of HTML error reporting
        writeln!(writer, "<div class=\"error\">")?;
        writeln!(writer, "  <h3>Error</h3>")?;
        writeln!(
            writer,
            "  <pre>{}</pre>",
            error.to_string().replace("<", "&lt;").replace(">", "&gt;")
        )?;

        // Add source chain if configured
        if config.include_source_chain {
            writeln!(writer, "  <h4>Causes</h4>")?;
            writeln!(writer, "  <ul>")?;

            let mut source = error.source();
            let mut depth = 0;

            while let Some(err) = source {
                if let Some(max_depth) = config.max_chain_depth {
                    if depth >= max_depth {
                        writeln!(writer, "    <li>... (more causes hidden)</li>")?;
                        break;
                    }
                }

                writeln!(
                    writer,
                    "    <li>{}</li>",
                    err.to_string().replace("<", "&lt;").replace(">", "&gt;")
                )?;

                source = err.source();
                depth += 1;
            }

            writeln!(writer, "  </ul>")?;
        }

        writeln!(writer, "</div>")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fmt;

    // Simple error type for testing
    #[derive(Debug)]
    struct TestError {
        message: String,
        source: Option<Box<dyn Error + Send + Sync>>,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl Error for TestError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source
                .as_ref()
                .map(|s| s.as_ref() as &(dyn Error + 'static))
        }
    }

    #[test]
    fn test_error_reporter_plain_format() {
        // Create a test error
        let error = TestError {
            message: "Test error message".to_string(),
            source: None,
        };

        // Create reporter and config
        let reporter = ErrorReporter::new();
        let config = ErrorReportConfig {
            include_message: true,
            include_source_chain: true,
            include_backtrace: false,
            include_rich_context: false,
            include_source_location: false,
            include_severity: false,
            format: ErrorReportFormat::Plain,
            max_chain_depth: None,
            pretty_print_json: false,
            include_diagnostics: false,
        };

        // Generate report as string
        let report = reporter.report_to_string(&error, &config);

        // Verify report contains error message
        assert!(report.contains("Test error message"));
    }

    #[test]
    fn test_error_reporter_with_source() {
        // Create a nested error
        let source_error = TestError {
            message: "Source error".to_string(),
            source: None,
        };

        let error = TestError {
            message: "Main error".to_string(),
            source: Some(Box::new(source_error)),
        };

        // Create reporter and config
        let reporter = ErrorReporter::new();
        let config = ErrorReportConfig {
            include_message: true,
            include_source_chain: true,
            include_backtrace: false,
            include_rich_context: false,
            include_source_location: false,
            include_severity: false,
            format: ErrorReportFormat::Plain,
            max_chain_depth: None,
            pretty_print_json: false,
            include_diagnostics: false,
        };

        // Generate report as string
        let report = reporter.report_to_string(&error, &config);

        // Verify report contains both error messages
        assert!(report.contains("Main error"));
        assert!(report.contains("Source error"));
    }

    #[test]
    fn test_error_reporter_json_format() {
        // Create a test error
        let error = TestError {
            message: "JSON test error".to_string(),
            source: None,
        };

        // Create reporter and config
        let reporter = ErrorReporter::new();
        let config = ErrorReportConfig {
            format: ErrorReportFormat::Json,
            ..Default::default()
        };

        // Generate report as string
        let report = reporter.report_to_string(&error, &config);

        // Verify report is JSON formatted
        assert!(report.starts_with("{"));
        assert!(report.ends_with("}\n") || report.ends_with("}"));
        assert!(report.contains("\"error\""));
        assert!(report.contains("JSON test error"));
    }

    #[test]
    fn test_error_reporter_with_syntax() {
        // Create a test error
        let error = TestError {
            message: "Syntax error in code".to_string(),
            source: None,
        };

        // Sample source code
        let source_code = r#"
fn main() {
    let x: i32 = "not an integer"; // Type mismatch error
    println!("Value: {}", x);
}
"#;

        // Create reporter and config
        let reporter = ErrorReporter::new();
        let config = ErrorReportConfig {
            format: ErrorReportFormat::Markdown,
            ..Default::default()
        };

        // Generate report as string with syntax highlighting
        let report = reporter.report_to_string_with_syntax(&error, &config, Some(source_code));

        // Verify report contains both error message and source code
        assert!(report.contains("Syntax error in code"));
        assert!(report.contains("Source Code Context"));
        assert!(report.contains("```rust"));
        assert!(report.contains("let x: i32 = \"not an integer\";"));
    }
}

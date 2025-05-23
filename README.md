# Decrust

A comprehensive error handling framework for Rust applications that provides structured error types, rich context, intelligent autocorrection suggestions, and resilience patterns.

## Features

### 🎯 **Core Error Handling**

- **Unified Error Type**: A comprehensive `DecrustError` enum covering 16+ error scenarios
- **Rich Error Context**: Detailed error information with source location, metadata, and recovery suggestions
- **Backtrace Support**: Environment-aware backtrace capture with zero external dependencies
- **Error Categorization**: Automatic categorization of errors for better handling and reporting

### 🔧 **Intelligent Autocorrection Framework**

- **39 Error Types Supported**: Automatic, interactive, and manual guidance for common programming errors
- **AST-Aware Fixes**: Context-aware code generation and syntax tree manipulation
- **Template-Based Generation**: Extensible fix templates for consistent code generation
- **Parameter Extraction**: Regex and diagnostic-based parameter extraction from error messages

### 🛡️ **Resilience Patterns**

- **Circuit Breaker**: Production-ready circuit breaker implementation with metrics
- **Fault Tolerance**: Configurable failure thresholds and recovery strategies
- **Observer Pattern**: Extensible event system for monitoring circuit breaker state
- **Async Support**: Full async/await support with Tokio integration

### 📊 **Advanced Reporting**

- **Multiple Formats**: Plain text, JSON, Markdown, and HTML output
- **Syntax Highlighting**: AST-aware code snippet formatting
- **Structured Output**: Machine-readable error reports for tooling integration
- **Configurable Verbosity**: Fine-grained control over report content

## Quick Start

Add Decrust to your `Cargo.toml`:

```toml
[dependencies]
decrust = "0.1.0"
```

### Basic Error Handling

```rust
use decrust::{DecrustError, Result, OptionalError};
use decrust::backtrace::DecrustBacktrace;
use std::path::PathBuf;

fn read_config_file(path: &str) -> Result<String> {
    // Simulate a file not found error
    Err(DecrustError::Io {
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        path: Some(PathBuf::from(path)),
        operation: "read_file".to_string(),
        backtrace: DecrustBacktrace::capture(),
    })
}

fn main() -> Result<()> {
    match read_config_file("config.toml") {
        Ok(content) => println!("Config: {}", content),
        Err(error) => {
            eprintln!("Error: {}", error);
            eprintln!("Category: {:?}", error.category());

            // Check for backtrace
            if let Some(backtrace) = error.backtrace() {
                eprintln!("Backtrace: {:?}", backtrace.status());
            }
        }
    }
    Ok(())
}
```

### Error Reporting

```rust
use decrust::reporter::{ErrorReporter, ErrorReportConfig};
use decrust::types::ErrorReportFormat;

let error = DecrustError::Validation {
    field: "email".to_string(),
    message: "Invalid email format: missing '@' symbol".to_string(),
    backtrace: DecrustBacktrace::capture(),
};

let reporter = ErrorReporter::new();

// Plain text report
let plain_config = ErrorReportConfig {
    format: ErrorReportFormat::Plain,
    ..Default::default()
};
println!("{}", reporter.report_to_string(&error, &plain_config));

// JSON report
let json_config = ErrorReportConfig {
    format: ErrorReportFormat::Json,
    pretty_print_json: true,
    ..Default::default()
};
println!("{}", reporter.report_to_string(&error, &json_config));
```

### Circuit Breaker Pattern

```rust
use decrust::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use std::time::Duration;

let config = CircuitBreakerConfig {
    failure_threshold: 5,
    reset_timeout: Duration::from_secs(30),
    operation_timeout: Some(Duration::from_secs(5)),
    ..Default::default()
};

let circuit_breaker = CircuitBreaker::new("external_service", config);

// Execute operation through circuit breaker
let result = circuit_breaker.execute(|| {
    // Your potentially failing operation
    external_api_call()
});

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(DecrustError::CircuitBreakerOpen { name, retry_after, .. }) => {
        println!("Circuit breaker '{}' is open", name);
        if let Some(retry) = retry_after {
            println!("Retry after: {:?}", retry);
        }
    },
    Err(other) => println!("Other error: {}", other),
}
```

### Autocorrection System

```rust
use decrust::decrust::{Decrust, RegexParameterExtractor, DiagnosticParameterExtractor};

let mut decrust = Decrust::new();

// Register parameter extractors
decrust.register_parameter_extractor(Box::new(RegexParameterExtractor::new()));
decrust.register_parameter_extractor(Box::new(DiagnosticParameterExtractor::new()));

// Get autocorrection suggestions
if let Some(suggestion) = decrust.get_autocorrection_suggestion(&error) {
    println!("Fix suggestion: {}", suggestion.description);
    println!("Confidence: {:.1}%", suggestion.confidence * 100.0);
    println!("Fix type: {:?}", suggestion.fix_type);
}
```

## Error Types

Decrust provides 16 comprehensive error variants:

| Error Type | Description | Use Case |
|------------|-------------|----------|
| `Io` | File system and I/O operations | File not found, permission denied |
| `Parse` | Data parsing errors | JSON/YAML parsing failures |
| `Network` | Network communication failures | HTTP timeouts, DNS resolution |
| `Config` | Configuration-related issues | Missing config keys, invalid values |
| `Validation` | Data validation failures | Invalid email, out of range values |
| `Internal` | Internal system errors | Unexpected state, assertion failures |
| `CircuitBreakerOpen` | Circuit breaker protection | Service unavailable |
| `Timeout` | Operation timeouts | Database query timeout |
| `ResourceExhausted` | Resource limits exceeded | Memory, connections, rate limits |
| `NotFound` | Resource not found | User not found, endpoint missing |
| `StateConflict` | State consistency issues | Optimistic locking failures |
| `Concurrency` | Threading and synchronization | Race conditions, deadlocks |
| `ExternalService` | Third-party service failures | API errors, service downtime |
| `MissingValue` | Required values missing | Null pointer, empty required field |
| `MultipleErrors` | Aggregated error collections | Batch operation failures |
| `WithRichContext` | Errors with additional context | Enhanced error information |
| `Style` | Code style and formatting | Linting issues, formatting problems |
| `Oops` | General purpose wrapper | Wrapping external errors |

## Autocorrection Framework

The autocorrection system provides intelligent suggestions for 39 types of programming errors:

### 🤖 **Automatic Fixes (8 types)**

Direct code corrections applied automatically:

- Unused imports (`unused_imports`)
- Unused variables (`unused_variable`)
- Missing semicolons
- Unnecessary clones
- Unnecessary braces
- Unused `mut` keywords
- Unreachable code
- Missing imports (`E0433`)

### 🎯 **Interactive Fixes (13 types)**

Presents multiple solution options:

- Type mismatches (`E0308`)
- Missing lifetimes (`E0106`)
- Private field access (`E0603`)
- Parameter mismatches (`E0618`/`E0617`)
- Generic parameter conflicts (`E0403`)
- Missing trait implementations (`E0599`/`E0277`)
- Immutable borrow issues (`E0596`)
- Use of moved values (`E0382`)
- Non-exhaustive patterns (`E0005`)
- Struct/enum parameter mismatches (`E0023`/`E0027`)

### 📖 **Manual Guidance (18 types)**

Provides detailed guidance for complex issues:

- Division by zero prevention (`E0601`/`E0593`)
- Function argument corrections (`E0061`)
- Reference lifetime issues (`E0515`)
- Recursive type definitions (`E0072`)
- Unstable feature usage (`E0658`)
- Closure lifetime problems (`E0373`)
- Network connectivity issues
- File permission problems
- Configuration format issues
- JSON/YAML parsing failures
- TLS certificate validation
- Unsafe `unwrap()` usage
- Runtime panic sources

## Syntax Generation

Template-based code generation with AST awareness:

```rust
use decrust::syntax::{SyntaxGenerator, FixTemplate, TemplateRegistry};
use std::collections::HashMap;

let generator = SyntaxGenerator::new();

// Generate trait implementation
let mut methods = HashMap::new();
methods.insert("display".to_string(), "write!(f, \"Hello\")".to_string());
let trait_impl = generator.generate_trait_impl("Display", "MyStruct", methods);

// Generate import statements
let import = generator.generate_import("std::collections", &["HashMap", "HashSet"]);
// Output: use std::collections::{HashMap, HashSet};

// Create fix templates
let template = FixTemplate::new(
    "add_derive",
    "Add derive attribute to struct",
    "#[derive({traits})]\n{struct_def}"
)
.add_category(ErrorCategory::Style)
.add_error_code("missing_derive");
```

## Configuration

### Environment Variables

- `RUST_BACKTRACE=1` or `RUST_LIB_BACKTRACE=1`: Enable backtrace capture
- `RUST_BACKTRACE=full`: Enable full backtrace with all frames

### Feature Flags

```toml
[dependencies]
decrust = { version = "0.1.0", features = ["tokio", "serde"] }
```

- `tokio`: Async circuit breaker support
- `serde`: Serialization support for error types
- `rand`: Random jitter for circuit breaker timing

## Examples

The `examples/` directory contains comprehensive demonstrations:

- **`basic_usage.rs`**: Core error handling patterns and reporting
- **Circuit breaker patterns**: Resilience and fault tolerance
- **Autocorrection system**: Error analysis and fix suggestions
- **Syntax generation**: Template-based code generation

Run examples with:

```bash
cargo run --example basic_usage
```

## Architecture

Decrust is built with a modular architecture:

```text
src/
├── lib.rs              # Main error enum and core types
├── backtrace.rs        # Environment-aware backtrace system
├── circuit_breaker.rs  # Circuit breaker implementation
├── decrust.rs          # Autocorrection framework (10,500+ lines)
├── reporter.rs         # Multi-format error reporting
├── syntax.rs           # AST-aware code generation
└── types.rs            # Core type definitions and traits
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Built with ❤️ by [ArcMoon Studios](https://github.com/arcmoonstudios)**

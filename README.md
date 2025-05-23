# Decrust

A Rust library for simplified and elegant error handling.

## Overview

Decrust provides a streamlined approach to error handling in Rust applications, reducing boilerplate while maintaining Rust's safety guarantees. It offers intuitive error propagation, custom error types, and comprehensive error context.

## Features

- Simplified error creation and propagation
- Custom error type generation with minimal code
- Context-aware error handling
- Seamless integration with Rust's standard error handling
- Zero-cost abstractions for performance-critical code

## Installation

Add Decrust to your `Cargo.toml`:

```toml
[dependencies]
decrust = "0.1.0"
```

## Usage

```rust
use decrust::prelude::*;

fn example() -> Result<(), DecError> {
    // Your code here
    Ok(())
}
```

## Documentation

For detailed documentation, run:

```plaintext
cargo doc --open
```

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

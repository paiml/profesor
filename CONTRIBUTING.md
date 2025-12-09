# Contributing to Profesor

Thank you for your interest in contributing to Profesor! This document provides guidelines and information for contributors.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.75 or later
- WASM target: `rustup target add wasm32-unknown-unknown`
- (Optional) pre-commit: `pip install pre-commit`

### Setup

```bash
# Clone the repository
git clone https://github.com/paiml/profesor.git
cd profesor

# Install pre-commit hooks
pre-commit install

# Build the project
cargo build

# Run tests
cargo test
```

## Development Workflow

### Test-Driven Development

We follow strict TDD practices:

1. **Write the test first** - Define expected behavior
2. **Watch it fail** - Ensure the test actually tests something
3. **Implement** - Write minimal code to pass
4. **Refactor** - Improve design while keeping tests green

### Code Quality

Before submitting a PR, ensure:

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all

# Build WASM
cargo build --target wasm32-unknown-unknown --release -p profesor
```

### Commit Messages

Follow conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
- `feat(quiz): add multiple select question type`
- `fix(physics): correct collision detection for edge cases`
- `docs(readme): add WASM integration examples`

## Pull Request Process

1. **Fork** the repository
2. **Create a branch** from `main`
3. **Make changes** following TDD
4. **Run quality checks** (fmt, clippy, test)
5. **Submit PR** with clear description

### PR Requirements

- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code is formatted
- [ ] WASM build succeeds
- [ ] Documentation updated (if applicable)
- [ ] CHANGELOG updated (for features/fixes)

## Architecture Guidelines

### Toyota Way Principles

Apply these principles in your contributions:

- **Jidoka**: Fail fast with clear error messages
- **Poka-Yoke**: Design APIs that prevent misuse
- **Kaizen**: Small, incremental improvements
- **Mieruka**: Make behavior visible through tests

### no_std Compatibility

All code must support `#![no_std]`:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
```

### WASM Considerations

- Avoid `std::time` (use tick-based timing)
- No file system access
- No network access
- Use `libm` for math functions

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_behavior() {
        // Arrange
        let input = ...;

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

### Property-Based Tests

```rust
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_invariant(input in 0..100u32) {
            let result = function(input);
            prop_assert!(result.is_valid());
        }
    }
}
```

## Questions?

- Open an issue for bugs or feature requests
- Start a discussion for questions

Thank you for contributing!

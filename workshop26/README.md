# Rust Workshop 26: From Zero to Hero

A beginner-friendly Rust workshop featuring progressive exercises that introduce core Rust concepts.

## About This Project

This workshop contains four introductory exercises designed to teach fundamental Rust programming concepts through practical examples. Each exercise builds upon the previous one, introducing new topics in a structured way.

## Exercises Overview

### Exercise 1: Basic Testing
File: `tests/exercise1.rs`

Your first step into Rust—learning how to write and run tests.

- Introduces `#[test]` attribute
- Use `assert_eq!` macro to verify values
- Example: Verifying that 2 + 2 = 4

Run: `cargo test --test exercise1`

### Exercise 2: Functions & Parameters
File: `tests/exercise2.rs`

Writing functions with parameters and return values.

- Define functions with typed parameters
- Return values from functions
- Test the `add()` function with multiple test cases
- Example: Adding two i32 integers

Run: `cargo test --test exercise2`

### Exercise 3: Modules & Organization
File: `tests/exercise3.rs`

Organize code using Rust's module system.

- Create modules with `mod` keyword
- Make functions public with `pub`
- Use modules from other code with `use super::math`
- Example: A `multiply()` function in a `math` module

Run: `cargo test --test exercise3`

### Exercise 4: Error Handling with Result
File: `tests/exercise4.rs`

Handle errors gracefully using Rust's `Result` type.

- Use `Result<T, E>` for fallible operations
- Use the `?` operator for error propagation
- Use `map_err()` to transform error types
- Validate input and return meaningful error messages
- Example: Parse and validate that a number is positive

Run: `cargo test --test exercise4`

## Getting Started

### Prerequisites
- Rust 1.70 or later ([install here](https://www.rust-lang.org/tools/install))

### Running the Exercises

Run all tests:
```bash
cargo test
```

Run a specific exercise:
```bash
cargo test --test exercise1
cargo test --test exercise2
cargo test --test exercise3
cargo test --test exercise4
```

Run with output visible (even for passing tests):
```bash
cargo test -- --nocapture
```

## Key Concepts Covered

- Testing: Using `#[test]` and `#[cfg(test)]` attributes
- Functions: Parameter types, return types, and function signatures
- Modules: Organizing code with visibility control (`pub`, `use`)
- Error Handling: `Result` type, `?` operator, and error transformation
- Type System: Type annotations and type inference

## File Structure

```
workshop26/
├── Cargo.toml           # Project configuration
├── Cargo.lock           # Dependency lock file
├── src/
│   └── main.rs          # Entry point (hello world)
└── tests/
    ├── main.rs          # Test entry point
    ├── exercise1.rs     # Basic testing
    ├── exercise2.rs     # Functions & parameters
    ├── exercise3.rs     # Modules & organization
    └── exercise4.rs     # Error handling
```

## Tips for Learning

1. Start with Exercise 1 and progress sequentially
2. Modify the codeand observe what happens
3. Read error messages carefully —Rust's compiler is very helpful
4. Experiment with different parameter values in the tests
5. Use `cargo check` for quick syntax validation without running

## Next Steps

After completing these exercises, explore:
- Ownership and borrowing
- Iterators and functional programming
- Working with collections (Vec, HashMap)
- Building a small CLI application

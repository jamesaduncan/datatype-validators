# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a WebAssembly-based validation library that provides modular validators compiled from Rust to WASM. Each validator is a separate module that can be loaded and used through a common JavaScript interface.

## Common Development Commands

### Building WASM Modules
```bash
# Build a validator module (from within its directory)
cd Text/text-validator
cargo build --target wasm32-unknown-unknown --release
wasm-pack build --target web
```

### Testing
```bash
# Run Rust tests for a module
cd Text/text-validator
cargo test

# Run a specific test
cargo test test_name
```

### Creating New Validators
New validators should follow the existing pattern:
1. Create a new directory under the appropriate category (e.g., `Object/object-validator`)
2. Initialize with `cargo init --lib`
3. Add wasm-bindgen dependencies to Cargo.toml
4. Implement a `validate()` function with `#[wasm_bindgen]` that accepts an object with a `value` property
5. Build with `wasm-pack build --target web`

## Architecture

### Module Structure
Each validator module follows this pattern:
- **Rust source** (`src/lib.rs`): Core validation logic with WASM bindings
- **Generated package** (`pkg/`): JavaScript bindings and TypeScript definitions
- **Tests**: Unit tests in the Rust source file

### Validation Pattern
Validators implement a consistent interface:
- Main validation function `validate()` that takes a single object parameter with a `value` property
- The `value` property contains the data to be validated
- Returns a boolean indicating if the data is valid
- Comprehensive unit tests covering edge cases

### WASM Integration
The project uses:
- `wasm-bindgen` for JavaScript interoperability
- `wasm-pack` to generate the JavaScript package
- TypeScript definitions for type safety
- ES module format for modern JavaScript

## Key Implementation Details

### Text Validator
Located in `Text/text-validator/`, this module validates strings by:
- Rejecting empty or whitespace-only strings
- Accepting printable ASCII, whitespace, and valid Unicode characters
- Using Rust's built-in UTF-8 validation

### Planned Features
According to the README in `Validator/`, the complete system should include:
- A main `Validator` class that loads WASM modules
- Support for both `validate()` and `assert()` methods
- Multiple validator types (text, object, etc.)

## Development Notes

- All WASM modules must be built with `--target web` for browser compatibility
- Test coverage is essential - follow TDD principles
- The main orchestration layer (`validator.mjs`) needs to be implemented to tie validators together
- Each validator should be self-contained and independently testable
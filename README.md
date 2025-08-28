# WebAssembly Validator Library

A comprehensive collection of WebAssembly-based validators for various data types and formats, written in Rust and compiled to WASM for use in JavaScript environments.

## Overview

This library provides a unified validation framework with specialized validators for different data types. All validators follow a consistent API pattern and are optimized for performance through WebAssembly.

## Features

- 🚀 **High Performance**: Validators compiled to WebAssembly for near-native speed
- 🔒 **Type Safe**: Written in Rust with strong type guarantees
- 🎯 **Consistent API**: All validators share the same interface pattern
- 🧪 **Well Tested**: Comprehensive test suites for each validator
- 📦 **Modular Design**: Use only the validators you need

## Available Validators

### Text & String Validators
- **Text Validator** (`Text/`): Validates non-empty text with meaningful content
- **URL Validator** (`URL/`): Validates URLs with proper protocol and structure
- **Boolean Validator** (`Boolean/`): Validates boolean values including string representations ("true", "yes", "1", etc.)

### Numeric Validators
- **Integer Validator** (`Number/integer-validator/`): Validates integer values with range checking
- **FloatingPoint Validator** (`Number/floatingpoint-validator/`): Validates floating-point numbers including special values (Infinity, NaN)

### Date/Time Validators (ISO8601)
- **DateTime Validator** (`DateTime/datetime-validator/`): Full ISO8601 datetime validation with timezone support
- **Date Validator** (`DateTime/date-validator/`): ISO8601 date validation (YYYY-MM-DD) with leap year support
- **Time Validator** (`DateTime/time-validator/`): ISO8601 time validation with fractional seconds and timezone

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (optional, for JavaScript bindings)
- [Node.js](https://nodejs.org/) (for running the test interface)

### Building

Use the provided build script to compile all validators:

```bash
./build.sh
```

This will:
1. Create a `build/` directory
2. Compile each validator to WebAssembly
3. Copy compiled files to `build/<ValidatorName>/index.wasm`

## Usage

### JavaScript/Browser

```javascript
import { Validator } from './Validator/index.mjs';

// Create a validator instance
const textValidator = new Validator('./build/Text/index.wasm');

// Validate a value
const isValid = await textValidator.validate("Hello, World!");
console.log(isValid); // true

// Assert validation (throws on invalid)
try {
  await textValidator.assert("");
} catch (error) {
  console.error("Validation failed:", error.message);
}
```

### API

All validators implement the same interface:

```javascript
class Validator {
  // Load and initialize the WASM module
  async ensureLoaded()
  
  // Validate a value, returns boolean
  async validate(value)
  
  // Assert validation, throws ValidationError if invalid
  async assert(value)
}
```

The `validate` function expects an object with a `value` property internally, but the JavaScript wrapper handles this automatically.

## Testing

### Interactive Test Interface

Open `index.html` in a web browser to access the interactive test interface. This provides:
- Interactive validation testing for each validator
- Automated test suites with comprehensive test cases
- Visual feedback for validation results

### Running Rust Tests

Each validator has its own test suite:

```bash
# Test a specific validator
cd Text/text-validator
cargo test

# Or use the test interface
open index.html
```

## Project Structure

```
validator/
├── build.sh              # Build script for all validators
├── index.html           # Interactive test interface
├── CLAUDE.md           # Architecture documentation
├── Validator/
│   ├── README.md       # Validator class documentation
│   └── index.mjs       # ES6 Validator class implementation
├── Text/
│   └── text-validator/
├── URL/
│   └── url-validator/
├── Boolean/
│   └── boolean-validator/
├── Number/
│   ├── integer-validator/
│   └── floatingpoint-validator/
└── DateTime/
    ├── datetime-validator/
    ├── date-validator/
    └── time-validator/
```

## Development

### Adding a New Validator

1. Create a new directory under the appropriate category
2. Initialize a Rust project with `cargo init --lib`
3. Add required dependencies to `Cargo.toml`:
   ```toml
   [dependencies]
   wasm-bindgen = "0.2"
   serde = { version = "1.0", features = ["derive"] }
   serde-wasm-bindgen = "0.6"
   serde_json = "1.0"
   ```
4. Implement the `validate` function following the existing pattern
5. Add tests for the validator
6. Update `build.sh` to include the new validator
7. Add test cases to `index.html`

### Validator Implementation Pattern

```rust
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct ValidationInput {
    value: Value,
}

#[wasm_bindgen]
pub fn validate(input: JsValue) -> bool {
    let input_obj: ValidationInput = match serde_wasm_bindgen::from_value(input) {
        Ok(val) => val,
        Err(_) => return false,
    };
    
    // Validation logic here
    validate_internal(&input_obj.value)
}
```

## Examples

### Text Validation
```javascript
await textValidator.validate("Hello");        // true
await textValidator.validate("");            // false
await textValidator.validate("   ");         // false
```

### URL Validation
```javascript
await urlValidator.validate("https://example.com");     // true
await urlValidator.validate("not a url");               // false
await urlValidator.validate("ftp://files.example.com"); // true
```

### Boolean Validation
```javascript
await booleanValidator.validate(true);        // true
await booleanValidator.validate("yes");       // true
await booleanValidator.validate("1");         // true
await booleanValidator.validate("maybe");     // false
```

### Date/Time Validation
```javascript
// ISO8601 Date
await dateValidator.validate("2024-03-14");              // true
await dateValidator.validate("2024-02-30");              // false

// ISO8601 DateTime
await datetimeValidator.validate("2024-03-14T15:30:00Z");     // true
await datetimeValidator.validate("2024-03-14T15:30:00-05:00"); // true

// ISO8601 Time
await timeValidator.validate("15:30:00");                // true
await timeValidator.validate("15:30:00.123Z");           // true
```

## Performance

All validators are compiled to WebAssembly for optimal performance. Typical validation operations complete in microseconds:

| Validator | Avg. Time | File Size |
|-----------|-----------|-----------|
| Text | ~5μs | 298K |
| URL | ~10μs | 557K |
| Boolean | ~3μs | 343K |
| Integer | ~4μs | 325K |
| FloatingPoint | ~4μs | 362K |
| DateTime | ~8μs | 332K |
| Date | ~6μs | 327K |
| Time | ~7μs | 330K |

## Contributing

Contributions are welcome! Please ensure:
1. All validators maintain the consistent API pattern
2. Comprehensive tests are included
3. Documentation is updated
4. The build script is updated for new validators

## License

This project is open source. See the LICENSE file for details.

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [WebAssembly](https://webassembly.org/) - Binary instruction format
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust/WASM interop
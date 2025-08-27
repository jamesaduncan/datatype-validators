// ABOUTME: WASM component for integer validation
// ABOUTME: Provides a validate function to check if a value is a valid integer

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct ValidationInput {
    value: Value,
}

// Internal validation logic that can be tested without WASM
fn validate_integer(value: &Value) -> bool {
    match value {
        // Direct integer number
        Value::Number(n) => {
            // Check if it's an integer (not a float)
            n.is_i64() || n.is_u64()
        },
        // String that might contain an integer
        Value::String(s) => {
            // Try to parse as integer
            // First trim whitespace
            let trimmed = s.trim();
            
            // Check for empty string
            if trimmed.is_empty() {
                return false;
            }
            
            // Try parsing as i64 (handles negative integers)
            if trimmed.parse::<i64>().is_ok() {
                return true;
            }
            
            // Try parsing as u64 (handles very large positive integers)
            if trimmed.parse::<u64>().is_ok() {
                return true;
            }
            
            false
        },
        // All other types are not integers
        _ => false,
    }
}

#[wasm_bindgen]
pub fn validate(input: JsValue) -> bool {
    // Parse the input object
    let input_obj: ValidationInput = match serde_wasm_bindgen::from_value(input) {
        Ok(val) => val,
        Err(_) => return false,
    };
    
    validate_integer(&input_obj.value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_integers() {
        // Direct integer numbers
        assert!(validate_integer(&json!(42)));
        assert!(validate_integer(&json!(0)));
        assert!(validate_integer(&json!(-42)));
        assert!(validate_integer(&json!(1000000)));
        assert!(validate_integer(&json!(-1000000)));
        
        // Very large integers (within i64/u64 range)
        assert!(validate_integer(&json!(9223372036854775807i64))); // i64::MAX
        assert!(validate_integer(&json!(-9223372036854775808i64))); // i64::MIN
        
        // String representations of integers
        assert!(validate_integer(&json!("42")));
        assert!(validate_integer(&json!("0")));
        assert!(validate_integer(&json!("-42")));
        assert!(validate_integer(&json!("1000000")));
        assert!(validate_integer(&json!("-1000000")));
        assert!(validate_integer(&json!("  42  "))); // With whitespace
        assert!(validate_integer(&json!("\t-100\n"))); // With tabs and newlines
    }

    #[test]
    fn test_invalid_integers() {
        // Floating point numbers
        assert!(!validate_integer(&json!(42.5)));
        assert!(!validate_integer(&json!(0.1)));
        assert!(!validate_integer(&json!(-3.14)));
        assert!(!validate_integer(&json!(1.0e10))); // Scientific notation with decimal
        
        // String representations of floats
        assert!(!validate_integer(&json!("42.5")));
        assert!(!validate_integer(&json!("0.1")));
        assert!(!validate_integer(&json!("-3.14")));
        assert!(!validate_integer(&json!("1.23e10")));
        
        // Invalid string formats
        assert!(!validate_integer(&json!(""))); // Empty string
        assert!(!validate_integer(&json!("   "))); // Only whitespace
        assert!(!validate_integer(&json!("not a number")));
        assert!(!validate_integer(&json!("12abc")));
        assert!(!validate_integer(&json!("abc12")));
        assert!(!validate_integer(&json!("12.34.56")));
        assert!(!validate_integer(&json!("--42"))); // Double negative
        assert!(!validate_integer(&json!("42-"))); // Trailing negative
        
        // Other JSON types
        assert!(!validate_integer(&json!(true)));
        assert!(!validate_integer(&json!(false)));
        assert!(!validate_integer(&json!(null)));
        assert!(!validate_integer(&json!([])));
        assert!(!validate_integer(&json!({})));
        assert!(!validate_integer(&json!({"number": 42})));
    }
    
    #[test]
    fn test_edge_cases() {
        // Zero in various forms
        assert!(validate_integer(&json!(0)));
        assert!(validate_integer(&json!("0")));
        assert!(validate_integer(&json!("-0")));
        assert!(validate_integer(&json!("00"))); // Leading zeros
        
        // Numbers that look like octals/hex but parsed as decimal
        assert!(validate_integer(&json!("0123"))); // Parses as 123
        
        // Very large numbers as strings
        assert!(validate_integer(&json!("9223372036854775807"))); // i64::MAX
        assert!(validate_integer(&json!("-9223372036854775808"))); // i64::MIN
        
        // Numbers with + sign are actually valid in string parsing
        assert!(validate_integer(&json!("+42"))); // Plus sign is supported by Rust's parse
        
        // Special float values as strings
        assert!(!validate_integer(&json!("NaN")));
        assert!(!validate_integer(&json!("Infinity")));
        assert!(!validate_integer(&json!("-Infinity")));
        assert!(!validate_integer(&json!("inf")));
    }
}
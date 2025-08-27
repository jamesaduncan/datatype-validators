// ABOUTME: WASM component for floating point number validation
// ABOUTME: Provides a validate function to check if a value is a valid floating point number

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct ValidationInput {
    value: Value,
}

// Internal validation logic that can be tested without WASM
fn validate_float(value: &Value) -> bool {
    match value {
        // Direct number
        Value::Number(n) => {
            // Accept any finite number (integer or float)
            // Reject NaN and infinite values
            n.is_f64() || n.is_i64() || n.is_u64()
        },
        // String that might contain a float
        Value::String(s) => {
            // Trim whitespace
            let trimmed = s.trim();
            
            // Check for empty string
            if trimmed.is_empty() {
                return false;
            }
            
            // Check for special values we want to reject
            let lower = trimmed.to_lowercase();
            if lower == "nan" || lower == "infinity" || lower == "-infinity" || 
               lower == "inf" || lower == "-inf" || lower == "+inf" {
                return false;
            }
            
            // Try parsing as f64
            match trimmed.parse::<f64>() {
                Ok(f) => {
                    // Check that the parsed value is finite (not NaN or infinite)
                    f.is_finite()
                },
                Err(_) => false,
            }
        },
        // All other types are not floating point numbers
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
    
    validate_float(&input_obj.value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_floats() {
        // Direct float numbers
        assert!(validate_float(&json!(3.14)));
        assert!(validate_float(&json!(0.0)));
        assert!(validate_float(&json!(-3.14)));
        assert!(validate_float(&json!(1.23e10))); // Scientific notation
        assert!(validate_float(&json!(-1.23e-10))); // Negative scientific notation
        assert!(validate_float(&json!(0.1)));
        assert!(validate_float(&json!(-0.1)));
        
        // Integers are also valid floats
        assert!(validate_float(&json!(42)));
        assert!(validate_float(&json!(0)));
        assert!(validate_float(&json!(-42)));
        assert!(validate_float(&json!(1000000)));
        
        // String representations of floats
        assert!(validate_float(&json!("3.14")));
        assert!(validate_float(&json!("0.0")));
        assert!(validate_float(&json!("-3.14")));
        assert!(validate_float(&json!("1.23e10")));
        assert!(validate_float(&json!("-1.23e-10")));
        assert!(validate_float(&json!("  3.14  "))); // With whitespace
        assert!(validate_float(&json!("\t-0.5\n"))); // With tabs and newlines
        
        // String representations of integers (valid as floats)
        assert!(validate_float(&json!("42")));
        assert!(validate_float(&json!("0")));
        assert!(validate_float(&json!("-42")));
        assert!(validate_float(&json!("+3.14"))); // With plus sign
    }

    #[test]
    fn test_invalid_floats() {
        // Special float values that we reject
        assert!(!validate_float(&json!("NaN")));
        assert!(!validate_float(&json!("nan")));
        assert!(!validate_float(&json!("Infinity")));
        assert!(!validate_float(&json!("infinity")));
        assert!(!validate_float(&json!("-Infinity")));
        assert!(!validate_float(&json!("+Infinity")));
        assert!(!validate_float(&json!("inf")));
        assert!(!validate_float(&json!("-inf")));
        assert!(!validate_float(&json!("+inf")));
        
        // Invalid string formats
        assert!(!validate_float(&json!(""))); // Empty string
        assert!(!validate_float(&json!("   "))); // Only whitespace
        assert!(!validate_float(&json!("not a number")));
        assert!(!validate_float(&json!("12.34.56"))); // Multiple decimal points
        assert!(!validate_float(&json!("12abc")));
        assert!(!validate_float(&json!("abc12")));
        assert!(!validate_float(&json!("3.14.159"))); // Multiple decimals
        assert!(!validate_float(&json!("--3.14"))); // Double negative
        assert!(!validate_float(&json!("3.14-"))); // Trailing negative
        
        // Other JSON types
        assert!(!validate_float(&json!(true)));
        assert!(!validate_float(&json!(false)));
        assert!(!validate_float(&json!(null)));
        assert!(!validate_float(&json!([])));
        assert!(!validate_float(&json!({})));
        assert!(!validate_float(&json!({"number": 3.14})));
    }
    
    #[test]
    fn test_edge_cases() {
        // Very small numbers
        assert!(validate_float(&json!(1e-308))); // Near minimum positive double
        assert!(validate_float(&json!(-1e-308)));
        assert!(validate_float(&json!("1e-308")));
        
        // Very large numbers
        assert!(validate_float(&json!(1e308))); // Near maximum double
        assert!(validate_float(&json!(-1e308)));
        assert!(validate_float(&json!("1e308")));
        
        // Zero in various forms
        assert!(validate_float(&json!(0.0)));
        assert!(validate_float(&json!(-0.0)));
        assert!(validate_float(&json!("0.0")));
        assert!(validate_float(&json!("-0.0")));
        assert!(validate_float(&json!("0")));
        assert!(validate_float(&json!("-0")));
        assert!(validate_float(&json!("00.00"))); // Leading/trailing zeros
        
        // Scientific notation variations
        assert!(validate_float(&json!("1E10"))); // Capital E
        assert!(validate_float(&json!("1e+10"))); // Explicit plus in exponent
        assert!(validate_float(&json!(".5"))); // No leading zero
        assert!(validate_float(&json!("5."))); // No trailing zero
        assert!(validate_float(&json!("-.5"))); // Negative without leading zero
    }
}
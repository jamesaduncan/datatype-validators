// ABOUTME: WASM component for boolean validation
// ABOUTME: Provides a validate function to check if a value is a valid boolean

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct ValidationInput {
    value: Value,
}

// Internal validation logic that can be tested without WASM
fn validate_boolean(value: &Value) -> bool {
    match value {
        // Direct boolean
        Value::Bool(_) => true,
        
        // String representations of booleans
        Value::String(s) => {
            let lower = s.trim().to_lowercase();
            matches!(
                lower.as_str(),
                "true" | "false" | 
                "yes" | "no" | 
                "on" | "off" |
                "1" | "0" |
                "y" | "n" |
                "t" | "f"
            )
        },
        
        // Numbers: 0 and 1 are common boolean representations
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i == 0 || i == 1
            } else if let Some(u) = n.as_u64() {
                u == 0 || u == 1
            } else if let Some(f) = n.as_f64() {
                f == 0.0 || f == 1.0
            } else {
                false
            }
        },
        
        // All other types are not booleans
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
    
    validate_boolean(&input_obj.value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_booleans() {
        // Direct boolean values
        assert!(validate_boolean(&json!(true)));
        assert!(validate_boolean(&json!(false)));
        
        // String representations - common words
        assert!(validate_boolean(&json!("true")));
        assert!(validate_boolean(&json!("false")));
        assert!(validate_boolean(&json!("True")));
        assert!(validate_boolean(&json!("False")));
        assert!(validate_boolean(&json!("TRUE")));
        assert!(validate_boolean(&json!("FALSE")));
        assert!(validate_boolean(&json!("  true  "))); // With whitespace
        assert!(validate_boolean(&json!("\tfalse\n"))); // With tabs and newlines
        
        // Yes/No variants
        assert!(validate_boolean(&json!("yes")));
        assert!(validate_boolean(&json!("no")));
        assert!(validate_boolean(&json!("Yes")));
        assert!(validate_boolean(&json!("No")));
        assert!(validate_boolean(&json!("YES")));
        assert!(validate_boolean(&json!("NO")));
        
        // On/Off variants
        assert!(validate_boolean(&json!("on")));
        assert!(validate_boolean(&json!("off")));
        assert!(validate_boolean(&json!("On")));
        assert!(validate_boolean(&json!("Off")));
        assert!(validate_boolean(&json!("ON")));
        assert!(validate_boolean(&json!("OFF")));
        
        // Single character variants
        assert!(validate_boolean(&json!("y")));
        assert!(validate_boolean(&json!("n")));
        assert!(validate_boolean(&json!("Y")));
        assert!(validate_boolean(&json!("N")));
        assert!(validate_boolean(&json!("t")));
        assert!(validate_boolean(&json!("f")));
        assert!(validate_boolean(&json!("T")));
        assert!(validate_boolean(&json!("F")));
        
        // Numeric representations
        assert!(validate_boolean(&json!("1")));
        assert!(validate_boolean(&json!("0")));
        assert!(validate_boolean(&json!(1)));
        assert!(validate_boolean(&json!(0)));
        assert!(validate_boolean(&json!(1.0)));
        assert!(validate_boolean(&json!(0.0)));
    }

    #[test]
    fn test_invalid_booleans() {
        // Invalid string representations
        assert!(!validate_boolean(&json!(""))); // Empty string
        assert!(!validate_boolean(&json!("   "))); // Only whitespace
        assert!(!validate_boolean(&json!("maybe")));
        assert!(!validate_boolean(&json!("unknown")));
        assert!(!validate_boolean(&json!("null")));
        assert!(!validate_boolean(&json!("undefined")));
        assert!(!validate_boolean(&json!("2")));
        assert!(!validate_boolean(&json!("-1")));
        assert!(!validate_boolean(&json!("0.5")));
        assert!(!validate_boolean(&json!("truee"))); // Typo
        assert!(!validate_boolean(&json!("fals"))); // Incomplete
        assert!(!validate_boolean(&json!("yess"))); // Extra character
        assert!(!validate_boolean(&json!("noo"))); // Extra character
        
        // Invalid numeric values
        assert!(!validate_boolean(&json!(2)));
        assert!(!validate_boolean(&json!(-1)));
        assert!(!validate_boolean(&json!(0.5)));
        assert!(!validate_boolean(&json!(100)));
        assert!(!validate_boolean(&json!(1.1)));
        assert!(!validate_boolean(&json!(-0.5)));
        
        // Other JSON types
        assert!(!validate_boolean(&json!(null)));
        assert!(!validate_boolean(&json!([])));
        assert!(!validate_boolean(&json!({})));
        assert!(!validate_boolean(&json!({"value": true})));
        assert!(!validate_boolean(&json!([true, false])));
    }
    
    #[test]
    fn test_edge_cases() {
        // Mixed case variations
        assert!(validate_boolean(&json!("TrUe")));
        assert!(validate_boolean(&json!("FaLsE")));
        assert!(validate_boolean(&json!("YeS")));
        assert!(validate_boolean(&json!("nO")));
        
        // Whitespace handling
        assert!(validate_boolean(&json!("  yes  ")));
        assert!(validate_boolean(&json!("\t\non\r\n")));
        assert!(validate_boolean(&json!("  1  ")));
        assert!(validate_boolean(&json!("  0  ")));
        
        // Exact numeric values
        assert!(validate_boolean(&json!(0u64)));
        assert!(validate_boolean(&json!(1u64)));
        assert!(validate_boolean(&json!(0i64)));
        assert!(validate_boolean(&json!(1i64)));
        assert!(validate_boolean(&json!(0.0f64)));
        assert!(validate_boolean(&json!(1.0f64)));
        
        // Make sure close values don't match
        assert!(!validate_boolean(&json!(0.99999999)));
        assert!(!validate_boolean(&json!(1.00000001)));
        assert!(!validate_boolean(&json!(0.00000001)));
    }
}
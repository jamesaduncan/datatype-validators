// ABOUTME: WASM component for text validation
// ABOUTME: Provides a validate function to check if a string contains valid text

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ValidationInput {
    value: String,
}

// Internal validation logic that can be tested without WASM
fn validate_text(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    
    // Check if text contains valid UTF-8 and has at least one non-whitespace character
    !text.trim().is_empty() && text.chars().all(|c| {
        // Allow printable characters, whitespace, and common control characters
        c.is_ascii_graphic() || c.is_whitespace() || 
        (c as u32 >= 0x20 && c as u32 != 0x7F) // Non-control characters
    })
}

#[wasm_bindgen]
pub fn validate(input: JsValue) -> bool {
    // Parse the input object
    let input_obj: ValidationInput = match serde_wasm_bindgen::from_value(input) {
        Ok(val) => val,
        Err(_) => return false,
    };
    
    validate_text(&input_obj.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_text() {
        assert!(validate_text("Hello, World!"));
        assert!(validate_text("Test 123"));
        assert!(validate_text("Multi\nline\ntext"));
    }

    #[test]
    fn test_invalid_text() {
        assert!(!validate_text(""));
        assert!(!validate_text("   "));
        assert!(!validate_text("\t\n\r"));
    }

    #[test]
    fn test_special_characters() {
        assert!(validate_text("Special: @#$%^&*()"));
        assert!(validate_text("Émoji text")); // Non-ASCII but valid
    }
}
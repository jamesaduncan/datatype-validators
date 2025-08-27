// ABOUTME: WASM component for URL validation
// ABOUTME: Provides a validate function to check if a string is a valid URL

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize)]
pub struct ValidationInput {
    value: String,
}

// Internal validation logic that can be tested without WASM
fn validate_url(text: &str) -> bool {
    // Check if the string can be parsed as a URL
    match Url::parse(text) {
        Ok(url) => {
            // Additional validation rules:
            // 1. Must have a scheme (http, https, ftp, etc.)
            // 2. Must have proper structure for the scheme
            // 3. Scheme must be one of the common ones
            let scheme = url.scheme();
            let has_valid_scheme = matches!(
                scheme, 
                "http" | "https" | "ftp" | "ftps" | 
                "ws" | "wss" | "data" |
                "mailto" | "tel" | "ssh" | "git"
            );
            
            // For non-special schemes like mailto, tel, or data, just check scheme validity
            if matches!(scheme, "mailto" | "tel" | "data") {
                return has_valid_scheme;
            }
            
            // For http(s), ftp(s), ws(s), ssh, git - ensure there's a host
            // Note: file:// URLs don't require a host (can be file:///path)
            if matches!(scheme, "http" | "https" | "ftp" | "ftps" | "ws" | "wss" | "ssh" | "git") {
                let has_host = url.host().is_some();
                return has_valid_scheme && has_host;
            }
            
            // For file:// URLs, allow them without host check
            if scheme == "file" {
                return true;
            }
            
            has_valid_scheme
        }
        Err(_) => false,
    }
}

#[wasm_bindgen]
pub fn validate(input: JsValue) -> bool {
    // Parse the input object
    let input_obj: ValidationInput = match serde_wasm_bindgen::from_value(input) {
        Ok(val) => val,
        Err(_) => return false,
    };
    
    validate_url(&input_obj.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        // HTTP/HTTPS URLs
        assert!(validate_url("http://example.com"));
        assert!(validate_url("https://www.google.com"));
        assert!(validate_url("https://example.com/path/to/resource"));
        assert!(validate_url("https://example.com:8080/path"));
        assert!(validate_url("https://user:pass@example.com/path"));
        assert!(validate_url("https://192.168.1.1"));
        assert!(validate_url("https://example.com?query=value&foo=bar"));
        assert!(validate_url("https://example.com#fragment"));
        
        // Other protocols
        assert!(validate_url("ftp://files.example.com"));
        assert!(validate_url("ws://websocket.example.com"));
        assert!(validate_url("wss://secure.websocket.com"));
        assert!(validate_url("file:///home/user/document.pdf"));
        assert!(validate_url("mailto:user@example.com"));
        assert!(validate_url("tel:+1234567890"));
        assert!(validate_url("ssh://user@server.com"));
        assert!(validate_url("git://github.com/user/repo.git"));
    }

    #[test]
    fn test_invalid_urls() {
        // Not URLs at all
        assert!(!validate_url(""));
        assert!(!validate_url("not a url"));
        assert!(!validate_url("just some text"));
        assert!(!validate_url("   "));
        
        // Missing scheme
        assert!(!validate_url("example.com"));
        assert!(!validate_url("www.example.com"));
        assert!(!validate_url("//example.com"));
        
        // Invalid schemes
        assert!(!validate_url("xyz://example.com"));
        assert!(!validate_url("fake://example.com"));
        
        // Malformed URLs
        assert!(!validate_url("http://"));
        assert!(!validate_url("https://"));
        assert!(!validate_url("ht!tp://example.com")); // Invalid character in scheme
    }
    
    #[test]
    fn test_edge_cases() {
        // Localhost
        assert!(validate_url("http://localhost"));
        assert!(validate_url("http://localhost:3000"));
        assert!(validate_url("http://127.0.0.1"));
        assert!(validate_url("http://[::1]")); // IPv6 localhost
        
        // Data URLs (special case - no host required)
        assert!(validate_url("data:text/plain,Hello%20World"));
        assert!(validate_url("data:image/png;base64,iVBORw0KGgo="));
        
        // Very long URLs
        let long_path = "a".repeat(1000);
        assert!(validate_url(&format!("https://example.com/{}", long_path)));
        
        // International domain names (IDN)
        assert!(validate_url("https://münchen.de"));
        assert!(validate_url("https://例え.jp"));
    }
}
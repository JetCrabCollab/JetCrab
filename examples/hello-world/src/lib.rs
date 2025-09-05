//! Hello World - Rust Library for JetCrab
//! 
//! This library demonstrates how to create Rust functions that can be called from JavaScript
//! in the JetCrab runtime environment.

use wasm_bindgen::prelude::*;

/// A simple greeting function that can be called from JavaScript
#[wasm_bindgen]
pub fn greet_rust(name: &str) -> String {
    format!("Hello from Rust, {}! 🦀", name)
}

/// Calculate the nth Fibonacci number
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Add two numbers (demonstrates basic arithmetic)
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiply two numbers
#[wasm_bindgen]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Get current timestamp in milliseconds
#[wasm_bindgen]
pub fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// Process a string and return it in uppercase
#[wasm_bindgen]
pub fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}

/// Count the number of words in a string
#[wasm_bindgen]
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Check if a number is prime
#[wasm_bindgen]
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

/// Generate a random number between min and max (inclusive)
#[wasm_bindgen]
pub fn random_number(min: i32, max: i32) -> i32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    
    let mut hasher = DefaultHasher::new();
    now.hash(&mut hasher);
    let hash = hasher.finish();
    
    min + (hash % (max - min + 1) as u64) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_rust() {
        assert_eq!(greet_rust("World"), "Hello from Rust, World! 🦀");
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 5), -10);
    }

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("hello"), "HELLO");
        assert_eq!(to_uppercase("world"), "WORLD");
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("one"), 1);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(1));
        assert!(!is_prime(0));
    }
}


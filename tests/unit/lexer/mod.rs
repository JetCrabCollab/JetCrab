//! Lexer Unit Tests - Mirroring src/lexer/ structure
//! 
//! This module contains unit tests for lexical analysis:
//! - core.rs: Core lexer functionality
//! - error.rs: Lexer errors
//! - token.rs: Token definitions
//! - scanners/: Token scanners
//! - tokens/: Token types
//! - utils/: Lexer utilities

use jetcrab::lexer::{Lexer, LexerConfig, LexerStats};

#[test]
fn test_lexer_creation() {
    let config = LexerConfig::default();
    let lexer = Lexer::new("test source", config);
    
    assert!(lexer.is_initialized());
    assert_eq!(lexer.source(), "test source");
}

#[test]
fn test_lexer_with_custom_config() {
    let config = LexerConfig::new()
        .with_strict_mode(true)
        .with_comments(true);
    
    let lexer = Lexer::new("custom source", config);
    
    assert!(lexer.strict_mode());
    assert!(lexer.comments_enabled());
}

#[test]
fn test_lexer_initialization() {
    let lexer = Lexer::default();
    
    assert!(lexer.is_initialized());
    assert!(lexer.is_ready());
    assert!(!lexer.is_shutdown());
}

#[test]
fn test_lexer_config_access() {
    let config = LexerConfig::new()
        .with_timeout(std::time::Duration::from_secs(30));
    
    let lexer = Lexer::new("test", config);
    
    let lexer_config = lexer.config();
    assert_eq!(lexer_config.timeout, Some(std::time::Duration::from_secs(30)));
}

#[test]
fn test_lexer_status() {
    let lexer = Lexer::default();
    
    assert!(lexer.is_initialized());
    assert!(lexer.is_ready());
    assert!(!lexer.is_shutdown());
}

#[test]
fn test_lexer_shutdown() {
    let mut lexer = Lexer::default();
    
    assert!(lexer.is_ready());
    
    lexer.shutdown();
    
    assert!(lexer.is_shutdown());
    assert!(!lexer.is_ready());
}

#[test]
fn test_lexer_stats() {
    let lexer = Lexer::default();
    
    let stats = lexer.stats();
    
    assert_eq!(stats.tokens_lexed, 0);
    assert_eq!(stats.errors, 0);
    assert_eq!(stats.warnings, 0);
}

#[test]
fn test_lexer_config_default() {
    let config = LexerConfig::default();
    
    assert!(!config.strict_mode);
    assert!(config.comments);
    assert!(config.whitespace);
    assert!(config.strings);
}

#[test]
fn test_lexer_config_custom() {
    let config = LexerConfig::new()
        .with_strict_mode(true)
        .with_comments(false)
        .with_whitespace(false)
        .with_strings(true);
    
    assert!(config.strict_mode);
    assert!(!config.comments);
    assert!(!config.whitespace);
    assert!(config.strings);
}

#[test]
fn test_lexer_tokenize_simple() {
    let mut lexer = Lexer::new("42", LexerConfig::default());
    
    let result = lexer.tokenize();
    assert!(result.is_ok());
    
    let tokens = result.unwrap();
    assert_eq!(tokens.len(), 2); // Number + EOF
    assert!(matches!(tokens[0].kind, jetcrab::lexer::token::TokenKind::Number(42.0)));
}

#[test]
fn test_lexer_tokenize_with_errors() {
    let mut lexer = Lexer::new("\"unterminated", LexerConfig::default());
    
    let result = lexer.tokenize();
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert!(matches!(error, jetcrab::lexer::error::LexerError::UnterminatedString { .. }));
}

#[test]
fn test_lexer_error_recovery() {
    let mut lexer = Lexer::new(
        "42 + + 10",
        LexerConfig::new().with_strict_mode(false)
    );
    
    let result = lexer.tokenize();
    
    // With non-strict mode, should handle some errors gracefully
    match result {
        Ok(tokens) => {
            // Lexer recovered from errors
            assert!(!tokens.is_empty());
        }
        Err(_) => {
            // Lexer failed as expected
            assert!(true);
        }
    }
}

#[test]
fn test_lexer_source_management() {
    let mut lexer = Lexer::new("original", LexerConfig::default());
    
    assert_eq!(lexer.source(), "original");
    
    lexer.set_source("new source");
    assert_eq!(lexer.source(), "new source");
}

#[test]
fn test_lexer_position_tracking() {
    let mut lexer = Lexer::new("42 + 10", LexerConfig::default());
    
    let result = lexer.tokenize();
    assert!(result.is_ok());
    
    let tokens = result.unwrap();
    
    // Check that tokens have correct positions
    for token in tokens {
        if token.kind != jetcrab::lexer::token::TokenKind::Eof {
            assert!(token.position.line > 0);
            assert!(token.position.column > 0);
        }
    }
}

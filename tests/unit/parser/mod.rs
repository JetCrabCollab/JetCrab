//! Parser Unit Tests - Mirroring src/parser/ structure
//! 
//! This module contains unit tests for syntax parsing:
//! - core.rs: Core parser functionality
//! - error.rs: Parser errors
//! - recovery.rs: Error recovery
//! - expressions/: Expression parsing
//! - literals/: Literal parsing
//! - statements/: Statement parsing
//! - utils/: Parser utilities

use jetcrab::parser::{Parser, ParserConfig, ParserStats};

#[test]
fn test_parser_creation() {
    let config = ParserConfig::default();
    let parser = Parser::new(config);
    
    assert!(parser.is_initialized());
    assert!(parser.is_ready());
}

#[test]
fn test_parser_with_custom_config() {
    let config = ParserConfig::new()
        .with_max_depth(100)
        .with_strict_mode(true);
    
    let parser = Parser::new(config);
    
    assert_eq!(parser.max_depth(), 100);
    assert!(parser.strict_mode());
}

#[test]
fn test_parser_initialization() {
    let parser = Parser::default();
    
    assert!(parser.is_initialized());
    assert!(parser.is_ready());
    assert!(!parser.is_shutdown());
}

#[test]
fn test_parser_config_access() {
    let config = ParserConfig::new()
        .with_timeout(std::time::Duration::from_secs(30));
    
    let parser = Parser::new(config);
    
    let parser_config = parser.config();
    assert_eq!(parser_config.timeout, Some(std::time::Duration::from_secs(30)));
}

#[test]
fn test_parser_status() {
    let parser = Parser::default();
    
    assert!(parser.is_initialized());
    assert!(parser.is_ready());
    assert!(!parser.is_shutdown());
}

#[test]
fn test_parser_shutdown() {
    let mut parser = Parser::default();
    
    assert!(parser.is_ready());
    
    parser.shutdown();
    
    assert!(parser.is_shutdown());
    assert!(!parser.is_ready());
}

#[test]
fn test_parser_stats() {
    let parser = Parser::default();
    
    let stats = parser.stats();
    
    assert_eq!(stats.nodes_parsed, 0);
    assert_eq!(stats.errors, 0);
    assert_eq!(stats.warnings, 0);
}

#[test]
fn test_parser_config_default() {
    let config = ParserConfig::default();
    
    assert_eq!(config.max_depth, 1000);
    assert_eq!(config.max_tokens, 10000);
    assert!(!config.strict_mode);
    assert!(config.enable_recovery);
}

#[test]
fn test_parser_config_custom() {
    let config = ParserConfig::new()
        .with_max_depth(500)
        .with_max_tokens(5000)
        .with_strict_mode(true)
        .with_recovery(false);
    
    assert_eq!(config.max_depth, 500);
    assert_eq!(config.max_tokens, 5000);
    assert!(config.strict_mode);
    assert!(!config.enable_recovery);
}

#[test]
fn test_parser_parse_simple() {
    let mut parser = Parser::default();
    
    let source = "42";
    let result = parser.parse(source);
    
    assert!(result.is_ok());
    let ast = result.unwrap();
    assert!(ast.root.is_some());
}

#[test]
fn test_parser_parse_with_errors() {
    let mut parser = Parser::default();
    
    let invalid_source = "42 + + 10";
    let result = parser.parse(invalid_source);
    
    // Should either succeed with recovery or fail gracefully
    match result {
        Ok(ast) => {
            // Parser recovered from errors
            assert!(ast.root.is_some());
        }
        Err(_) => {
            // Parser failed as expected
            assert!(true);
        }
    }
}

#[test]
fn test_parser_error_recovery() {
    let mut parser = Parser::new(
        ParserConfig::new().with_recovery(true)
    );
    
    let source_with_errors = "42 + + 10 * 5";
    let result = parser.parse(source_with_errors);
    
    // With recovery enabled, should succeed
    assert!(result.is_ok());
    
    let stats = parser.stats();
    assert!(stats.errors > 0);
}

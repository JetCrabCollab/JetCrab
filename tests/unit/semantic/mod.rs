//! Semantic Analysis Unit Tests - Mirroring src/semantic/ structure
//! 
//! This module contains unit tests for semantic analysis:
//! - analyzer.rs: Semantic analyzer
//! - errors.rs: Semantic errors
//! - scope.rs: Scope management
//! - types.rs: Type system

use jetcrab::semantic::{SemanticAnalyzer, SemanticConfig, SemanticStats};

#[test]
fn test_semantic_analyzer_creation() {
    let config = SemanticConfig::default();
    let analyzer = SemanticAnalyzer::new(config);
    
    assert!(analyzer.is_initialized());
    assert!(analyzer.scopes.is_empty());
}

#[test]
fn test_semantic_analyzer_with_custom_config() {
    let config = SemanticConfig::new()
        .with_strict_mode(true)
        .with_type_checking(true);
    
    let analyzer = SemanticAnalyzer::new(config);
    
    assert!(analyzer.strict_mode());
    assert!(analyzer.type_checking_enabled());
}

#[test]
fn test_semantic_analyzer_initialization() {
    let analyzer = SemanticAnalyzer::default();
    
    assert!(analyzer.is_initialized());
    assert!(analyzer.scopes.is_empty());
}

#[test]
fn test_semantic_analyzer_config_access() {
    let config = SemanticConfig::new()
        .with_timeout(std::time::Duration::from_secs(30));
    
    let analyzer = SemanticAnalyzer::new(config);
    
    let analyzer_config = analyzer.config();
    assert_eq!(analyzer_config.timeout, Some(std::time::Duration::from_secs(30)));
}

#[test]
fn test_semantic_analyzer_status() {
    let analyzer = SemanticAnalyzer::default();
    
    assert!(analyzer.is_initialized());
    assert!(analyzer.scopes.is_empty());
}

#[test]
fn test_semantic_analyzer_stats() {
    let analyzer = SemanticAnalyzer::default();
    
    let stats = analyzer.stats();
    
    assert_eq!(stats.scopes_created, 0);
    assert_eq!(stats.variables_declared, 0);
    assert_eq!(stats.functions_declared, 0);
    assert_eq!(stats.errors, 0);
}

#[test]
fn test_semantic_config_default() {
    let config = SemanticConfig::default();
    
    assert!(!config.strict_mode);
    assert!(config.type_checking);
    assert!(config.scope_analysis);
    assert!(config.error_recovery);
}

#[test]
fn test_semantic_config_custom() {
    let config = SemanticConfig::new()
        .with_strict_mode(true)
        .with_type_checking(false)
        .with_scope_analysis(true)
        .with_error_recovery(false);
    
    assert!(config.strict_mode);
    assert!(!config.type_checking);
    assert!(config.scope_analysis);
    assert!(!config.error_recovery);
}

#[test]
fn test_semantic_analyzer_scope_management() {
    let mut analyzer = SemanticAnalyzer::default();
    
    // Enter a new scope
    analyzer.enter_scope();
    assert_eq!(analyzer.scopes.len(), 1);
    
    // Enter another scope
    analyzer.enter_scope();
    assert_eq!(analyzer.scopes.len(), 2);
    
    // Exit a scope
    analyzer.exit_scope();
    assert_eq!(analyzer.scopes.len(), 1);
}

#[test]
fn test_semantic_analyzer_variable_declaration() {
    let mut analyzer = SemanticAnalyzer::default();
    analyzer.enter_scope();
    
    let node = jetcrab::ast::node::AstNode::new(
        jetcrab::ast::node::NodeType::Expression,
        "x".to_string(),
        None,
    );
    
    // This would normally analyze the node and add variables to scope
    // For now, we'll just test the basic structure
    assert!(analyzer.scopes.len() > 0);
}

#[test]
fn test_semantic_analyzer_type_checking() {
    let mut analyzer = SemanticAnalyzer::new(
        SemanticConfig::new().with_type_checking(true)
    );
    
    // Test type checking functionality
    let node = jetcrab::ast::node::AstNode::new(
        jetcrab::ast::node::NodeType::Expression,
        "type_check".to_string(),
        None,
    );
    
    // This would normally perform type checking
    // For now, we'll just verify the analyzer is configured for type checking
    assert!(analyzer.type_checking_enabled());
}

#[test]
fn test_semantic_analyzer_error_handling() {
    let mut analyzer = SemanticAnalyzer::new(
        SemanticConfig::new().with_error_recovery(true)
    );
    
    // Test error handling functionality
    let node = jetcrab::ast::node::AstNode::new(
        jetcrab::ast::node::NodeType::Expression,
        "error_test".to_string(),
        None,
    );
    
    // This would normally analyze the node and handle any errors
    // For now, we'll just verify the analyzer is configured for error recovery
    assert!(analyzer.error_recovery_enabled());
}

#[test]
fn test_semantic_analyzer_reset() {
    let mut analyzer = SemanticAnalyzer::default();
    
    analyzer.enter_scope();
    analyzer.enter_scope();
    
    assert_eq!(analyzer.scopes.len(), 2);
    
    analyzer.reset();
    
    assert_eq!(analyzer.scopes.len(), 0);
    assert_eq!(analyzer.current_scope_depth, 0);
}

#[test]
fn test_semantic_analyzer_current_scope() {
    let mut analyzer = SemanticAnalyzer::default();
    
    // Initially no scope
    assert!(analyzer.current_scope().is_none());
    
    // Enter a scope
    analyzer.enter_scope();
    assert!(analyzer.current_scope().is_some());
    
    // Exit scope
    analyzer.exit_scope();
    assert!(analyzer.current_scope().is_none());
}

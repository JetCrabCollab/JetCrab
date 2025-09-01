use jetcrab::semantic::analyzer::SemanticAnalyzer;
use jetcrab::semantic::scope::Scope;
use jetcrab::ast::node::{AstNode, NodeType, NodePosition};

#[test]
fn test_analyzer_creation() {
    let analyzer = SemanticAnalyzer::new();
    
    assert!(analyzer.scopes.is_empty());
    assert_eq!(analyzer.current_scope_depth, 0);
}

#[test]
fn test_scope_management() {
    let mut analyzer = SemanticAnalyzer::new();
    
    // Enter a new scope
    analyzer.enter_scope();
    assert_eq!(analyzer.current_scope_depth, 1);
    assert_eq!(analyzer.scopes.len(), 1);
    
    // Enter another scope
    analyzer.enter_scope();
    assert_eq!(analyzer.current_scope_depth, 2);
    assert_eq!(analyzer.scopes.len(), 2);
    
    // Exit a scope
    analyzer.exit_scope();
    assert_eq!(analyzer.current_scope_depth, 1);
    assert_eq!(analyzer.scopes.len(), 1);
}

#[test]
fn test_variable_declaration() {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.enter_scope();
    
    let node = AstNode::new(
        NodeType::Expression,
        "x".to_string(),
        Some(NodePosition::new(1, 1)),
    );
    
    // This would normally analyze the node and add variables to scope
    // For now, we'll just test the basic structure
    assert!(analyzer.scopes.len() > 0);
}

#[test]
fn test_analyzer_reset() {
    let mut analyzer = SemanticAnalyzer::new();
    
    analyzer.enter_scope();
    analyzer.enter_scope();
    
    assert_eq!(analyzer.current_scope_depth, 2);
    
    analyzer.reset();
    
    assert_eq!(analyzer.current_scope_depth, 0);
    assert!(analyzer.scopes.is_empty());
}

#[test]
fn test_current_scope() {
    let mut analyzer = SemanticAnalyzer::new();
    
    // Initially no scope
    assert!(analyzer.current_scope().is_none());
    
    // Enter a scope
    analyzer.enter_scope();
    assert!(analyzer.current_scope().is_some());
    
    // Exit scope
    analyzer.exit_scope();
    assert!(analyzer.current_scope().is_none());
}

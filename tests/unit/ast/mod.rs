//! AST Unit Tests - Mirroring src/ast/ structure
//! 
//! This module contains unit tests for abstract syntax tree:
//! - node.rs: AST node definitions
//! - error.rs: AST errors
//! - serialization.rs: AST serialization
//! - common/: Common AST components
//! - expressions/: Expression nodes
//! - literals/: Literal nodes
//! - statements/: Statement nodes
//! - visitor/: AST visitors

use jetcrab::ast::{AbstractSyntaxTree, AstConfig, AstStats};

#[test]
fn test_ast_creation() {
    let config = AstConfig::default();
    let ast = AbstractSyntaxTree::new(config);
    
    assert!(ast.is_initialized());
    assert!(ast.root.is_none());
}

#[test]
fn test_ast_with_custom_config() {
    let config = AstConfig::new()
        .with_max_depth(100)
        .with_max_nodes(1000);
    
    let ast = AbstractSyntaxTree::new(config);
    
    assert_eq!(ast.max_depth(), 100);
    assert_eq!(ast.max_nodes(), 1000);
}

#[test]
fn test_ast_initialization() {
    let ast = AbstractSyntaxTree::default();
    
    assert!(ast.is_initialized());
    assert!(ast.root.is_none());
}

#[test]
fn test_ast_config_access() {
    let config = AstConfig::new()
        .with_timeout(std::time::Duration::from_secs(30));
    
    let ast = AbstractSyntaxTree::new(config);
    
    let ast_config = ast.config();
    assert_eq!(ast_config.timeout, Some(std::time::Duration::from_secs(30)));
}

#[test]
fn test_ast_status() {
    let ast = AbstractSyntaxTree::default();
    
    assert!(ast.is_initialized());
    assert!(ast.root.is_none());
}

#[test]
fn test_ast_stats() {
    let ast = AbstractSyntaxTree::default();
    
    let stats = ast.stats();
    
    assert_eq!(stats.total_nodes, 0);
    assert_eq!(stats.max_depth, 0);
    assert_eq!(stats.node_types.len(), 0);
}

#[test]
fn test_ast_config_default() {
    let config = AstConfig::default();
    
    assert_eq!(config.max_depth, 1000);
    assert_eq!(config.max_nodes, 10000);
    assert!(!config.strict_mode);
    assert!(config.enable_validation);
}

#[test]
fn test_ast_config_custom() {
    let config = AstConfig::new()
        .with_max_depth(500)
        .with_max_nodes(5000)
        .with_strict_mode(true)
        .with_validation(false);
    
    assert_eq!(config.max_depth, 500);
    assert_eq!(config.max_nodes, 5000);
    assert!(config.strict_mode);
    assert!(!config.enable_validation);
}

#[test]
fn test_ast_add_node() {
    let mut ast = AbstractSyntaxTree::default();
    
    let node = jetcrab::ast::node::AstNode::new(
        jetcrab::ast::node::NodeType::Expression,
        "test".to_string(),
        None,
    );
    
    ast.add_node(node);
    
    assert!(ast.root.is_some());
    assert_eq!(ast.stats().total_nodes, 1);
}

#[test]
fn test_ast_validation() {
    let mut ast = AbstractSyntaxTree::new(
        AstConfig::new().with_validation(true)
    );
    
    // Add a valid node
    let node = jetcrab::ast::node::AstNode::new(
        jetcrab::ast::node::NodeType::Expression,
        "valid".to_string(),
        None,
    );
    
    let result = ast.add_node(node);
    assert!(result.is_ok());
}

#[test]
fn test_ast_traversal() {
    let mut ast = AbstractSyntaxTree::default();
    
    // Add some nodes to create a simple tree
    let root_node = jetcrab::ast::node::AstNode::new(
        jetcrab::ast::node::NodeType::Statement,
        "root".to_string(),
        None,
    );
    
    ast.add_node(root_node);
    
    // Traverse the tree
    let nodes: Vec<_> = ast.traverse().collect();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].value, "root");
}

#[test]
fn test_ast_serialization() {
    let mut ast = AbstractSyntaxTree::default();
    
    let node = jetcrab::ast::node::AstNode::new(
        jetcrab::ast::node::NodeType::Expression,
        "serializable".to_string(),
        None,
    );
    
    ast.add_node(node);
    
    // Test serialization (if implemented)
    let serialized = format!("{:?}", ast);
    assert!(serialized.contains("serializable"));
}

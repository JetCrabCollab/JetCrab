use jetcrab::ast::node::{AstNode, NodeType, NodePosition};

#[test]
fn test_node_creation() {
    let node = AstNode::new(
        NodeType::Expression,
        "test_expression".to_string(),
        Some(NodePosition::new(10, 5)),
    );
    
    assert!(matches!(node.node_type, NodeType::Expression));
    assert_eq!(node.value, "test_expression");
    assert!(node.position.is_some());
}

#[test]
fn test_node_without_position() {
    let node = AstNode::new(
        NodeType::Statement,
        "test_statement".to_string(),
        None,
    );
    
    assert!(matches!(node.node_type, NodeType::Statement));
    assert_eq!(node.value, "test_statement");
    assert!(node.position.is_none());
}

#[test]
fn test_node_types() {
    let expression_node = AstNode::new(NodeType::Expression, "expr".to_string(), None);
    let statement_node = AstNode::new(NodeType::Statement, "stmt".to_string(), None);
    let literal_node = AstNode::new(NodeType::Literal, "lit".to_string(), None);
    let operator_node = AstNode::new(NodeType::Operator, "op".to_string(), None);
    
    assert!(matches!(expression_node.node_type, NodeType::Expression));
    assert!(matches!(statement_node.node_type, NodeType::Statement));
    assert!(matches!(literal_node.node_type, NodeType::Literal));
    assert!(matches!(operator_node.node_type, NodeType::Operator));
}

#[test]
fn test_node_position() {
    let position = NodePosition::new(20, 15);
    
    assert_eq!(position.line, 20);
    assert_eq!(position.column, 15);
}

#[test]
fn test_node_clone() {
    let original = AstNode::new(
        NodeType::Expression,
        "original".to_string(),
        Some(NodePosition::new(5, 10)),
    );
    
    let cloned = original.clone();
    
    assert_eq!(original.node_type, cloned.node_type);
    assert_eq!(original.value, cloned.value);
    assert_eq!(original.position, cloned.position);
}

#[test]
fn test_node_debug() {
    let node = AstNode::new(
        NodeType::Literal,
        "test_value".to_string(),
        Some(NodePosition::new(8, 12)),
    );
    
    let debug_str = format!("{:?}", node);
    assert!(debug_str.contains("Literal"));
    assert!(debug_str.contains("test_value"));
    assert!(debug_str.contains("8:12"));
}

use jetcrab::ast::node::Node;
use jetcrab::ast::serialization::{deserialize_ast, serialize_ast};
use jetcrab::ast::*;
use jetcrab::vm::types::*;

#[test]
fn test_node_number() {
    let node = Node::Number(42.0);
    assert!(matches!(node, Node::Number(42.0)));
}

#[test]
fn test_node_string() {
    let node = Node::String("test".to_string());
    assert!(matches!(node, Node::String(ref s) if s == "test"));
}

#[test]
fn test_node_boolean() {
    let node = Node::Boolean(true);
    assert!(matches!(node, Node::Boolean(true)));
}

#[test]
fn test_node_null() {
    let node = Node::Null;
    assert!(matches!(node, Node::Null));
}

#[test]
fn test_node_undefined() {
    let node = Node::Undefined;
    assert!(matches!(node, Node::Undefined));
}

#[test]
fn test_node_this() {
    let node = Node::This;
    assert!(matches!(node, Node::This));
}

#[test]
fn test_node_identifier() {
    let node = Node::Identifier("x".to_string());
    assert!(matches!(node, Node::Identifier(ref s) if s == "x"));
}

#[test]
fn test_node_super() {
    let node = Node::Super(jetcrab::ast::Super { span: None });
    assert!(matches!(node, Node::Super(_)));
}

#[test]
fn test_node_debugger_statement() {
    let node = Node::DebuggerStatement(jetcrab::ast::DebuggerStatement { span: None });
    assert!(matches!(node, Node::DebuggerStatement(_)));
}

#[test]
fn test_position_new() {
    let position = Position::new(10, 5);
    assert_eq!(position.line, LineNumber::new(10));
    assert_eq!(position.column, ColumnNumber::new(5));
}

#[test]
fn test_position_equality() {
    let pos1 = Position::new(1, 1);
    let pos2 = Position::new(1, 1);
    let pos3 = Position::new(2, 1);
    assert_eq!(pos1, pos2);
    assert_ne!(pos1, pos3);
}

#[test]
fn test_line_number_new() {
    let line = LineNumber::new(42);
    assert_eq!(line.as_usize(), 42);
}

#[test]
fn test_line_number_equality() {
    let line1 = LineNumber::new(10);
    let line2 = LineNumber::new(10);
    let line3 = LineNumber::new(20);
    assert_eq!(line1, line2);
    assert_ne!(line1, line3);
}

#[test]
fn test_column_number_new() {
    let column = ColumnNumber::new(15);
    assert_eq!(column.as_usize(), 15);
}

#[test]
fn test_column_number_equality() {
    let col1 = ColumnNumber::new(5);
    let col2 = ColumnNumber::new(5);
    let col3 = ColumnNumber::new(10);
    assert_eq!(col1, col2);
    assert_ne!(col1, col3);
}

#[test]
fn test_ast_serialization() {
    let ast = Node::Number(42.0);
    let serialized = serialize_ast(&ast);
    assert!(serialized.is_ok());

    let deserialized = deserialize_ast(&serialized.unwrap());
    assert!(deserialized.is_ok());
    assert_eq!(deserialized.unwrap(), ast);
}

#[test]
fn test_ast_serialization_string() {
    let ast = Node::String("test".to_string());
    let serialized = serialize_ast(&ast);
    assert!(serialized.is_ok());

    let deserialized = deserialize_ast(&serialized.unwrap());
    assert!(deserialized.is_ok());
    assert_eq!(deserialized.unwrap(), ast);
}

#[test]
fn test_ast_serialization_boolean() {
    let ast = Node::Boolean(true);
    let serialized = serialize_ast(&ast);
    assert!(serialized.is_ok());

    let deserialized = deserialize_ast(&serialized.unwrap());
    assert!(deserialized.is_ok());
    assert_eq!(deserialized.unwrap(), ast);
}

#[test]
fn test_ast_serialization_null() {
    let ast = Node::Null;
    let serialized = serialize_ast(&ast);
    assert!(serialized.is_ok());

    let deserialized = deserialize_ast(&serialized.unwrap());
    assert!(deserialized.is_ok());
    assert_eq!(deserialized.unwrap(), ast);
}

#[test]
fn test_ast_serialization_undefined() {
    let ast = Node::Undefined;
    let serialized = serialize_ast(&ast);
    assert!(serialized.is_ok());

    let deserialized = deserialize_ast(&serialized.unwrap());
    assert!(deserialized.is_ok());
    assert_eq!(deserialized.unwrap(), ast);
}

#[test]
fn test_ast_serialization_this() {
    let ast = Node::This;
    let serialized = serialize_ast(&ast);
    assert!(serialized.is_ok());

    let deserialized = deserialize_ast(&serialized.unwrap());
    assert!(deserialized.is_ok());
    assert_eq!(deserialized.unwrap(), ast);
}

#[test]
fn test_ast_serialization_identifier() {
    let ast = Node::Identifier("x".to_string());
    let serialized = serialize_ast(&ast);
    assert!(serialized.is_ok());

    let deserialized = deserialize_ast(&serialized.unwrap());
    assert!(deserialized.is_ok());
    assert_eq!(deserialized.unwrap(), ast);
}

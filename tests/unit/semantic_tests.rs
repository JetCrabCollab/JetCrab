use jetcrab::semantic::scope::{Scope, ScopeType};
use jetcrab::semantic::types::Type;
use jetcrab::semantic::*;
use jetcrab::vm::types::*;

#[test]
fn test_semantic_analyzer_new() {
    let analyzer = SemanticAnalyzer::new();
    assert!(analyzer.scope_depth().as_usize() == 0);
}

#[test]
fn test_scope_new() {
    let scope = Scope::new();
    assert!(scope.get_local_variables().is_empty());
    assert!(scope.get_local_functions().is_empty());
    assert_eq!(*scope.scope_type(), ScopeType::Global);
}

#[test]
fn test_scope_new_global() {
    let scope = Scope::new_global();
    assert_eq!(*scope.scope_type(), ScopeType::Global);
}

#[test]
fn test_scope_with_parent() {
    let parent = Scope::new();
    let child = Scope::with_parent(parent);
    assert_eq!(*child.scope_type(), ScopeType::Block);
}

#[test]
fn test_scope_new_child() {
    let parent = Scope::new();
    let child = Scope::new_child(parent, ScopeType::Function);
    assert_eq!(*child.scope_type(), ScopeType::Function);
}

#[test]
fn test_scope_declare_variable() {
    let mut scope = Scope::new();
    scope.declare_variable("x".to_string(), Type::Number, LineNumber::new(1));

    let var_info = scope.get_variable("x");
    assert!(var_info.is_some());
    let var_info = var_info.unwrap();
    assert_eq!(var_info.name, "x");
    assert_eq!(var_info.type_info, Type::Number);
    assert!(var_info.mutable);
    assert!(var_info.initialized);
}

#[test]
fn test_scope_declare_variable_with_details() {
    let mut scope = Scope::new();
    let success = scope.declare_variable_with_details("x", Type::String, false, LineNumber::new(1));
    assert!(success);

    let var_info = scope.get_variable("x");
    assert!(var_info.is_some());
    let var_info = var_info.unwrap();
    assert_eq!(var_info.name, "x");
    assert_eq!(var_info.type_info, Type::String);
    assert!(!var_info.mutable);
    assert!(!var_info.initialized);
}

#[test]
fn test_scope_initialize_variable() {
    let mut scope = Scope::new();
    scope.declare_variable_with_details("x", Type::Number, true, LineNumber::new(1));

    let success = scope.initialize_variable("x");
    assert!(success);

    let var_info = scope.get_variable("x");
    assert!(var_info.is_some());
    assert!(var_info.unwrap().initialized);
}

#[test]
fn test_scope_get_variable_type() {
    let mut scope = Scope::new();
    scope.declare_variable("x".to_string(), Type::Boolean, LineNumber::new(1));

    let var_type = scope.get_variable_type("x");
    assert!(var_type.is_some());
    assert_eq!(var_type.unwrap(), Type::Boolean);
}

#[test]
fn test_scope_has_variable() {
    let mut scope = Scope::new();
    scope.declare_variable("x".to_string(), Type::Number, LineNumber::new(1));

    assert!(scope.has_variable("x"));
    assert!(!scope.has_variable("y"));
}

#[test]
fn test_scope_is_variable_declared() {
    let mut scope = Scope::new();
    scope.declare_variable("x".to_string(), Type::Number, LineNumber::new(1));

    assert!(scope.is_variable_declared("x"));
    assert!(!scope.is_variable_declared("y"));
}

#[test]
fn test_scope_is_variable_declared_in_current_scope() {
    let mut scope = Scope::new();
    scope.declare_variable("x".to_string(), Type::Number, LineNumber::new(1));

    assert!(scope.is_variable_declared_in_current_scope("x"));
    assert!(!scope.is_variable_declared_in_current_scope("y"));
}

#[test]
fn test_scope_declare_function() {
    let mut scope = Scope::new();
    let success = scope.declare_function(
        "test",
        vec![Type::Number, Type::String],
        Type::Boolean,
        false,
        LineNumber::new(1),
    );
    assert!(success);

    let func_info = scope.get_function("test");
    assert!(func_info.is_some());
    let func_info = func_info.unwrap();
    assert_eq!(func_info.name, "test");
    assert_eq!(func_info.param_types, vec![Type::Number, Type::String]);
    assert_eq!(func_info.return_type, Type::Boolean);
    assert!(!func_info.is_method);
}

#[test]
fn test_scope_is_function_declared() {
    let mut scope = Scope::new();
    scope.declare_function("test", vec![], Type::Any, false, LineNumber::new(1));

    assert!(scope.is_function_declared("test"));
    assert!(!scope.is_function_declared("other"));
}

#[test]
fn test_scope_scope_type() {
    let scope = Scope::new();
    assert_eq!(*scope.scope_type(), ScopeType::Global);

    let function_scope = Scope::new_child(scope, ScopeType::Function);
    assert_eq!(*function_scope.scope_type(), ScopeType::Function);
}

#[test]
fn test_scope_is_function_scope() {
    let scope = Scope::new();
    assert!(!scope.is_function_scope());

    let function_scope = Scope::new_child(scope, ScopeType::Function);
    assert!(function_scope.is_function_scope());
}

#[test]
fn test_scope_is_block_scope() {
    let scope = Scope::new();
    assert!(!scope.is_block_scope());

    let block_scope = Scope::with_parent(scope);
    assert!(block_scope.is_block_scope());
}

#[test]
fn test_scope_is_global_scope() {
    let scope = Scope::new();
    assert!(scope.is_global_scope());

    let function_scope = Scope::new_child(scope, ScopeType::Function);
    assert!(!function_scope.is_global_scope());
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

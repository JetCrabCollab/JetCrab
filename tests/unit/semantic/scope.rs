use jetcrab::semantic::scope::Scope;

#[test]
fn test_scope_creation() {
    let scope = Scope::new("test_scope".to_string());
    
    assert_eq!(scope.name, "test_scope");
    assert!(scope.variables.is_empty());
    assert!(scope.functions.is_empty());
    assert!(scope.parent.is_none());
}

#[test]
fn test_scope_with_parent() {
    let parent_scope = Scope::new("parent".to_string());
    let child_scope = Scope::with_parent("child".to_string(), Box::new(parent_scope));
    
    assert_eq!(child_scope.name, "child");
    assert!(child_scope.parent.is_some());
}

#[test]
fn test_scope_add_variable() {
    let mut scope = Scope::new("test".to_string());
    
    scope.add_variable("x".to_string(), "number".to_string());
    
    assert_eq!(scope.variables.len(), 1);
    assert_eq!(scope.variables.get("x"), Some(&"number".to_string()));
}

#[test]
fn test_scope_add_function() {
    let mut scope = Scope::new("test".to_string());
    
    scope.add_function("f".to_string(), "function".to_string());
    
    assert_eq!(scope.functions.len(), 1);
    assert_eq!(scope.functions.get("f"), Some(&"function".to_string()));
}

#[test]
fn test_scope_get_variable() {
    let mut scope = Scope::new("test".to_string());
    scope.add_variable("x".to_string(), "number".to_string());
    
    let var_type = scope.get_variable("x");
    assert_eq!(var_type, Some(&"number".to_string()));
    
    let non_existent = scope.get_variable("y");
    assert!(non_existent.is_none());
}

#[test]
fn test_scope_get_function() {
    let mut scope = Scope::new("test".to_string());
    scope.add_function("f".to_string(), "function".to_string());
    
    let func_type = scope.get_function("f");
    assert_eq!(func_type, Some(&"function".to_string()));
    
    let non_existent = scope.get_function("g");
    assert!(non_existent.is_none());
}

#[test]
fn test_scope_clear() {
    let mut scope = Scope::new("test".to_string());
    
    scope.add_variable("x".to_string(), "number".to_string());
    scope.add_function("f".to_string(), "function".to_string());
    
    assert_eq!(scope.variables.len(), 1);
    assert_eq!(scope.functions.len(), 1);
    
    scope.clear();
    
    assert!(scope.variables.is_empty());
    assert!(scope.functions.is_empty());
}

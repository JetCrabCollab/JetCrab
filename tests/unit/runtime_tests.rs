use jetcrab::runtime::function::NativeFunction;
use jetcrab::runtime::*;
use jetcrab::vm::value::Value;
use jetcrab::vm::HeapHandleId;

#[test]
fn test_function_new() {
    let function = Function::new("test".to_string(), vec!["x".to_string(), "y".to_string()]);
    assert_eq!(function.name, "test");
    assert_eq!(function.parameters.len(), 2);
    assert!(function.native_function.is_none());
}

#[test]
fn test_function_native() {
    let native_func: NativeFunction = |_context, _args| Ok(Value::Number(42.0));
    let function = Function::native("test".to_string(), vec![], native_func);
    assert_eq!(function.name, "test");
    assert!(function.native_function.is_some());
}

#[test]
fn test_function_closure_variables() {
    let mut function = Function::new("test".to_string(), vec![]);
    function.set_closure_variable("x".to_string(), Value::Number(42.0));

    let value = function.get_closure_variable("x");
    assert!(value.is_some());
    assert_eq!(*value.unwrap(), Value::Number(42.0));
}

#[test]
fn test_function_call() {
    let native_func: NativeFunction = |_context, args| {
        if let Some(Value::Number(n)) = args.first() {
            Ok(Value::Number(n * 2.0))
        } else {
            Ok(Value::Number(0.0))
        }
    };
    let function = Function::native("test".to_string(), vec!["x".to_string()], native_func);
    let mut context = Context::new();
    let args = vec![Value::Number(21.0)];

    let result = function.call(&mut context, &args);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(42.0));
}

#[test]
fn test_object_new() {
    let object = Object::new();
    assert!(object.get_property("nonexistent").is_none());
}

#[test]
fn test_object_set_get_property() {
    let mut object = Object::new();
    object.set_property("x".to_string(), Value::Number(42.0));

    let value = object.get_property("x");
    assert!(value.is_some());
    assert_eq!(*value.unwrap(), Value::Number(42.0));
}

#[test]
fn test_object_has_property() {
    let mut object = Object::new();
    object.set_property("x".to_string(), Value::String("test".to_string()));

    assert!(object.has_property("x"));
    assert!(!object.has_property("y"));
}

#[test]
fn test_object_delete_property() {
    let mut object = Object::new();
    object.set_property("x".to_string(), Value::Boolean(true));

    assert!(object.has_property("x"));
    assert!(object.delete_property("x"));
    assert!(!object.has_property("x"));
}

#[test]
fn test_object_prototype_chain() {
    let mut parent = Object::new();
    parent.set_property("x".to_string(), Value::Number(42.0));

    let mut child = Object::new();
    child.set_prototype(parent);

    let value = child.get_property("x");
    assert!(value.is_some());
    assert_eq!(*value.unwrap(), Value::Number(42.0));
}

#[test]
fn test_object_prototype_override() {
    let mut parent = Object::new();
    parent.set_property("x".to_string(), Value::Number(42.0));

    let mut child = Object::new();
    child.set_prototype(parent);
    child.set_property("x".to_string(), Value::String("override".to_string()));

    let value = child.get_property("x");
    assert!(value.is_some());
    assert_eq!(*value.unwrap(), Value::String("override".to_string()));
}

#[test]
fn test_context_new() {
    let context = Context::new();
    assert!(context.variables.is_empty());
    assert_eq!(*context.get_this(), Value::Undefined);
}

#[test]
fn test_context_set_get_variable() {
    let mut context = Context::new();
    context.set_variable("x".to_string(), Value::Number(42.0));

    let value = context.get_variable("x");
    assert!(value.is_some());
    assert_eq!(*value.unwrap(), Value::Number(42.0));
}

#[test]
fn test_context_has_variable() {
    let mut context = Context::new();
    context.set_variable("x".to_string(), Value::String("test".to_string()));

    assert!(context.has_variable("x"));
    assert!(!context.has_variable("y"));
}

#[test]
fn test_context_set_get_this() {
    let mut context = Context::new();
    let this_value = Value::Number(42.0);
    context.set_this(this_value.clone());

    assert_eq!(*context.get_this(), this_value);
}

#[test]
fn test_value_number() {
    let value = Value::Number(42.0);
    assert!(matches!(value, Value::Number(42.0)));
}

#[test]
fn test_value_string() {
    let value = Value::String("test".to_string());
    assert!(matches!(value, Value::String(ref s) if s == "test"));
}

#[test]
fn test_value_boolean() {
    let value = Value::Boolean(true);
    assert!(matches!(value, Value::Boolean(true)));
}

#[test]
fn test_value_null() {
    let value = Value::Null;
    assert!(matches!(value, Value::Null));
}

#[test]
fn test_value_undefined() {
    let value = Value::Undefined;
    assert!(matches!(value, Value::Undefined));
}

#[test]
fn test_heap_handle_id_new() {
    let handle_id = HeapHandleId::new(123);
    assert_eq!(handle_id.as_usize(), 123);
}

#[test]
fn test_heap_handle_id_equality() {
    let handle1 = HeapHandleId::new(42);
    let handle2 = HeapHandleId::new(42);
    let handle3 = HeapHandleId::new(43);
    assert_eq!(handle1, handle2);
    assert_ne!(handle1, handle3);
}

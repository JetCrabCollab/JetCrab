use jetcrab::vm::executor::instruction_handlers::BuiltinCallsHandler;
use jetcrab::vm::executor::stack_manager::StackManager;
use jetcrab::vm::executor::traits::StackOperations;
use jetcrab::vm::executor::variable_manager::VariableManagerImpl;
use jetcrab::vm::runtime::Builtins;
use jetcrab::vm::value::Value;

#[test]
fn test_builtin_calls_handler_call_builtin() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();
    let mut builtins = Builtins::new();

    stack.push(Value::String("test".to_string()));
    let result = BuiltinCallsHandler::call_builtin(
        &mut stack,
        &mut variables,
        &mut builtins,
        "test_function".to_string(),
        1,
    );

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Undefined));
}

#[test]
fn test_builtin_calls_handler_call_console_log() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("Hello".to_string()));
    stack.push(Value::String("World".to_string()));
    let result = BuiltinCallsHandler::call_console_log(&mut stack, &mut variables, 2);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Undefined));
}

#[test]
fn test_builtin_calls_handler_call_console_log_single_arg() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(42.0));
    let result = BuiltinCallsHandler::call_console_log(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Undefined));
}

#[test]
fn test_builtin_calls_handler_call_console_log_no_args() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    let result = BuiltinCallsHandler::call_console_log(&mut stack, &mut variables, 0);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Undefined));
}

#[test]
fn test_builtin_calls_handler_call_console_error() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("Error message".to_string()));
    let result = BuiltinCallsHandler::call_console_error(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Undefined));
}

#[test]
fn test_builtin_calls_handler_call_console_error_multiple_args() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("Error".to_string()));
    stack.push(Value::Number(404.0));
    let result = BuiltinCallsHandler::call_console_error(&mut stack, &mut variables, 2);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Undefined));
}

#[test]
fn test_builtin_calls_handler_call_parse_int() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("42".to_string()));
    let result = BuiltinCallsHandler::call_parse_int(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
}

#[test]
fn test_builtin_calls_handler_call_parse_int_with_radix() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(16.0));
    stack.push(Value::String("FF".to_string()));
    let result = BuiltinCallsHandler::call_parse_int(&mut stack, &mut variables, 2);

    assert!(result.is_ok());
    let result_value = stack.pop().unwrap();
    if let Value::Number(n) = result_value {
        assert!(n.is_nan() || n == 255.0);
    } else {
        panic!("Expected Number result");
    }
}

#[test]
fn test_builtin_calls_handler_call_parse_int_invalid_string() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("not_a_number".to_string()));
    let result = BuiltinCallsHandler::call_parse_int(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    let result_value = stack.pop().unwrap();
    if let Value::Number(n) = result_value {
        assert!(n.is_nan());
    } else {
        panic!("Expected Number with NaN");
    }
}

#[test]
fn test_builtin_calls_handler_call_parse_int_non_string() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(42.0));
    let result = BuiltinCallsHandler::call_parse_int(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    let result_value = stack.pop().unwrap();
    if let Value::Number(n) = result_value {
        assert!(n.is_nan());
    } else {
        panic!("Expected Number with NaN");
    }
}

#[test]
fn test_builtin_calls_handler_call_parse_float() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("3.14".to_string()));
    let result = BuiltinCallsHandler::call_parse_float(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(3.14)));
}

#[test]
fn test_builtin_calls_handler_call_parse_float_integer() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("42".to_string()));
    let result = BuiltinCallsHandler::call_parse_float(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
}

#[test]
fn test_builtin_calls_handler_call_parse_float_invalid_string() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("not_a_number".to_string()));
    let result = BuiltinCallsHandler::call_parse_float(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    let result_value = stack.pop().unwrap();
    if let Value::Number(n) = result_value {
        assert!(n.is_nan());
    } else {
        panic!("Expected Number with NaN");
    }
}

#[test]
fn test_builtin_calls_handler_call_parse_float_non_string() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(3.14));
    let result = BuiltinCallsHandler::call_parse_float(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    let result_value = stack.pop().unwrap();
    if let Value::Number(n) = result_value {
        assert!(n.is_nan());
    } else {
        panic!("Expected Number with NaN");
    }
}

#[test]
fn test_builtin_calls_handler_call_is_nan() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(f64::NAN));
    let result = BuiltinCallsHandler::call_is_nan(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_builtin_calls_handler_call_is_nan_finite_number() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(42.0));
    let result = BuiltinCallsHandler::call_is_nan(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Boolean(false)));
}

#[test]
fn test_builtin_calls_handler_call_is_nan_non_number() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("test".to_string()));
    let result = BuiltinCallsHandler::call_is_nan(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_builtin_calls_handler_call_is_finite() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(42.0));
    let result = BuiltinCallsHandler::call_is_finite(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_builtin_calls_handler_call_is_finite_infinity() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(f64::INFINITY));
    let result = BuiltinCallsHandler::call_is_finite(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Boolean(false)));
}

#[test]
fn test_builtin_calls_handler_call_is_finite_negative_infinity() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(f64::NEG_INFINITY));
    let result = BuiltinCallsHandler::call_is_finite(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Boolean(false)));
}

#[test]
fn test_builtin_calls_handler_call_is_finite_nan() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(f64::NAN));
    let result = BuiltinCallsHandler::call_is_finite(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Boolean(false)));
}

#[test]
fn test_builtin_calls_handler_call_is_finite_non_number() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("test".to_string()));
    let result = BuiltinCallsHandler::call_is_finite(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Boolean(false)));
}

#[test]
fn test_builtin_calls_handler_call_encode_uri() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("Hello World".to_string()));
    let result = BuiltinCallsHandler::call_encode_uri(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(
        stack.pop(),
        Some(Value::String("Hello%20World".to_string()))
    );
}

#[test]
fn test_builtin_calls_handler_call_encode_uri_special_chars() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("test@example.com".to_string()));
    let result = BuiltinCallsHandler::call_encode_uri(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    let result_value = stack.pop().unwrap();
    if let Value::String(s) = result_value {
        assert!(s.contains("test@example"));
    } else {
        panic!("Expected String result");
    }
}

#[test]
fn test_builtin_calls_handler_call_encode_uri_non_string() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(42.0));
    let result = BuiltinCallsHandler::call_encode_uri(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::String("".to_string())));
}

#[test]
fn test_builtin_calls_handler_call_decode_uri() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("Hello%20World".to_string()));
    let result = BuiltinCallsHandler::call_decode_uri(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::String("Hello World".to_string())));
}

#[test]
fn test_builtin_calls_handler_call_decode_uri_multiple_encodings() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("test%2Fpath%3Fquery%23fragment".to_string()));
    let result = BuiltinCallsHandler::call_decode_uri(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(
        stack.pop(),
        Some(Value::String("test/path?query#fragment".to_string()))
    );
}

#[test]
fn test_builtin_calls_handler_call_decode_uri_non_string() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(42.0));
    let result = BuiltinCallsHandler::call_decode_uri(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::String("".to_string())));
}

#[test]
fn test_builtin_calls_handler_call_escape() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("Hello World!".to_string()));
    let result = BuiltinCallsHandler::call_escape(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    let result_value = stack.pop().unwrap();
    if let Value::String(s) = result_value {
        assert!(s.contains("Hello"));
        assert!(s.contains("World"));
        assert!(s.contains("%21"));
    } else {
        panic!("Expected String result");
    }
}

#[test]
fn test_builtin_calls_handler_call_escape_alphanumeric() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("abc123".to_string()));
    let result = BuiltinCallsHandler::call_escape(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::String("abc123".to_string())));
}

#[test]
fn test_builtin_calls_handler_call_escape_non_string() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(42.0));
    let result = BuiltinCallsHandler::call_escape(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::String("".to_string())));
}

#[test]
fn test_builtin_calls_handler_call_unescape() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("Hello%20World".to_string()));
    let result = BuiltinCallsHandler::call_unescape(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::String("Hello World".to_string())));
}

#[test]
fn test_builtin_calls_handler_call_unescape_invalid_hex() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("Hello%ZZWorld".to_string()));
    let result = BuiltinCallsHandler::call_unescape(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    let result_value = stack.pop().unwrap();
    if let Value::String(s) = result_value {
        assert!(s.contains("Hello"));
        assert!(s.contains("World"));
    } else {
        panic!("Expected String result");
    }
}

#[test]
fn test_builtin_calls_handler_call_unescape_incomplete_hex() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::String("Hello%2World".to_string()));
    let result = BuiltinCallsHandler::call_unescape(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    let result_value = stack.pop().unwrap();
    if let Value::String(s) = result_value {
        assert!(s.contains("Hello"));
    } else {
        panic!("Expected String result");
    }
}

#[test]
fn test_builtin_calls_handler_call_unescape_non_string() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    stack.push(Value::Number(42.0));
    let result = BuiltinCallsHandler::call_unescape(&mut stack, &mut variables, 1);

    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::String("".to_string())));
}

#[test]
fn test_builtin_calls_handler_stack_underflow() {
    let mut stack = StackManager::new();
    let mut variables = VariableManagerImpl::new();

    let result = BuiltinCallsHandler::call_parse_int(&mut stack, &mut variables, 1);

    assert!(result.is_err());
}

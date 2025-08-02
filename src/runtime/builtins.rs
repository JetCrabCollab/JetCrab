use crate::runtime::context::Context;
use crate::runtime::function::Function;
use crate::vm::value::Value;

pub fn create_global_builtins() -> Vec<Function> {
    vec![
        Function::native(
            "console.log".to_string(),
            vec!["message".to_string()],
            console_log,
        ),
        Function::native(
            "parseInt".to_string(),
            vec!["string".to_string()],
            parse_int,
        ),
        Function::native(
            "parseFloat".to_string(),
            vec!["string".to_string()],
            parse_float,
        ),
        Function::native("isNaN".to_string(), vec!["value".to_string()], is_nan),
        Function::native("isFinite".to_string(), vec!["value".to_string()], is_finite),
    ]
}

fn console_log(_context: &mut Context, _arguments: &[Value]) -> Result<Value, String> {
    // console.log functionality removed for production
    Ok(Value::Undefined)
}

fn parse_int(_context: &mut Context, arguments: &[Value]) -> Result<Value, String> {
    if let Some(value) = arguments.first() {
        let string_value = value.to_string();
        if let Ok(int_value) = string_value.parse::<i64>() {
            Ok(Value::Number(int_value as f64))
        } else {
            Ok(Value::Number(f64::NAN))
        }
    } else {
        Ok(Value::Number(f64::NAN))
    }
}

fn parse_float(_context: &mut Context, arguments: &[Value]) -> Result<Value, String> {
    if let Some(value) = arguments.first() {
        let string_value = value.to_string();
        if let Ok(float_value) = string_value.parse::<f64>() {
            Ok(Value::Number(float_value))
        } else {
            Ok(Value::Number(f64::NAN))
        }
    } else {
        Ok(Value::Number(f64::NAN))
    }
}

fn is_nan(_context: &mut Context, arguments: &[Value]) -> Result<Value, String> {
    if let Some(value) = arguments.first() {
        let number = value.to_number();
        Ok(Value::Boolean(number.is_nan()))
    } else {
        Ok(Value::Boolean(true))
    }
}

fn is_finite(_context: &mut Context, arguments: &[Value]) -> Result<Value, String> {
    if let Some(value) = arguments.first() {
        let number = value.to_number();
        Ok(Value::Boolean(number.is_finite()))
    } else {
        Ok(Value::Boolean(false))
    }
}

use crate::vm::value::Value;
use crate::vm::executor::traits::{StackOperations, VariableManager};
use crate::vm::executor::error_handler::ExecutionError;
use crate::runtime::builtins::Builtins;

pub struct BuiltinCallsHandler;

impl BuiltinCallsHandler {
    pub fn call_builtin<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        builtins: &mut Builtins,
        function_name: String,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        args.reverse();
        
        let result = match builtins.get_function(&function_name) {
            Some(_) => Value::String(format!("builtin:{}", function_name)),
            None => Value::Undefined,
        };
        stack.push(result);
        Ok(())
    }

    pub fn call_console_log<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        
        args.reverse();
        
        print!("console.log: ");
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{:?}", arg);
        }
        println!();
        
        stack.push(Value::Undefined);
        Ok(())
    }

    pub fn call_console_error<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        
        args.reverse();
        
        eprint!("console.error: ");
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                eprint!(" ");
            }
            eprint!("{:?}", arg);
        }
        eprintln!();
        
        stack.push(Value::Undefined);
        Ok(())
    }

    pub fn call_parse_int<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        args.reverse();
        
        let result = if let Some(Value::String(s)) = args.get(0) {
            let radix = args.get(1).and_then(|v| {
                if let Value::Number(n) = v {
                    Some(*n as i32)
                } else {
                    None
                }
            }).unwrap_or(10);
            
            match i64::from_str_radix(s, radix as u32) {
                Ok(n) => Value::Number(n as f64),
                Err(_) => Value::Number(f64::NAN),
            }
        } else {
            Value::Number(f64::NAN)
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn call_parse_float<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        args.reverse();
        
        let result = if let Some(Value::String(s)) = args.get(0) {
            match s.parse::<f64>() {
                Ok(n) => Value::Number(n),
                Err(_) => Value::Number(f64::NAN),
            }
        } else {
            Value::Number(f64::NAN)
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn call_is_nan<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        args.reverse();
        
        let result = if let Some(Value::Number(n)) = args.get(0) {
            Value::Boolean(n.is_nan())
        } else {
            Value::Boolean(true)
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn call_is_finite<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        args.reverse();
        
        let result = if let Some(Value::Number(n)) = args.get(0) {
            Value::Boolean(n.is_finite())
        } else {
            Value::Boolean(false)
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn call_encode_uri<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        args.reverse();
        
        let result = if let Some(Value::String(s)) = args.get(0) {
            let encoded = s.chars().map(|c| {
                if c.is_alphanumeric() || "!*'();:@&=+$,/?#[]".contains(c) {
                    c.to_string()
                } else {
                    format!("%{:02X}", c as u32)
                }
            }).collect::<String>();
            Value::String(encoded)
        } else {
            Value::String("".to_string())
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn call_decode_uri<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        args.reverse();
        
        let result = if let Some(Value::String(s)) = args.get(0) {
            let decoded = s.replace("%20", " ")
                          .replace("%2F", "/")
                          .replace("%3F", "?")
                          .replace("%23", "#");
            Value::String(decoded)
        } else {
            Value::String("".to_string())
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn call_escape<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        args.reverse();
        
        let result = if let Some(Value::String(s)) = args.get(0) {
            let escaped = s.chars().map(|c| {
                match c {
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '@' | '*' | '_' | '+' | '-' | '.' | '/' => c.to_string(),
                    _ => format!("%{:02X}", c as u32),
                }
            }).collect::<String>();
            Value::String(escaped)
        } else {
            Value::String("".to_string())
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn call_unescape<S, V>(
        stack: &mut S,
        _variable_manager: &mut V,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        args.reverse();
        
        let result = if let Some(Value::String(s)) = args.get(0) {
            let mut unescaped = String::new();
            let mut chars = s.chars().peekable();
            
            while let Some(c) = chars.next() {
                if c == '%' {
                    if let (Some(h1), Some(h2)) = (chars.next(), chars.next()) {
                        if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                            unescaped.push(byte as char);
                            continue;
                        }
                    }
                    unescaped.push('%');
                    if let Some(h1) = chars.next() {
                        unescaped.push(h1);
                    }
                    if let Some(h2) = chars.next() {
                        unescaped.push(h2);
                    }
                } else {
                    unescaped.push(c);
                }
            }
            
            Value::String(unescaped)
        } else {
            Value::String("".to_string())
        };
        
        stack.push(result);
        Ok(())
    }
}

//! # Builtin Calls Handler
//!
//! Handles execution of JavaScript built-in functions including console operations,
//! type conversion functions, and utility functions. Provides implementations
//! for commonly used JavaScript global functions.
//!
//! ## Supported Functions
//!
//! ### Console Functions
//! - **console.log**: Output values to stdout
//! - **console.error**: Output values to stderr
//!
//! ### Type Conversion
//! - **parseInt**: Parse string to integer
//! - **parseFloat**: Parse string to floating point
//! - **isNaN**: Check if value is NaN
//! - **isFinite**: Check if value is finite
//!
//! ### String Functions
//! - **encodeURI**: Encode URI string (simplified)
//! - **decodeURI**: Decode URI string (simplified)
//! - **escape**: Escape string for URL (simplified)
//! - **unescape**: Unescape URL string (simplified)
//!
//! ## Implementation Notes
//!
//! The builtin functions are implemented with simplified behavior compared
//! to full JavaScript semantics, focusing on common use cases and basic
//! functionality expected in a VM environment.
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::instruction_handlers::BuiltinCallsHandler;
//! use jetcrab::vm::executor::stack_manager::StackManager;
//! use jetcrab::vm::executor::traits::StackOperations;
//! use jetcrab::vm::executor::variable_manager::VariableManagerImpl;
//! use jetcrab::vm::value::Value;
//!
//! let mut stack = StackManager::new();
//! let mut variables = VariableManagerImpl::new();
//! stack.push(Value::String("Hello".to_string()));
//! stack.push(Value::String("World".to_string()));
//! BuiltinCallsHandler::call_console_log(&mut stack, &mut variables, 2).unwrap();
//! stack.push(Value::String("42".to_string()));
//! BuiltinCallsHandler::call_parse_int(&mut stack, &mut variables, 1).unwrap();
//! ```

use crate::vm::executor::error_handler::ExecutionError;
use crate::vm::executor::traits::{StackOperations, VariableManager};
use crate::vm::runtime::Builtins;
use crate::vm::value::Value;

/// Handles execution of JavaScript built-in functions
///
/// Provides implementations for common JavaScript global functions
/// including console operations, type conversions, and utilities.
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
            Some(_) => Value::String(format!("builtin:{function_name}")),
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
            print!("{arg:?}");
        }
        print!("\n");

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
            eprint!("{arg:?}");
        }
        eprint!("\n");

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

        let result = if let Some(Value::String(s)) = args.first() {
            let radix = args
                .get(1)
                .and_then(|v| {
                    if let Value::Number(n) = v {
                        Some(*n as i32)
                    } else {
                        None
                    }
                })
                .unwrap_or(10);

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

        let result = if let Some(Value::String(s)) = args.first() {
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

        let result = if let Some(Value::Number(n)) = args.first() {
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

        let result = if let Some(Value::Number(n)) = args.first() {
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

        let result = if let Some(Value::String(s)) = args.first() {
            let encoded = s
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() || "!*'();:@&=+$,/?#[]".contains(c) {
                        c.to_string()
                    } else {
                        format!("%{:02X}", c as u32)
                    }
                })
                .collect::<String>();
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

        let result = if let Some(Value::String(s)) = args.first() {
            let decoded = s
                .replace("%20", " ")
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

        let result = if let Some(Value::String(s)) = args.first() {
            let escaped = s
                .chars()
                .map(|c| match c {
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '@' | '*' | '_' | '+' | '-' | '.' | '/' => {
                        c.to_string()
                    }
                    _ => format!("%{:02X}", c as u32),
                })
                .collect::<String>();
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

        let result = if let Some(Value::String(s)) = args.first() {
            let mut unescaped = String::new();
            let mut chars = s.chars().peekable();

            while let Some(c) = chars.next() {
                if c == '%' {
                    if let (Some(h1), Some(h2)) = (chars.next(), chars.next()) {
                        if let Ok(byte) = u8::from_str_radix(&format!("{h1}{h2}"), 16) {
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

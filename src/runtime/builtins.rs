use crate::runtime::context::Context;
use crate::vm::heap::HeapEntry;
use crate::vm::value::Value;
use std::collections::HashMap;

pub type BuiltinFunction = fn(&mut Context, &[Value]) -> Result<Value, String>;

pub struct Builtins {
    functions: HashMap<String, BuiltinFunction>,
}

impl Builtins {
    pub fn new() -> Self {
        let mut builtins = Self {
            functions: HashMap::new(),
        };

        builtins.register_math_functions();
        builtins.register_string_functions();
        builtins.register_array_functions();

        builtins
    }

    fn register_math_functions(&mut self) {
        self.functions
            .insert("Math.pow".to_string(), Self::math_pow);
        self.functions
            .insert("Math.abs".to_string(), Self::math_abs);
        self.functions
            .insert("Math.sqrt".to_string(), Self::math_sqrt);
        self.functions
            .insert("Math.floor".to_string(), Self::math_floor);
        self.functions
            .insert("Math.ceil".to_string(), Self::math_ceil);
        self.functions
            .insert("Math.round".to_string(), Self::math_round);
        self.functions
            .insert("Math.max".to_string(), Self::math_max);
        self.functions
            .insert("Math.min".to_string(), Self::math_min);
    }

    fn register_string_functions(&mut self) {
        self.functions.insert(
            "String.prototype.toUpperCase".to_string(),
            Self::string_to_upper,
        );
        self.functions.insert(
            "String.prototype.toLowerCase".to_string(),
            Self::string_to_lower,
        );
        self.functions
            .insert("String.prototype.trim".to_string(), Self::string_trim);
        self.functions
            .insert("String.prototype.length".to_string(), Self::string_length);

        // Also register with simpler names for direct access
        self.functions
            .insert("toUpperCase".to_string(), Self::string_to_upper);
        self.functions
            .insert("toLowerCase".to_string(), Self::string_to_lower);
        self.functions.insert("trim".to_string(), Self::string_trim);
    }

    fn register_array_functions(&mut self) {
        self.functions
            .insert("Array.prototype.push".to_string(), Self::array_push);
        self.functions
            .insert("Array.prototype.pop".to_string(), Self::array_pop);
        self.functions
            .insert("Array.prototype.length".to_string(), Self::array_length);
    }

    pub fn get_function(&self, name: &str) -> Option<&BuiltinFunction> {
        self.functions.get(name)
    }

    // Math functions
    fn math_pow(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("Math.pow requires exactly 2 arguments".to_string());
        }

        let base = match &args[0] {
            Value::Number(n) => *n,
            _ => return Err("First argument must be a number".to_string()),
        };

        let exponent = match &args[1] {
            Value::Number(n) => *n,
            _ => return Err("Second argument must be a number".to_string()),
        };

        Ok(Value::Number(base.powf(exponent)))
    }

    fn math_abs(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("Math.abs requires exactly 1 argument".to_string());
        }

        let num = match &args[0] {
            Value::Number(n) => *n,
            _ => return Err("Argument must be a number".to_string()),
        };

        Ok(Value::Number(num.abs()))
    }

    fn math_sqrt(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("Math.sqrt requires exactly 1 argument".to_string());
        }

        let num = match &args[0] {
            Value::Number(n) => *n,
            _ => return Err("Argument must be a number".to_string()),
        };

        Ok(Value::Number(num.sqrt()))
    }

    fn math_floor(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("Math.floor requires exactly 1 argument".to_string());
        }

        let num = match &args[0] {
            Value::Number(n) => *n,
            _ => return Err("Argument must be a number".to_string()),
        };

        Ok(Value::Number(num.floor()))
    }

    fn math_ceil(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("Math.ceil requires exactly 1 argument".to_string());
        }

        let num = match &args[0] {
            Value::Number(n) => *n,
            _ => return Err("Argument must be a number".to_string()),
        };

        Ok(Value::Number(num.ceil()))
    }

    fn math_round(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("Math.round requires exactly 1 argument".to_string());
        }

        let num = match &args[0] {
            Value::Number(n) => *n,
            _ => return Err("Argument must be a number".to_string()),
        };

        Ok(Value::Number(num.round()))
    }

    fn math_max(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("Math.max requires at least 1 argument".to_string());
        }

        let mut max_val = f64::NEG_INFINITY;
        for arg in args {
            if let Value::Number(n) = arg {
                if *n > max_val {
                    max_val = *n;
                }
            }
        }

        Ok(Value::Number(max_val))
    }

    fn math_min(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("Math.min requires at least 1 argument".to_string());
        }

        let mut min_val = f64::INFINITY;
        for arg in args {
            if let Value::Number(n) = arg {
                if *n < min_val {
                    min_val = *n;
                }
            }
        }

        Ok(Value::Number(min_val))
    }

    // String functions
    fn string_to_upper(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("toUpperCase requires exactly 1 argument".to_string());
        }

        let str_val = match &args[0] {
            Value::String(s) => s,
            _ => return Err("Argument must be a string".to_string()),
        };

        Ok(Value::String(str_val.to_uppercase()))
    }

    fn string_to_lower(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("toLowerCase requires exactly 1 argument".to_string());
        }

        let str_val = match &args[0] {
            Value::String(s) => s,
            _ => return Err("Argument must be a string".to_string()),
        };

        Ok(Value::String(str_val.to_lowercase()))
    }

    fn string_trim(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("trim requires exactly 1 argument".to_string());
        }

        let str_val = match &args[0] {
            Value::String(s) => s,
            _ => return Err("Argument must be a string".to_string()),
        };

        Ok(Value::String(str_val.trim().to_string()))
    }

    fn string_length(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("length requires exactly 1 argument".to_string());
        }

        let str_val = match &args[0] {
            Value::String(s) => s,
            _ => return Err("Argument must be a string".to_string()),
        };

        Ok(Value::Number(str_val.len() as f64))
    }

    // Array functions
    fn array_push(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("push requires at least 2 arguments".to_string());
        }

        let array_handle = match &args[0] {
            Value::Array(handle) => handle,
            _ => return Err("First argument must be an array".to_string()),
        };

        // Get the array from the heap
        if let Some(heap) = _context.get_heap() {
            if let Some(HeapEntry::Array(arr)) = heap.get_mut(array_handle.id()) {
                // Add all arguments except the first one (which is the array)
                for arg in &args[1..] {
                    arr.push(arg.clone());
                }

                // Return the new length
                Ok(Value::Number(arr.len() as f64))
            } else {
                Err("Array not found in heap".to_string())
            }
        } else {
            Err("Heap not available in context".to_string())
        }
    }

    fn array_pop(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("pop requires exactly 1 argument".to_string());
        }

        let array_handle = match &args[0] {
            Value::Array(handle) => handle,
            _ => return Err("Argument must be an array".to_string()),
        };

        // Get the array from the heap
        if let Some(heap) = _context.get_heap() {
            if let Some(HeapEntry::Array(arr)) = heap.get_mut(array_handle.id()) {
                if arr.is_empty() {
                    Ok(Value::Undefined)
                } else {
                    arr.pop().ok_or("Failed to pop from array".to_string())
                }
            } else {
                Err("Array not found in heap".to_string())
            }
        } else {
            Err("Heap not available in context".to_string())
        }
    }

    fn array_length(_context: &mut Context, args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("length requires exactly 1 argument".to_string());
        }

        let array_handle = match &args[0] {
            Value::Array(handle) => handle,
            _ => return Err("Argument must be an array".to_string()),
        };

        // Get the array from the heap
        if let Some(heap) = _context.get_heap() {
            if let Some(HeapEntry::Array(arr)) = heap.get(array_handle.id()) {
                Ok(Value::Number(arr.len() as f64))
            } else {
                Err("Array not found in heap".to_string())
            }
        } else {
            Err("Heap not available in context".to_string())
        }
    }
}

impl Default for Builtins {
    fn default() -> Self {
        Self::new()
    }
}

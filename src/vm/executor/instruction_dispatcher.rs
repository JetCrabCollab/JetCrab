use crate::vm::executor::error_handler::ExecutionError;
use crate::vm::executor::instruction_handlers::{
    ArithmeticHandler, BuiltinCallsHandler, ComparisonHandler, ControlFlowHandler, HeapOpsHandler,
    StackOpsHandler,
};
use crate::vm::executor::traits::{HeapOperations, StackOperations, VariableManager};
use crate::vm::frame::Frame;
use crate::vm::instructions::Instruction;
use crate::vm::registers::Registers;
use crate::vm::types::{ArraySize, ConstantIndex, GlobalIndex};
use crate::vm::value::Value;

pub struct InstructionDispatcher;

impl InstructionDispatcher {
    pub fn execute_instruction<S, H, V>(
        instruction: &Instruction,
        stack: &mut S,
        heap: &mut H,
        variable_manager: &mut V,
        frame: &mut Frame,
        registers: &mut Registers,
        builtins: &mut crate::runtime::builtins::Builtins,
    ) -> Result<Option<usize>, ExecutionError>
    where
        S: StackOperations,
        H: HeapOperations,
        V: VariableManager,
    {
        match instruction {
            // Stack operations
            Instruction::PushConst(idx) => {
                let value = frame.get_constant(*idx).unwrap_or(Value::Undefined);
                stack.push(value);
                Ok(None)
            }
            Instruction::Pop => {
                StackOpsHandler::pop(stack)?;
                Ok(None)
            }
            Instruction::Dup => {
                StackOpsHandler::dup(stack)?;
                Ok(None)
            }

            // Arithmetic operations
            Instruction::Add => {
                ArithmeticHandler::add(stack)?;
                Ok(None)
            }
            Instruction::Sub => {
                ArithmeticHandler::subtract(stack)?;
                Ok(None)
            }
            Instruction::Mul => {
                ArithmeticHandler::multiply(stack)?;
                Ok(None)
            }
            Instruction::Div => {
                ArithmeticHandler::divide(stack)?;
                Ok(None)
            }
            Instruction::Mod => {
                ArithmeticHandler::modulo(stack)?;
                Ok(None)
            }
            Instruction::Exp => {
                ArithmeticHandler::power(stack)?;
                Ok(None)
            }
            Instruction::Inc => {
                ArithmeticHandler::increment(stack)?;
                Ok(None)
            }
            Instruction::Dec => {
                ArithmeticHandler::decrement(stack)?;
                Ok(None)
            }

            // Logical operations
            Instruction::And => {
                ComparisonHandler::logical_and(stack)?;
                Ok(None)
            }
            Instruction::Or => {
                ComparisonHandler::logical_or(stack)?;
                Ok(None)
            }
            Instruction::Not => {
                ComparisonHandler::logical_not(stack)?;
                Ok(None)
            }
            Instruction::Xor => {
                ComparisonHandler::bitwise_xor(stack)?;
                Ok(None)
            }

            // Comparison operations
            Instruction::Eq => {
                ComparisonHandler::equal(stack)?;
                Ok(None)
            }
            Instruction::Ne => {
                ComparisonHandler::not_equal(stack)?;
                Ok(None)
            }
            Instruction::Lt => {
                ComparisonHandler::less_than(stack)?;
                Ok(None)
            }
            Instruction::Le => {
                ComparisonHandler::less_equal(stack)?;
                Ok(None)
            }
            Instruction::Gt => {
                ComparisonHandler::greater_than(stack)?;
                Ok(None)
            }
            Instruction::Ge => {
                ComparisonHandler::greater_equal(stack)?;
                Ok(None)
            }
            Instruction::StrictEq => {
                ComparisonHandler::strict_equal(stack)?;
                Ok(None)
            }
            Instruction::StrictNe => {
                ComparisonHandler::strict_not_equal(stack)?;
                Ok(None)
            }

            // Variable operations
            Instruction::LoadGlobal(idx) => {
                let value = variable_manager
                    .get_global((*idx).into())
                    .unwrap_or(&Value::Undefined)
                    .clone();
                stack.push(value);
                Ok(None)
            }
            Instruction::StoreGlobal(idx) => {
                let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                variable_manager.set_global((*idx).into(), value);
                Ok(None)
            }
            Instruction::LoadLocal(idx) => {
                let value = variable_manager
                    .get_local((*idx).into())
                    .unwrap_or(&Value::Undefined)
                    .clone();
                stack.push(value);
                Ok(None)
            }
            Instruction::StoreLocal(idx) => {
                let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                variable_manager.set_local((*idx).into(), value);
                Ok(None)
            }
            Instruction::LoadArg(idx) => {
                let value = variable_manager
                    .get_argument((*idx).into())
                    .unwrap_or(&Value::Undefined)
                    .clone();
                stack.push(value);
                Ok(None)
            }
            Instruction::LoadThisFunction => {
                let function_handle = frame.function_handle.clone().unwrap_or_else(|| {
                    crate::vm::handle::FunctionHandle::new(crate::vm::handle::HeapHandleId::new(0))
                });
                stack.push(Value::Function(function_handle));
                Ok(None)
            }
            Instruction::LoadThis => {
                let this_value = frame.this_value.clone().unwrap_or(Value::Undefined);
                stack.push(this_value);
                Ok(None)
            }
            Instruction::LoadClosureVar(name) => {
                let value = frame
                    .closure_vars
                    .get(name)
                    .cloned()
                    .unwrap_or(Value::Undefined);
                stack.push(value);
                Ok(None)
            }

            // Control flow
            Instruction::Jump(target_ip) => {
                let new_ip =
                    ControlFlowHandler::jump::<S, V>(stack, registers, (*target_ip).into())?;
                Ok(Some(new_ip))
            }
            Instruction::JumpIfTrue(target_ip) => {
                let new_ip = ControlFlowHandler::jump_if_true::<S, V>(
                    stack,
                    registers,
                    (*target_ip).into(),
                )?;
                Ok(Some(new_ip))
            }
            Instruction::JumpIfFalse(target_ip) => {
                let new_ip = ControlFlowHandler::jump_if_false::<S, V>(
                    stack,
                    registers,
                    (*target_ip).into(),
                )?;
                Ok(Some(new_ip))
            }
            Instruction::Call(function_index) => {
                ControlFlowHandler::call::<S, V>(
                    stack,
                    registers,
                    frame,
                    (*function_index).into(),
                )?;
                Ok(None)
            }
            Instruction::Return => {
                let new_ip =
                    ControlFlowHandler::return_from_function::<S, V>(stack, registers, frame)?;
                Ok(Some(new_ip))
            }

            // Heap operations
            Instruction::NewObject => {
                HeapOpsHandler::alloc_object(stack, heap)?;
                Ok(None)
            }
            Instruction::NewArray(size) => {
                HeapOpsHandler::alloc_array(stack, heap)?;
                Ok(None)
            }
            Instruction::SetProperty => {
                let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let property_key = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let object_handle = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                if let (Value::String(key), Value::Object(handle)) = (property_key, object_handle) {
                    HeapOpsHandler::set_object_property(stack, heap, handle, key, value)?;
                }
                Ok(None)
            }
            Instruction::SetPropertyAssign => {
                let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let property_key = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let object_handle = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                if let (Value::String(key), Value::Object(handle)) = (property_key, object_handle) {
                    HeapOpsHandler::set_object_property(stack, heap, handle, key, value)?;
                }
                Ok(None)
            }
            Instruction::GetProperty => {
                let property_key = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let object_handle = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                if let (Value::String(key), Value::Object(handle)) = (property_key, object_handle) {
                    HeapOpsHandler::get_object_property(stack, heap, handle, key)?;
                }
                Ok(None)
            }

            // Type operations
            Instruction::TypeOf => {
                let value = stack.peek().ok_or(ExecutionError::StackUnderflow)?;
                let type_name = match value {
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    Value::Boolean(_) => "boolean",
                    Value::Object(_) => "object",
                    Value::Array(_) => "array",
                    Value::Function(_) => "function",
                    Value::Null => "null",
                    Value::Undefined => "undefined",
                };
                stack.push(Value::String(type_name.to_string()));
                Ok(None)
            }
            Instruction::InstanceOf => {
                let constructor = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                let result = match (value, constructor) {
                    (Value::Object(_), Value::Function(_)) => true,
                    _ => false,
                };

                stack.push(Value::Boolean(result));
                Ok(None)
            }
            Instruction::In => {
                let property_key = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let object = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                let result = match (object, property_key) {
                    (Value::Object(handle), Value::String(key)) => {
                        heap.has_object_property(handle.id(), &key)
                    }
                    _ => false,
                };

                stack.push(Value::Boolean(result));
                Ok(None)
            }
            Instruction::Delete => {
                let property_key = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let object_handle = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                if let (Value::String(key), Value::Object(handle)) = (property_key, object_handle) {
                    HeapOpsHandler::remove_object_property(stack, heap, handle, key)?;
                }
                Ok(None)
            }
            Instruction::New => {
                let constructor = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                if let Value::Function(_) = constructor {
                    HeapOpsHandler::alloc_object(stack, heap)?;
                }
                Ok(None)
            }

            // Class operations
            Instruction::NewClass => {
                HeapOpsHandler::alloc_object(stack, heap)?;
                Ok(None)
            }
            Instruction::GetPrototype => {
                let object = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                if let Value::Object(handle) = object {
                    let prototype = heap
                        .get_object_property(handle.id(), "__proto__")
                        .unwrap_or(&Value::Undefined)
                        .clone();
                    stack.push(prototype);
                }
                Ok(None)
            }
            Instruction::SetPrototype => {
                let prototype = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let object = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                if let Value::Object(handle) = object {
                    heap.set_object_property(handle.id(), "__proto__".to_string(), prototype);
                }
                Ok(None)
            }

            // Async operations
            Instruction::Await => {
                let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                stack.push(value);
                Ok(None)
            }
            Instruction::Yield => {
                let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                stack.push(value);
                Ok(None)
            }

            // Error handling
            Instruction::Throw => {
                ControlFlowHandler::throw::<S, V>(stack, registers)?;
                Ok(None)
            }
            Instruction::Try(try_block_size, catch_block_size) => {
                let new_ip = ControlFlowHandler::try_catch::<S, V>(
                    stack,
                    registers,
                    frame,
                    (*try_block_size).into(),
                    (*catch_block_size).into(),
                )?;
                Ok(Some(new_ip))
            }
            Instruction::Catch => Ok(None),
            Instruction::Finally => {
                let new_ip = ControlFlowHandler::finally::<S, V>(stack, registers, frame)?;
                Ok(Some(new_ip))
            }

            // Other operations
            Instruction::Spread => {
                let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                stack.push(value);
                Ok(None)
            }
            Instruction::Destructure => Ok(None),
            Instruction::OptionalChain => Ok(None),
            Instruction::NullishCoalesce => {
                let right = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let left = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                let result = match left {
                    Value::Null | Value::Undefined => right,
                    _ => left,
                };

                stack.push(result);
                Ok(None)
            }

            // Push constants
            Instruction::PushNull => {
                stack.push(Value::Null);
                Ok(None)
            }
            Instruction::PushUndefined => {
                stack.push(Value::Undefined);
                Ok(None)
            }
            Instruction::PushTrue => {
                stack.push(Value::Boolean(true));
                Ok(None)
            }
            Instruction::PushFalse => {
                stack.push(Value::Boolean(false));
                Ok(None)
            }
            Instruction::PushSymbol(idx) => {
                let symbol = frame
                    .get_constant(*idx)
                    .unwrap_or(Value::String("Symbol".to_string()));
                stack.push(symbol);
                Ok(None)
            }
            Instruction::PushBigInt(idx) => {
                let bigint = frame.get_constant(*idx).unwrap_or(Value::Number(0.0));
                stack.push(bigint);
                Ok(None)
            }

            // Function calls
            Instruction::CallFunction(function_index, arg_count) => {
                ControlFlowHandler::call::<S, V>(stack, registers, frame, (*arg_count).into())?;
                Ok(None)
            }
            Instruction::CallBuiltin(function_name, arg_count) => {
                BuiltinCallsHandler::call_builtin(
                    stack,
                    variable_manager,
                    builtins,
                    function_name.clone(),
                    (*arg_count).into(),
                )?;
                Ok(None)
            }

            // Object methods
            Instruction::RemoveObjectProperty => {
                let property_key = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let object_handle = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                if let (Value::String(key), Value::Object(handle)) = (property_key, object_handle) {
                    HeapOpsHandler::remove_object_property(stack, heap, handle, key)?;
                }
                Ok(None)
            }
            Instruction::CallObjectMethod(method_name, arg_count) => {
                let object = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                if let Value::Object(handle) = object {
                    let method = heap
                        .get_object_property(handle.id(), method_name)
                        .unwrap_or(&Value::Undefined)
                        .clone();
                    stack.push(method);
                    BuiltinCallsHandler::call_builtin(
                        stack,
                        variable_manager,
                        builtins,
                        method_name.clone(),
                        (*arg_count).into(),
                    )?;
                }
                Ok(None)
            }
            Instruction::CallArrayMethod(method_name, arg_count) => {
                let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                if let Value::Array(handle) = array {
                    let method = heap
                        .get_array_element(handle.id(), ArraySize::new(0))
                        .unwrap_or(&Value::Undefined)
                        .clone();
                    stack.push(method);
                    BuiltinCallsHandler::call_builtin(
                        stack,
                        variable_manager,
                        builtins,
                        method_name.clone(),
                        (*arg_count).into(),
                    )?;
                }
                Ok(None)
            }

            // Array operations
            Instruction::GetArrayLength => {
                let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                if let Value::Array(handle) = array {
                    let length = heap.get_array_length(handle.id());
                    stack.push(Value::Number(length as f64));
                }
                Ok(None)
            }
            Instruction::RemoveArrayElement(index) => {
                let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                if let Value::Array(handle) = array {
                    let element = heap
                        .get_array_element(handle.id(), *index)
                        .unwrap_or(&Value::Undefined)
                        .clone();
                    stack.push(element);
                }
                Ok(None)
            }
            Instruction::PushArrayElement => {
                let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                if let Value::Array(handle) = array {
                    HeapOpsHandler::push_array_element(stack, heap, handle, value)?;
                }
                Ok(None)
            }
            Instruction::PopArrayElement => {
                let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                if let Value::Array(handle) = array {
                    let element = heap
                        .get_array_element(handle.id(), ArraySize::new(0))
                        .unwrap_or(&Value::Undefined)
                        .clone();
                    stack.push(element);
                }
                Ok(None)
            }
            Instruction::ShiftArrayElement => {
                let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                if let Value::Array(handle) = array {
                    let element = heap
                        .get_array_element(handle.id(), ArraySize::new(0))
                        .unwrap_or(&Value::Undefined)
                        .clone();
                    stack.push(element);
                }
                Ok(None)
            }
            Instruction::UnshiftArrayElement(size) => {
                let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                if let Value::Array(handle) = array {
                    heap.set_array_element(handle.id(), *size, value);
                }
                Ok(None)
            }
            Instruction::SliceArray(start, end) => {
                let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                if let Value::Array(handle) = array {
                    let start_idx = usize::from(*start);
                    let end_idx = usize::from(*end);

                    let mut elements = Vec::new();
                    for i in start_idx..end_idx {
                        if let Some(element) =
                            heap.get_array_element(handle.id(), ArraySize::new(i))
                        {
                            elements.push(element.clone());
                        }
                    }

                    let new_array = heap.alloc_array();
                    let new_handle =
                        crate::vm::handle::HeapHandle::<crate::vm::handle::ArrayEntry>::new(
                            new_array,
                        );

                    for (i, element) in elements.iter().enumerate() {
                        heap.set_array_element(new_array, ArraySize::new(i), element.clone());
                    }

                    stack.push(Value::Array(new_handle));
                }
                Ok(None)
            }
            Instruction::ConcatArray(count) => {
                let mut arrays = Vec::new();
                for _ in 0..(*count).into() {
                    let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                    if let Value::Array(handle) = array {
                        arrays.push(handle);
                    }
                }

                let result_array = heap.alloc_array();
                let result_handle =
                    crate::vm::handle::HeapHandle::<crate::vm::handle::ArrayEntry>::new(
                        result_array,
                    );

                let mut index = 0;
                for array_handle in arrays.iter().rev() {
                    let length = heap.get_array_length(array_handle.id());
                    for i in 0..length {
                        if let Some(element) =
                            heap.get_array_element(array_handle.id(), ArraySize::new(i))
                        {
                            heap.set_array_element(
                                result_array,
                                ArraySize::new(index),
                                element.clone(),
                            );
                            index += 1;
                        }
                    }
                }

                stack.push(Value::Array(result_handle));
                Ok(None)
            }
            Instruction::IndexOfArray(target) => {
                let target_value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                if let Value::Array(handle) = array {
                    let length = heap.get_array_length(handle.id());
                    let mut found_index = -1;

                    for i in 0..length {
                        if let Some(element) =
                            heap.get_array_element(handle.id(), ArraySize::new(i))
                        {
                            if element == &target_value {
                                found_index = i as i32;
                                break;
                            }
                        }
                    }

                    stack.push(Value::Number(found_index as f64));
                }
                Ok(None)
            }
            Instruction::IncludesArray(target) => {
                let target_value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
                let array = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

                if let Value::Array(handle) = array {
                    let length = heap.get_array_length(handle.id());
                    let mut found = false;

                    for i in 0..length {
                        if let Some(element) =
                            heap.get_array_element(handle.id(), ArraySize::new(i))
                        {
                            if element == &target_value {
                                found = true;
                                break;
                            }
                        }
                    }

                    stack.push(Value::Boolean(found));
                }
                Ok(None)
            }

            // Halt
            Instruction::Halt => Ok(None),
        }
    }
}

use crate::vm::bytecode::Bytecode;
use crate::vm::frame::Frame;
use crate::vm::handle::{ArrayHandle, FunctionHandle, HeapHandleId, ObjectHandle};
use crate::vm::heap::{Heap, HeapEntry};
use crate::vm::instructions::Instruction;
use crate::vm::registers::Registers;
use crate::vm::stack::Stack;
use crate::vm::types::{ArgIndex, CodeAddress};
use crate::vm::value::Value;

pub struct Executor {
    pub stack: Stack,
    pub frame: Frame,
    pub registers: Registers,
    pub heap: Heap,
    pub globals: Vec<Value>,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            frame: Frame::new(),
            registers: Registers::new(),
            heap: Heap::new(),
            globals: vec![Value::Undefined; 32],
        }
    }

    pub fn execute(&mut self, bytecode: &Bytecode, constants: &[Value]) {
        let mut ip = 0;
        let mut locals = vec![Value::Undefined; 16];
        let mut call_stack = Vec::new();

        while ip < bytecode.instructions.len() {
            match &bytecode.instructions[ip] {
                Instruction::PushConst(idx) => {
                    let value = constants
                        .get(idx.as_usize())
                        .cloned()
                        .unwrap_or(Value::Undefined);
                    self.stack.push(value);
                }
                Instruction::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a.clone(), b.clone()) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a + b));
                        }
                        _ => {
                            let a_str = a.to_string();
                            let b_str = b.to_string();
                            self.stack.push(Value::String(format!("{a_str}{b_str}")));
                        }
                    }
                }
                Instruction::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a - b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a * b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a / b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Mod => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a % b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Exp => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a.powf(b)));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(a == b));
                }
                Instruction::Ne => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(a != b));
                }
                Instruction::Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a < b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a > b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Le => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a <= b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Ge => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a >= b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::And => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(
                        a.as_bool().unwrap_or(false) && b.as_bool().unwrap_or(false),
                    ));
                }
                Instruction::Or => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(
                        a.as_bool().unwrap_or(false) || b.as_bool().unwrap_or(false),
                    ));
                }
                Instruction::Not => {
                    let a = self.stack.pop().unwrap();
                    self.stack
                        .push(Value::Boolean(!a.as_bool().unwrap_or(false)));
                }
                Instruction::Jump(target) => {
                    ip = target.as_usize();
                    continue;
                }
                Instruction::JumpIfTrue(target) => {
                    let cond = self.stack.pop().unwrap();
                    if cond.as_bool().unwrap_or(false) {
                        ip = target.as_usize();
                        continue;
                    }
                }
                Instruction::JumpIfFalse(target) => {
                    let cond = self.stack.pop().unwrap();
                    if !cond.as_bool().unwrap_or(false) {
                        ip = target.as_usize();
                        continue;
                    }
                }
                Instruction::LoadLocal(idx) => {
                    let value = locals
                        .get(idx.as_usize())
                        .cloned()
                        .unwrap_or(Value::Undefined);
                    self.stack.push(value);
                }
                Instruction::StoreLocal(idx) => {
                    let value = self.stack.pop().unwrap();
                    if let Some(slot) = locals.get_mut(idx.as_usize()) {
                        *slot = value;
                    }
                }
                Instruction::LoadGlobal(idx) => {
                    self.stack.push(
                        self.globals
                            .get(idx.as_usize())
                            .cloned()
                            .unwrap_or(Value::Undefined),
                    );
                }
                Instruction::StoreGlobal(idx) => {
                    let value = self.stack.pop().unwrap();
                    if let Some(slot) = self.globals.get_mut(idx.as_usize()) {
                        *slot = value;
                    }
                }
                Instruction::Call(argc) => {
                    let func_value = if let Some(top_value) = self.stack.values.last() {
                        if let Value::Function(_) = top_value {
                            self.stack.pop().unwrap()
                        } else {
                            let mut found_func = None;
                            for (i, value) in self.stack.values.iter().enumerate().rev() {
                                if let Value::Function(_) = value {
                                    found_func = Some((i, value.clone()));
                                    break;
                                }
                            }
                            if let Some((index, func)) = found_func {
                                self.stack.values.remove(index);
                                func
                            } else {
                                panic!("Nenhuma função encontrada na stack para Call");
                            }
                        }
                    } else {
                        panic!("Stack vazia ao executar Call");
                    };

                    if let Value::Function(handle) = func_value {
                        let (bytecode, closure_vars) = if let Some(HeapEntry::Function {
                            bytecode,
                            closure_vars,
                            ..
                        }) = self.heap.get(handle.id())
                        {
                            (bytecode.clone(), closure_vars.clone())
                        } else {
                            panic!("Handle de função inválido no heap");
                        };

                        let mut args = Vec::new();
                        for _ in 0..argc.as_usize() {
                            args.push(self.stack.pop().unwrap());
                        }
                        args.reverse();

                        let this_value = self.stack.pop();

                        let mut new_frame = Frame::new();
                        new_frame.return_address = CodeAddress::new(ip + 1);
                        new_frame.arg_count = ArgIndex::new(argc.as_usize());
                        new_frame.arguments = args;
                        new_frame.closure_vars = closure_vars;
                        new_frame.function_handle = Some(handle);
                        new_frame.this_value = this_value;

                        self.stack.push_frame(self.frame.clone());
                        self.frame = new_frame;

                        self.execute(&bytecode, constants);

                        if let Some(prev_frame) = self.stack.pop_frame() {
                            self.frame = prev_frame;
                        }

                        if let Some(return_ip) = call_stack.pop() {
                            ip = return_ip;
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        panic!("Topo da stack não é uma função ao executar Call");
                    }
                }
                Instruction::CallFunction(handle, argc) => {
                    if let Some(HeapEntry::Function {
                        bytecode,
                        closure_vars,
                        ..
                    }) = self.heap.get(HeapHandleId::from(handle.as_usize()))
                    {
                        let bytecode = bytecode.clone();
                        let closure_vars = closure_vars.clone();

                        let mut args = Vec::new();
                        for _ in 0..argc.as_usize() {
                            args.push(self.stack.pop().unwrap());
                        }
                        args.reverse();

                        let this_value = self.stack.pop();

                        let mut new_frame = Frame::new();
                        new_frame.return_address = CodeAddress::new(ip + 1);
                        new_frame.arg_count = ArgIndex::new(argc.as_usize());
                        new_frame.arguments = args;
                        new_frame.closure_vars = closure_vars;
                        new_frame.function_handle = Some(FunctionHandle::from(handle.as_usize()));
                        new_frame.this_value = this_value;

                        self.stack.push_frame(self.frame.clone());
                        self.frame = new_frame;

                        self.execute(&bytecode, constants);

                        if let Some(prev_frame) = self.stack.pop_frame() {
                            self.frame = prev_frame;
                        }

                        if let Some(return_ip) = call_stack.pop() {
                            ip = return_ip;
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        panic!("Handle de função inválido no heap: {handle}");
                    }
                }
                Instruction::Return => {
                    let return_value = self.stack.pop();

                    if let Some(prev_frame) = self.stack.pop_frame() {
                        self.frame = prev_frame;
                    }

                    if let Some(value) = return_value {
                        self.stack.push(value);
                    }

                    if let Some(return_ip) = call_stack.pop() {
                        ip = return_ip;
                        continue;
                    } else {
                        break;
                    }
                }
                Instruction::Pop => {
                    self.stack.pop();
                }
                Instruction::Dup => {
                    if let Some(top) = self.stack.values.last().cloned() {
                        self.stack.push(top);
                    }
                }
                Instruction::NewObject => {
                    let handle = self.heap.alloc_object();
                    self.stack
                        .push(Value::Object(ObjectHandle::from(handle.as_usize())));
                }
                Instruction::NewArray(_size) => {
                    let handle = self.heap.alloc_array();
                    self.stack
                        .push(Value::Array(ArrayHandle::from(handle.as_usize())));
                }
                Instruction::SetProperty => {
                    if self.stack.values.len() < 3 {
                        panic!("Stack underflow in SetProperty");
                    }
                    let value = self.stack.pop().unwrap();
                    let key = self.stack.pop().unwrap();
                    let obj = self.stack.pop().unwrap();
                    match (obj, key) {
                        (Value::Object(handle), Value::String(key_str)) => {
                            self.heap.set_object_property(handle.id(), key_str, value);
                        }
                        (obj, key) => {
                            panic!("Invalid types for SetProperty: obj={obj:?}, key={key:?}",);
                        }
                    }
                }
                Instruction::GetProperty => {
                    let key = self.stack.pop().unwrap();
                    let obj = self.stack.pop().unwrap();
                    if let (Value::Object(handle), Value::String(key)) = (obj, key) {
                        if let Some(val) = self.heap.get_object_property(handle.id(), &key) {
                            self.stack.push(val.clone());
                        } else {
                            self.stack.push(Value::Undefined);
                        }
                    } else {
                        self.stack.push(Value::Undefined);
                    }
                }
                Instruction::LoadArg(idx) => {
                    let value = self
                        .frame
                        .arguments
                        .get(idx.as_usize())
                        .cloned()
                        .unwrap_or(Value::Undefined);
                    self.stack.push(value);
                }
                Instruction::LoadThisFunction => {
                    if let Some(func_handle) = &self.frame.function_handle {
                        self.stack.push(Value::Function(func_handle.clone()));
                    } else {
                        panic!("LoadThisFunction chamado fora de uma função");
                    }
                }
                Instruction::LoadThis => {
                    if let Some(this_val) = &self.frame.this_value {
                        self.stack.push(this_val.clone());
                    } else {
                        self.stack.push(Value::Undefined);
                    }
                }
                Instruction::LoadClosureVar(name) => {
                    if let Some(value) = self.frame.closure_vars.get(name) {
                        self.stack.push(value.clone());
                    } else {
                        self.stack.push(Value::Undefined);
                    }
                }
                Instruction::PushTrue => {
                    self.stack.push(Value::Boolean(true));
                }
                Instruction::PushFalse => {
                    self.stack.push(Value::Boolean(false));
                }
                Instruction::PushNull => {
                    self.stack.push(Value::Null);
                }
                Instruction::PushUndefined => {
                    self.stack.push(Value::Undefined);
                }
                Instruction::TypeOf => {
                    let value = self.stack.pop().unwrap();
                    let type_str = match value {
                        Value::Number(_) => "number",
                        Value::String(_) => "string",
                        Value::Boolean(_) => "boolean",
                        Value::Null => "object", // JavaScript quirk
                        Value::Undefined => "undefined",
                        Value::Object(_) => "object",
                        Value::Array(_) => "object",
                        Value::Function(_) => "function",
                    };
                    self.stack.push(Value::String(type_str.to_string()));
                }
                _ => todo!("Instrução não implementada ainda"),
            }
            ip += 1;
        }
    }
}

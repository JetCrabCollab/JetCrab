use crate::vm::bytecode::Bytecode;
use crate::vm::value::Value;
use super::{
    InstructionExecutor,
    stack_manager::StackManager,
    heap_manager::HeapManager,
    variable_manager::VariableManagerImpl,
    instruction_executor::InstructionExecutorImpl,
};

pub struct Executor {
    instruction_executor: InstructionExecutorImpl<StackManager, HeapManager, VariableManagerImpl>,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    pub fn new() -> Self {
        let stack_manager = StackManager::new();
        let heap_manager = HeapManager::new();
        let variable_manager = VariableManagerImpl::new();
        
        let instruction_executor = InstructionExecutorImpl::new(
            stack_manager,
            heap_manager,
            variable_manager,
        );

        Self {
            instruction_executor,
        }
    }

    pub fn execute(&mut self, bytecode: &Bytecode, constants: &[Value]) -> Result<(), crate::vm::executor::error_handler::ExecutionError> {
        self.instruction_executor.execute(bytecode, constants)
    }

    pub fn stack(&self) -> &crate::vm::stack::Stack {
        self.instruction_executor.stack_manager().stack()
    }

    pub fn stack_mut(&mut self) -> &mut crate::vm::stack::Stack {
        self.instruction_executor.stack_manager_mut().stack_mut()
    }

    pub fn heap(&self) -> &crate::vm::heap::Heap {
        self.instruction_executor.heap_manager().heap()
    }

    pub fn heap_mut(&mut self) -> &mut crate::vm::heap::Heap {
        self.instruction_executor.heap_manager_mut().heap_mut()
    }
}

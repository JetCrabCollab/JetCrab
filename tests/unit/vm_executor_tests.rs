use jetcrab::vm::executor::*;
use jetcrab::vm::value::Value;
use jetcrab::vm::memory::heap::Heap;
use jetcrab::vm::executor::stack_manager::StackManager;
use jetcrab::vm::executor::variable_manager::VariableManagerImpl;

#[test]
fn test_instruction_executor_new() {
    let executor = InstructionExecutorImpl::new(
        StackManager::new(),
        Heap::new(),
        VariableManagerImpl::new(),
    );
    assert_eq!(executor.stack_manager().size(), 0);
    assert_eq!(executor.variable_manager().get_variable_count(), 0);
}

#[test]
fn test_instruction_executor_stack_operations() {
    let mut executor = InstructionExecutorImpl::new(
        StackManager::new(),
        Heap::new(),
        VariableManagerImpl::new(),
    );
    
    executor.stack_manager_mut().push(Value::Number(42.0));
    assert_eq!(executor.stack_manager().size(), 1);
    assert_eq!(executor.stack_manager().peek(), Some(&Value::Number(42.0)));
}

#[test]
fn test_instruction_executor_variable_operations() {
    let mut executor = InstructionExecutorImpl::new(
        StackManager::new(),
        Heap::new(),
        VariableManagerImpl::new(),
    );
    
    executor.variable_manager_mut().declare_variable("x", Value::Number(42.0));
    assert_eq!(executor.variable_manager().get_variable_count(), 1);
    assert_eq!(executor.variable_manager().get_variable("x"), Some(&Value::Number(42.0)));
}

#[test]
fn test_instruction_executor_heap_operations() {
    let mut executor = InstructionExecutorImpl::new(
        StackManager::new(),
        Heap::new(),
        VariableManagerImpl::new(),
    );
    
    let handle = executor.heap_manager_mut().allocate_object();
    assert!(handle.is_some());
}

#[test]
fn test_stack_ops_handler_push() {
    let mut stack = StackManager::new();
    let result = StackOpsHandler::push(&mut stack, Value::Number(42.0));
    assert!(result.is_ok());
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek(), Some(&Value::Number(42.0)));
}

#[test]
fn test_stack_ops_handler_pop() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(42.0));
    let result = StackOpsHandler::pop(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_stack_ops_handler_pop_empty() {
    let mut stack = StackManager::new();
    let result = StackOpsHandler::pop(&mut stack);
    assert!(result.is_err());
}

#[test]
fn test_stack_ops_handler_dup() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(42.0));
    let result = StackOpsHandler::dup(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek(), Some(&Value::Number(42.0)));
}

#[test]
fn test_stack_ops_handler_dup_empty() {
    let mut stack = StackManager::new();
    let result = StackOpsHandler::dup(&mut stack);
    assert!(result.is_err());
}

#[test]
fn test_stack_ops_handler_dup2() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    let result = StackOpsHandler::dup2(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.size(), 4);
}

#[test]
fn test_stack_ops_handler_dup2_insufficient() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    let result = StackOpsHandler::dup2(&mut stack);
    assert!(result.is_err());
}

#[test]
fn test_stack_ops_handler_swap() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    let result = StackOpsHandler::swap(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(1.0)));
    assert_eq!(stack.pop(), Some(Value::Number(2.0)));
}

#[test]
fn test_stack_ops_handler_swap_insufficient() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    let result = StackOpsHandler::swap(&mut stack);
    assert!(result.is_err());
}

#[test]
fn test_stack_ops_handler_rot() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    stack.push(Value::Number(3.0));
    let result = StackOpsHandler::rot(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(1.0)));
    assert_eq!(stack.pop(), Some(Value::Number(3.0)));
    assert_eq!(stack.pop(), Some(Value::Number(2.0)));
}

#[test]
fn test_stack_ops_handler_rot_insufficient() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    let result = StackOpsHandler::rot(&mut stack);
    assert!(result.is_err());
}

#[test]
fn test_stack_ops_handler_over() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    let result = StackOpsHandler::over(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.pop(), Some(Value::Number(1.0)));
    assert_eq!(stack.pop(), Some(Value::Number(2.0)));
    assert_eq!(stack.pop(), Some(Value::Number(1.0)));
}

#[test]
fn test_stack_ops_handler_over_insufficient() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    let result = StackOpsHandler::over(&mut stack);
    assert!(result.is_err());
}

#[test]
fn test_stack_ops_handler_drop() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(42.0));
    let result = StackOpsHandler::drop(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_stack_ops_handler_drop2() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    let result = StackOpsHandler::drop2(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_stack_ops_handler_clear() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    stack.push(Value::Number(3.0));
    let result = StackOpsHandler::clear(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_stack_ops_handler_size() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    let result = StackOpsHandler::size(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(2.0)));
}

#[test]
fn test_stack_ops_handler_is_empty() {
    let mut stack = StackManager::new();
    let result = StackOpsHandler::is_empty(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_stack_ops_handler_is_not_empty() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(42.0));
    let result = StackOpsHandler::is_empty(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Boolean(false)));
}

#[test]
fn test_stack_ops_handler_peek() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(42.0));
    let result = StackOpsHandler::peek(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
}

#[test]
fn test_stack_ops_handler_depth() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    let result = StackOpsHandler::depth(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(2.0)));
}

#[test]
fn test_stack_ops_handler_reserve() {
    let mut stack = StackManager::new();
    let result = StackOpsHandler::reserve(&mut stack, 100);
    assert!(result.is_ok());
}

#[test]
fn test_stack_ops_handler_truncate() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    stack.push(Value::Number(3.0));
    let result = StackOpsHandler::truncate(&mut stack, 2);
    assert!(result.is_ok());
    assert_eq!(stack.size(), 2);
}

#[test]
fn test_stack_ops_handler_truncate_too_large() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    let result = StackOpsHandler::truncate(&mut stack, 5);
    assert!(result.is_err());
}

#[test]
fn test_arithmetic_handler_add() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(3.0));
    stack.push(Value::Number(4.0));
    let result = ArithmeticHandler::add(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(7.0)));
}

#[test]
fn test_arithmetic_handler_subtract() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(10.0));
    stack.push(Value::Number(3.0));
    let result = ArithmeticHandler::subtract(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(7.0)));
}

#[test]
fn test_arithmetic_handler_multiply() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(6.0));
    stack.push(Value::Number(7.0));
    let result = ArithmeticHandler::multiply(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
}

#[test]
fn test_arithmetic_handler_divide() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(15.0));
    stack.push(Value::Number(3.0));
    let result = ArithmeticHandler::divide(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(5.0)));
}

#[test]
fn test_arithmetic_handler_modulo() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(7.0));
    stack.push(Value::Number(3.0));
    let result = ArithmeticHandler::modulo(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(1.0)));
}

#[test]
fn test_arithmetic_handler_power() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(2.0));
    stack.push(Value::Number(3.0));
    let result = ArithmeticHandler::power(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(8.0)));
}

#[test]
fn test_arithmetic_handler_negate() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(42.0));
    let result = ArithmeticHandler::negate(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(-42.0)));
}

#[test]
fn test_arithmetic_handler_increment() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(41.0));
    let result = ArithmeticHandler::increment(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
}

#[test]
fn test_arithmetic_handler_decrement() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(43.0));
    let result = ArithmeticHandler::decrement(&mut stack);
    assert!(result.is_ok());
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
}

#[test]
fn test_heap_ops_handler_allocate() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    stack.push(Value::Number(64.0));
    let result = HeapOpsHandler::allocate(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_heap_ops_handler_deallocate() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let handle = heap.allocate_object();
    stack.push(Value::Handle(handle.unwrap()));
    let result = HeapOpsHandler::deallocate(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_heap_ops_handler_get_property() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let handle = heap.allocate_object();
    stack.push(Value::String("test".to_string()));
    stack.push(Value::Handle(handle.unwrap()));
    let result = HeapOpsHandler::get_property(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_heap_ops_handler_set_property() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let handle = heap.allocate_object();
    stack.push(Value::Number(42.0));
    stack.push(Value::String("test".to_string()));
    stack.push(Value::Handle(handle.unwrap()));
    let result = HeapOpsHandler::set_property(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_object_handler_create() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let result = ObjectHandler::create(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_object_handler_get_property() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let handle = heap.allocate_object();
    stack.push(Value::String("test".to_string()));
    stack.push(Value::Handle(handle.unwrap()));
    let result = ObjectHandler::get_property(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_object_handler_set_property() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let handle = heap.allocate_object();
    stack.push(Value::Number(42.0));
    stack.push(Value::String("test".to_string()));
    stack.push(Value::Handle(handle.unwrap()));
    let result = ObjectHandler::set_property(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_object_handler_delete_property() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let handle = heap.allocate_object();
    stack.push(Value::String("test".to_string()));
    stack.push(Value::Handle(handle.unwrap()));
    let result = ObjectHandler::delete_property(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_object_handler_has_property() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let handle = heap.allocate_object();
    stack.push(Value::String("test".to_string()));
    stack.push(Value::Handle(handle.unwrap()));
    let result = ObjectHandler::has_property(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_object_handler_get_prototype() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let handle = heap.allocate_object();
    stack.push(Value::Handle(handle.unwrap()));
    let result = ObjectHandler::get_prototype(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_object_handler_set_prototype() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let handle1 = heap.allocate_object();
    let handle2 = heap.allocate_object();
    stack.push(Value::Handle(handle2.unwrap()));
    stack.push(Value::Handle(handle1.unwrap()));
    let result = ObjectHandler::set_prototype(&mut stack, &mut heap);
    assert!(result.is_ok());
}

#[test]
fn test_stack_manager_new() {
    let stack = StackManager::new();
    assert_eq!(stack.size(), 0);
    assert!(stack.is_empty());
}

#[test]
fn test_stack_manager_push_pop() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(42.0));
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_stack_manager_peek() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(42.0));
    assert_eq!(stack.peek(), Some(&Value::Number(42.0)));
    assert_eq!(stack.size(), 1);
}

#[test]
fn test_stack_manager_clear() {
    let mut stack = StackManager::new();
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    stack.clear();
    assert_eq!(stack.size(), 0);
    assert!(stack.is_empty());
}

#[test]
fn test_variable_manager_new() {
    let manager = VariableManagerImpl::new();
    assert_eq!(manager.get_variable_count(), 0);
}

#[test]
fn test_variable_manager_declare_get() {
    let mut manager = VariableManagerImpl::new();
    manager.declare_variable("x", Value::Number(42.0));
    assert_eq!(manager.get_variable_count(), 1);
    assert_eq!(manager.get_variable("x"), Some(&Value::Number(42.0)));
}

#[test]
fn test_variable_manager_set() {
    let mut manager = VariableManagerImpl::new();
    manager.declare_variable("x", Value::Number(0.0));
    manager.set_variable("x", Value::Number(42.0));
    assert_eq!(manager.get_variable("x"), Some(&Value::Number(42.0)));
}

#[test]
fn test_variable_manager_undeclared() {
    let manager = VariableManagerImpl::new();
    assert_eq!(manager.get_variable("x"), None);
}

#[test]
fn test_variable_manager_clear() {
    let mut manager = VariableManagerImpl::new();
    manager.declare_variable("x", Value::Number(42.0));
    manager.clear();
    assert_eq!(manager.get_variable_count(), 0);
    assert_eq!(manager.get_variable("x"), None);
}

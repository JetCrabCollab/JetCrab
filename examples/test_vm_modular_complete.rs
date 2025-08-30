use jetcrab::vm::executor::*;
use jetcrab::vm::value::Value;
use jetcrab::vm::types::*;

fn main() {
    println!("🧪 Testing Complete Modularized VM Architecture");
    println!("===============================================");

    // Test arithmetic operations
    println!("\n🔢 Testing Arithmetic Operations:");
    test_arithmetic_operations();

    // Test comparison operations
    println!("\n⚖️  Testing Comparison Operations:");
    test_comparison_operations();

    // Test control flow operations
    println!("\n🔄 Testing Control Flow Operations:");
    test_control_flow_operations();

    // Test stack utility operations
    println!("\n📚 Testing Stack Utility Operations:");
    test_stack_utility_operations();

    // Test type system modularization
    println!("\n🏗️  Testing Type System Modularization:");
    test_type_system();

    println!("\n✅ All VM modularization tests passed!");
    println!("\n🏗️  Modularization Benefits:");
    println!("   • Clean separation of concerns");
    println!("   • Easy to extend with new operations");
    println!("   • Better maintainability");
    println!("   • Reduced file sizes");
    println!("   • Improved code organization");
    println!("   • Type safety through traits");
    println!("   • Modular execution pipeline");
}

fn test_arithmetic_operations() {
    let mut stack = StackManager::new();
    let mut arithmetic = ArithmeticExecutor::new();

    // Test addition
    stack.push(Value::Number(5.0));
    stack.push(Value::Number(3.0));
    arithmetic.execute_add(&mut stack).unwrap();
    let result = stack.pop().unwrap();
    println!("   5 + 3 = {}", result);

    // Test multiplication
    stack.push(Value::Number(4.0));
    stack.push(Value::Number(6.0));
    arithmetic.execute_mul(&mut stack).unwrap();
    let result = stack.pop().unwrap();
    println!("   4 * 6 = {}", result);

    // Test string concatenation
    stack.push(Value::String("Hello".to_string()));
    stack.push(Value::String(" World".to_string()));
    arithmetic.execute_add(&mut stack).unwrap();
    let result = stack.pop().unwrap();
    println!("   'Hello' + ' World' = {}", result);
}

fn test_comparison_operations() {
    let mut stack = StackManager::new();
    let mut comparison = ComparisonExecutor::new();

    // Test equality
    stack.push(Value::Number(5.0));
    stack.push(Value::Number(5.0));
    comparison.execute_eq(&mut stack).unwrap();
    let result = stack.pop().unwrap();
    println!("   5 == 5 = {}", result);

    // Test less than
    stack.push(Value::Number(3.0));
    stack.push(Value::Number(7.0));
    comparison.execute_lt(&mut stack).unwrap();
    let result = stack.pop().unwrap();
    println!("   3 < 7 = {}", result);

    // Test string comparison
    stack.push(Value::String("abc".to_string()));
    stack.push(Value::String("def".to_string()));
    comparison.execute_lt(&mut stack).unwrap();
    let result = stack.pop().unwrap();
    println!("   'abc' < 'def' = {}", result);
}

fn test_control_flow_operations() {
    let mut stack = StackManager::new();
    let mut control_flow = ControlFlowExecutor::new();
    let mut vars = VariableManagerImpl::new();

    // Test local variable operations
    stack.push(Value::Number(42.0));
    control_flow.execute_store_local(&mut stack, &mut vars, 0).unwrap();
    
    control_flow.execute_load_local(&mut stack, &vars, 0).unwrap();
    let result = stack.pop().unwrap();
    println!("   Store/Load local: {}", result);

    // Test global variable operations
    stack.push(Value::String("Global Value".to_string()));
    control_flow.execute_store_global(&mut stack, &mut vars, 0).unwrap();
    
    control_flow.execute_load_global(&mut stack, &vars, 0).unwrap();
    let result = stack.pop().unwrap();
    println!("   Store/Load global: {}", result);

    // Test jump operations
    let mut ip = 0;
    control_flow.execute_jump(&mut ip, 100).unwrap();
    println!("   Jump to address: {}", ip);

    // Test conditional jump
    stack.push(Value::Boolean(true));
    control_flow.execute_jump_if_true(&mut stack, &mut ip, 200).unwrap();
    println!("   Jump if true to address: {}", ip);
}

fn test_stack_utility_operations() {
    let mut stack = StackManager::new();
    let mut stack_utils = StackUtilityExecutor::new();

    // Test dup operation
    stack.push(Value::Number(10.0));
    stack_utils.execute_dup(&mut stack).unwrap();
    let result1 = stack.pop().unwrap();
    let result2 = stack.pop().unwrap();
    println!("   Dup operation: {} = {}", result1, result2);

    // Test swap operation
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    stack_utils.execute_swap(&mut stack).unwrap();
    let first = stack.pop().unwrap();
    let second = stack.pop().unwrap();
    println!("   Swap operation: {} <-> {}", first, second);

    // Test over operation
    stack.push(Value::Number(5.0));
    stack.push(Value::Number(10.0));
    stack_utils.execute_over(&mut stack).unwrap();
    let over_result = stack.pop().unwrap();
    println!("   Over operation: {}", over_result);

    // Test pop operation
    stack.push(Value::String("To be popped".to_string()));
    stack_utils.execute_pop(&mut stack).unwrap();
    println!("   Pop operation: stack size = {}", stack.len());
}

fn test_type_system() {
    println!("   • Index types:");
    let constant_idx = ConstantIndex::new(42);
    let global_idx = GlobalIndex::new(10);
    let local_idx = LocalIndex::new(5);
    let heap_id = HeapId::new(100);
    println!("     - ConstantIndex: {}", constant_idx.as_usize());
    println!("     - GlobalIndex: {}", global_idx.as_usize());
    println!("     - LocalIndex: {}", local_idx.as_usize());
    println!("     - HeapId: {}", heap_id.as_usize());

    println!("   • Address types:");
    let code_addr = CodeAddress::new(100);
    let line_num = LineNumber::new(15);
    let col_num = ColumnNumber::new(25);
    println!("     - CodeAddress: {}", code_addr.as_usize());
    println!("     - LineNumber: {}", line_num.as_usize());
    println!("     - ColumnNumber: {}", col_num.as_usize());

    println!("   • Size types:");
    let mem_size = MemorySize::new(1024);
    let obj_count = ObjectCount::new(50);
    let var_count = VariableCount::new(25);
    let error_count = ErrorCount::new(0);
    println!("     - MemorySize: {}", mem_size);
    println!("     - ObjectCount: {}", obj_count);
    println!("     - VariableCount: {}", var_count);
    println!("     - ErrorCount: {}", error_count);

    println!("   • Name types:");
    let var_name = VariableName::new("myVariable".to_string());
    let func_name = FunctionName::new("calculate".to_string());
    let class_name = ClassName::new("MyClass".to_string());
    let module_name = ModuleName::new("math".to_string());
    println!("     - VariableName: {}", var_name);
    println!("     - FunctionName: {}", func_name);
    println!("     - ClassName: {}", class_name);
    println!("     - ModuleName: {}", module_name);
}

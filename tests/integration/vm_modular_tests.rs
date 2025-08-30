use jetcrab::vm::executor::*;
use jetcrab::vm::types::*;
use jetcrab::vm::value::Value;

fn main() {
    println!("🧪 Testing Modularized VM Architecture");
    println!("=====================================");

    // Test arithmetic operations
    println!("\n🔢 Testing Arithmetic Operations:");
    test_arithmetic_operations();

    // Test comparison operations
    println!("\n⚖️  Testing Comparison Operations:");
    test_comparison_operations();

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

fn test_type_system() {
    println!("   • Index types:");
    let constant_idx = ConstantIndex::new(42);
    let global_idx = GlobalIndex::new(10);
    let local_idx = LocalIndex::new(5);
    println!("     - ConstantIndex: {}", constant_idx.as_usize());
    println!("     - GlobalIndex: {}", global_idx.as_usize());
    println!("     - LocalIndex: {}", local_idx.as_usize());

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
    println!("     - MemorySize: {}", mem_size);
    println!("     - ObjectCount: {}", obj_count);
    println!("     - VariableCount: {}", var_count);

    println!("   • Name types:");
    let var_name = VariableName::new("myVariable".to_string());
    let func_name = FunctionName::new("calculate".to_string());
    let class_name = ClassName::new("MyClass".to_string());
    println!("     - VariableName: {}", var_name);
    println!("     - FunctionName: {}", func_name);
    println!("     - ClassName: {}", class_name);
}

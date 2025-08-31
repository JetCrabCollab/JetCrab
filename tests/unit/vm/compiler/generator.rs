//! Bytecode Generator Unit Tests
//! 
//! Tests for bytecode generation from AST

use jetcrab::vm::compiler::BytecodeGenerator;

#[test]
fn test_bytecode_generator_creation() {
    let generator = BytecodeGenerator::new();
    assert!(generator.is_ok());
}

#[test]
fn test_bytecode_generator_simple_expression() {
    let mut generator = BytecodeGenerator::new().unwrap();
    let result = generator.generate("42");
    assert!(result.is_ok());
}

#[test]
fn test_bytecode_generator_variable_declaration() {
    let mut generator = BytecodeGenerator::new().unwrap();
    let result = generator.generate("let x = 42;");
    assert!(result.is_ok());
}

#[test]
fn test_bytecode_generator_function_declaration() {
    let mut generator = BytecodeGenerator::new().unwrap();
    let result = generator.generate("function test() { return 42; }");
    assert!(result.is_ok());
}

#[test]
fn test_bytecode_generator_object_literal() {
    let mut generator = BytecodeGenerator::new().unwrap();
    let result = generator.generate("let obj = { x: 42 };");
    assert!(result.is_ok());
}

#[test]
fn test_bytecode_generator_array_literal() {
    let mut generator = BytecodeGenerator::new().unwrap();
    let result = generator.generate("let arr = [1, 2, 3];");
    assert!(result.is_ok());
}

#[test]
fn test_bytecode_generator_arithmetic_expression() {
    let mut generator = BytecodeGenerator::new().unwrap();
    let result = generator.generate("let result = 10 + 5 * 2;");
    assert!(result.is_ok());
}

#[test]
fn test_bytecode_generator_control_flow() {
    let mut generator = BytecodeGenerator::new().unwrap();
    let result = generator.generate("if (true) { return 42; } else { return 0; }");
    assert!(result.is_ok());
}

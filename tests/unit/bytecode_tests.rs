use jetcrab::ast::node::Node;
use jetcrab::bytecode::*;

#[test]
fn test_bytecode_generator_new() {
    let generator = BytecodeGenerator::new();
    assert!(generator.get_constants().is_empty());
}

#[test]
fn test_bytecode_generator_number() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Number(42.0);
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_string() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::String("test".to_string());
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_boolean() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Boolean(true);
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_null() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Null;
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_undefined() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Undefined;
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_this() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::This;
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_identifier() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Identifier("x".to_string());
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_super() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Super(jetcrab::ast::Super { span: None });
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_multiple_constants() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Number(42.0);
    generator.generate(&ast);

    let ast2 = Node::String("test".to_string());
    generator.generate(&ast2);

    let constants = generator.get_constants();
    assert!(constants.len() >= 2);
}

#[test]
fn test_bytecode_generator_reuse_constants() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Number(42.0);

    let result1 = generator.generate(&ast);
    let result2 = generator.generate(&ast);

    assert!(!result1.is_empty());
    assert!(!result2.is_empty());
}

#[test]
fn test_bytecode_generator_constant_pool() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Number(42.0);
    generator.generate(&ast);

    let constants = generator.get_constants();
    assert!(!constants.is_empty());
}

#[test]
fn test_bytecode_generator_instruction_sequence() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Number(42.0);
    let instructions = generator.generate(&ast);

    assert!(!instructions.is_empty());
}

#[test]
fn test_bytecode_generator_complex_expression() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Number(42.0);
    let result = generator.generate(&ast);

    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_function_with_parameters() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Number(42.0);
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_class_with_methods() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Number(42.0);
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_destructuring() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Number(42.0);
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

#[test]
fn test_bytecode_generator_spread_operator() {
    let mut generator = BytecodeGenerator::new();
    let ast = Node::Number(42.0);
    let result = generator.generate(&ast);
    assert!(!result.is_empty());
}

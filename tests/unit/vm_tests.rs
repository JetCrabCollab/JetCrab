use jetcrab::lexer::core::Lexer;
use jetcrab::parser::core::Parser;
use jetcrab::vm::compiler::generator::BytecodeGenerator;

#[test]
fn test_vm_pipeline_basic_expression() {
    let source = "let x = 45 + 12;";

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parser should succeed");

    let mut generator = BytecodeGenerator::new();
    let instructions = generator.generate(&ast);
    assert!(!instructions.is_empty());
}

#[test]
fn test_vm_pipeline_variable_declaration() {
    let source = "let y = 100;";

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parser should succeed");

    let mut generator = BytecodeGenerator::new();
    let instructions = generator.generate(&ast);
    assert!(!instructions.is_empty());
}

#[test]
fn test_vm_pipeline_function_declaration() {
    let source = "function add(a, b) { return a + b; }";

    let mut lexer = Lexer::new(source);
    let _tokens = lexer.tokenize().expect("Lexer should succeed");

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parser should succeed");

    let mut generator = BytecodeGenerator::new();
    let instructions = generator.generate(&ast);
    assert!(!instructions.is_empty());
}

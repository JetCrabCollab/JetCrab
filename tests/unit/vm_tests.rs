use jetcrab::bytecode::BytecodeGenerator;
use jetcrab::lexer::Lexer;
use jetcrab::parser::Parser;

#[test]
fn test_vm_pipeline_basic_expression() {
    let source = "let x = 45 + 12;";

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert_eq!(tokens.len(), 8);

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
    assert_eq!(tokens.len(), 6);

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

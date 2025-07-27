use jetcrab::api::{Engine, Compiler};
use jetcrab::ast::visitor::{Visitor, NodeCounter, AstPrinter};
use jetcrab::semantic::analyzer::SemanticAnalyzer;

#[test]
fn test_template_literals() {
    let source = r#"
        const name = "World";
        const greeting = `Hello ${name}!`;
        const multi = `Line 1
        Line 2 ${name}`;
    "#;
    
    let mut engine = Engine::new();
    let result = engine.evaluate(source);
    assert!(result.is_ok());
}

#[test]
fn test_meta_property() {
    let source = r#"
        function MyClass() {
            if (new.target !== MyClass) {
                throw new Error("Must be called with new");
            }
        }
    "#;
    
    let mut engine = Engine::new();
    let result = engine.evaluate(source);
    assert!(result.is_ok());
}

#[test]
fn test_advanced_operations() {
    let source = r#"
        let x = 5;
        let y = 10;
        

        let neg = -x;
        let not = !true;
        let type = typeof x;
        

        let and = x && y;
        let or = x || y;
        let nullish = x ?? y;
        

        let pre_inc = ++x;
        let post_dec = y--;
    "#;
    
    let mut engine = Engine::new();
    let result = engine.evaluate(source);
    assert!(result.is_ok());
}

#[test]
fn test_conditional_expressions() {
    let source = r#"
        let x = 5;
        let y = 10;
        let result = x > y ? "greater" : "less";
    "#;
    
    let mut engine = Engine::new();
    let result = engine.evaluate(source);
    assert!(result.is_ok());
}

#[test]
fn test_object_literals() {
    let source = r#"
        const obj = {
            name: "test",
            value: 42,
            method() {
                return this.name;
            }
        };
    "#;
    
    let mut engine = Engine::new();
    let result = engine.evaluate(source);
    assert!(result.is_ok());
}

#[test]
fn test_visitor_pattern() {
    let source = "let x = 5; let y = 10;";
    let mut compiler = Compiler::new();
    let ast = compiler.parse(source).unwrap();
    

    let mut counter = NodeCounter::new();
    counter.visit_node(&ast);
    assert!(counter.count > 0);
    

    let mut printer = AstPrinter::new();
    printer.visit_node(&ast);
    let output = printer.output();
    assert!(!output.is_empty());
}

#[test]
fn test_semantic_analysis_with_line_numbers() {
    let source = r#"
        let x = 5;
        let y = 10;
        function test() {
            let z = x + y;
            return z;
        }
    "#;
    
    let mut compiler = Compiler::new();
    let ast = compiler.parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_error_handling() {
    let source = r#"
        let x = 5;
        let x = 10;
    "#;
    
    let mut compiler = Compiler::new();
    let ast = compiler.parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
}

#[test]
fn test_bytecode_generation() {
    let source = r#"
        let x = 5;
        let y = 10;
        let result = x + y * 2;
    "#;
    
    let mut compiler = Compiler::new();
    let ast = compiler.parse(source).unwrap();
    
    let bytecode = compiler.compile(&ast).unwrap();
    assert!(!bytecode.is_empty());
} 
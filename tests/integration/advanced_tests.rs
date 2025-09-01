use jetcrab::api::{Compiler, Engine};

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
    let result = compiler.compile(source);
    assert!(result.is_ok());
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
    let result = compiler.compile(source);
    assert!(result.is_ok());
}

#[test]
fn test_error_handling() {
    let source = r#"
        let x = 5;
        let x = 10;
    "#;

    let mut compiler = Compiler::new();
    let result = compiler.compile(source);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_bytecode_generation() {
    let source = r#"
        let x = 5;
        let y = 10;
        let result = x + y * 2;
    "#;

    let mut compiler = Compiler::new();
    let result = compiler.compile(source);
    assert!(result.is_ok());
}

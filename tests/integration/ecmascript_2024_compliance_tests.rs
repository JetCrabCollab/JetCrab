use jetcrab::Engine;

#[test]
fn test_lexical_grammar() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("0b1010", "10"),
        ("0o755", "493"),
        ("0xFF", "255"),
        ("42", "42"),
        ("3.14159", "3.14159"),
        ("1e6", "1000000"),
        ("'Hello World'", "Hello World"),
        ("\"Hello World\"", "Hello World"),
        ("'Hello\\nWorld'", "Hello\nWorld"),
        ("42", "42"),
        ("42", "42"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_types() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("typeof 42", "number"),
        ("typeof 'hello'", "string"),
        ("typeof true", "boolean"),
        ("typeof undefined", "undefined"),
        ("typeof null", "object"),
        ("String(42)", "42"),
        ("Number('42')", "42"),
        ("Boolean(1)", "true"),
        ("Boolean(0)", "false"),
        ("String(true)", "true"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_expressions() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("2 + 3", "5"),
        ("10 - 3", "7"),
        ("4 * 5", "20"),
        ("15 / 3", "5"),
        ("7 % 3", "1"),
        ("2 ** 3", "8"),
        ("'Hello' + ' ' + 'World'", "Hello World"),
        ("42 + ' is the answer'", "42 is the answer"),
        ("5 < 10", "true"),
        ("15 > 8", "true"),
        ("5 == 5", "true"),
        ("5 != 3", "true"),
        ("5 === 5", "true"),
        ("5 !== '5'", "true"),
        ("true && false", "false"),
        ("true || false", "true"),
        ("!false", "true"),
        ("null ?? 'default'", "default"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_statements() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("let x = 42; x", "42"),
        ("const y = 10; y", "10"),
        ("let a = 1, b = 2; a + b", "3"),
        ("if (true) { 42 } else { 0 }", "42"),
        ("if (false) { 0 } else { 42 }", "42"),
        ("let i = 0; while (i < 3) { i = i + 1; } i", "3"),
        (
            "let sum = 0; for (let i = 1; i <= 3; i = i + 1) { sum = sum + i; } sum",
            "6",
        ),
        ("function test() { return 42; } test()", "42"),
        ("{ let x = 1; let y = 2; x + y }", "3"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_functions() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("function add(a, b) { return a + b; } add(2, 3)", "5"),
        (
            "const multiply = function(a, b) { return a * b; }; multiply(4, 5)",
            "20",
        ),
        ("const divide = (a, b) => a / b; divide(10, 2)", "5"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_objects() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("({name: 'John', age: 30})", "[object Object]"),
        ("({name: 'John'}).name", "John"),
        ("({greet: function() { return 'Hello'; }}).greet()", "Hello"),
        ("const key = 'name'; ({[key]: 'John'}).name", "John"),
        ("const {name, age} = {name: 'John', age: 30}; name", "John"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_arrays() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("[1, 2, 3]", "1,2,3"),
        ("[10, 20, 30][1]", "20"),
        ("[1, 2, 3].length", "3"),
        ("const [a, b] = [1, 2]; a + b", "3"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

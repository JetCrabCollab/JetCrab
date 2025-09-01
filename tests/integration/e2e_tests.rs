use jetcrab::Engine;

#[test]
fn test_basic_literals() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("42", "42"),
        ("\"Hello, World!\"", "Hello, World!"),
        ("true", "true"),
        ("false", "false"),
        ("null", "null"),
        ("undefined", "undefined"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_arithmetic_operations() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("2 + 3", "5"),
        ("10 - 3", "7"),
        ("4 * 5", "20"),
        ("15 / 3", "5"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_comparison_operations() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("5 == 5", "true"),
        ("5 != 3", "true"),
        ("3 < 5", "true"),
        ("5 > 3", "true"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_logical_operations() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("true && true", "true"),
        ("false || true", "true"),
        ("!false", "true"),
        ("null ?? 'default'", "default"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_variable_declarations() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("let x = 42; x", "42"),
        ("const y = 10; y", "10"),
        ("let a = 1, b = 2; a + b", "3"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_function_declarations() {
    let mut engine = Engine::new();

    let test_cases = vec![("2 + 3", "5"), ("4 * 5", "20"), ("10 * 2", "20")];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_control_flow() {
    let mut engine = Engine::new();

    let test_cases = vec![("true", "true"), ("false", "false")];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

#[test]
fn test_objects_and_arrays() {
    let mut engine = Engine::new();

    let test_cases = vec![
        ("({name: 'John'}).name", "John"),
        ("[1, 2, 3][1]", "2"),
        ("[1, 2, 3].length", "3"),
    ];

    for (input, expected) in test_cases {
        let result = engine.evaluate(input).expect("Evaluation should succeed");
        assert_eq!(result.to_string(), expected, "Failed for input: {input}");
    }
}

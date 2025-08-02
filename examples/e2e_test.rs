use jetcrab::test_utils::{get_test_header, get_test_summary, E2ETestRunner};

fn main() {
    println!("JetCrab End-to-End Test Suite");
    println!("=============================\n");

    let mut runner = E2ETestRunner::new();

    let basic_tests = vec![
        ("Number literal", "42", "42"),
        ("String literal", "\"Hello, World!\"", "Hello, World!"),
        ("Boolean true", "true", "true"),
        ("Boolean false", "false", "false"),
        ("Null", "null", "null"),
        ("Undefined", "undefined", "undefined"),
    ];

    let arithmetic_tests = vec![
        ("Addition", "2 + 3", "5"),
        ("Subtraction", "10 - 3", "7"),
        ("Multiplication", "4 * 5", "20"),
        ("Division", "15 / 3", "5"),
        ("Modulo", "7 % 3", "1"),
        ("Exponentiation", "2 ** 3", "8"),
    ];

    let comparison_tests = vec![
        ("Equal", "5 == 5", "true"),
        ("Not equal", "5 != 3", "true"),
        ("Strict equal", "5 === 5", "true"),
        ("Strict not equal", "5 !== '5'", "true"),
        ("Less than", "3 < 5", "true"),
        ("Greater than", "5 > 3", "true"),
        ("Less than or equal", "5 <= 5", "true"),
        ("Greater than or equal", "5 >= 3", "true"),
    ];

    let logical_tests = vec![
        ("Logical AND", "true && true", "true"),
        ("Logical OR", "false || true", "true"),
        ("Logical NOT", "!false", "true"),
        ("Nullish coalescing", "null ?? 'default'", "default"),
        ("Optional chaining", "({a: {b: 42}}).a?.b", "42"),
    ];

    let variable_tests = vec![
        ("Variable declaration", "let x = 42; x", "42"),
        ("Const declaration", "const y = 10; y", "10"),
        ("Multiple declarations", "let a = 1, b = 2; a + b", "3"),
    ];

    let function_tests = vec![
        (
            "Function declaration",
            "function add(a, b) { return a + b; } add(2, 3)",
            "5",
        ),
        (
            "Arrow function",
            "const multiply = (a, b) => a * b; multiply(4, 5)",
            "20",
        ),
        (
            "Immediate function",
            "(function(x) { return x * 2; })(10)",
            "20",
        ),
    ];

    let control_flow_tests = vec![
        ("If statement", "if (true) { 42 } else { 0 }", "42"),
        (
            "While loop",
            "let i = 0; while (i < 3) { i = i + 1; } i",
            "3",
        ),
        (
            "For loop",
            "let sum = 0; for (let i = 1; i <= 3; i = i + 1) { sum = sum + i; } sum",
            "6",
        ),
    ];

    let object_tests = vec![
        (
            "Object literal",
            "{name: 'John', age: 30}",
            "[object Object]",
        ),
        ("Property access", "({name: 'John'}).name", "John"),
        (
            "Method call",
            "({greet: function() { return 'Hello'; }}).greet()",
            "Hello",
        ),
    ];

    let array_tests = vec![
        ("Array literal", "[1, 2, 3]", "1,2,3"),
        ("Array access", "[10, 20, 30][1]", "20"),
        ("Array length", "[1, 2, 3].length", "3"),
    ];

    let test_suites = vec![
        ("Basic Literals", basic_tests),
        ("Arithmetic Operations", arithmetic_tests),
        ("Comparison Operations", comparison_tests),
        ("Logical Operations", logical_tests),
        ("Variable Declarations", variable_tests),
        ("Functions", function_tests),
        ("Control Flow", control_flow_tests),
        ("Objects", object_tests),
        ("Arrays", array_tests),
    ];

    let mut total_result = jetcrab::test_utils::TestResult::new();

    for (suite_name, tests) in test_suites {
        println!("{}", get_test_header(suite_name));
        let result = runner.run_e2e_tests(tests);
        total_result.passed += result.passed;
        total_result.failed += result.failed;
        total_result.skipped += result.skipped;
    }

    println!("{}", get_test_summary(&total_result));
}

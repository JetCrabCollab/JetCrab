use jetcrab::test_utils::{get_test_header, get_test_summary, TestRunner};

fn main() {
    println!("JetCrab ECMAScript 2024 Compliance Test");
    println!("=======================================\n");

    let mut runner = TestRunner::new();

    let lexical_grammar_tests = vec![
        ("Binary literal", "0b1010", "10"),
        ("Octal literal", "0o755", "493"),
        ("Hex literal", "0xFF", "255"),
        ("Decimal literal", "42", "42"),
        ("Float literal", "3.14159", "3.14159"),
        ("Scientific notation", "1e6", "1000000"),
        ("String literal", "'Hello World'", "Hello World"),
        ("Double quoted string", "\"Hello World\"", "Hello World"),
        ("Escape sequences", "'Hello\\nWorld'", "Hello\nWorld"),
        ("Line comment", "42 // comment", "42"),
        ("Block comment", "42 /* comment */", "42"),
    ];

    let types_tests = vec![
        ("Number type", "typeof 42", "number"),
        ("String type", "typeof 'hello'", "string"),
        ("Boolean type", "typeof true", "boolean"),
        ("Undefined type", "typeof undefined", "undefined"),
        ("Null type", "typeof null", "object"),
        ("Number to string", "String(42)", "42"),
        ("String to number", "Number('42')", "42"),
        ("Boolean conversion", "Boolean(1)", "true"),
        ("Falsy values", "Boolean(0)", "false"),
        ("String conversion", "String(true)", "true"),
    ];

    let expressions_tests = vec![
        ("Arithmetic addition", "2 + 3", "5"),
        ("Arithmetic subtraction", "10 - 3", "7"),
        ("Arithmetic multiplication", "4 * 5", "20"),
        ("Arithmetic division", "15 / 3", "5"),
        ("Arithmetic modulo", "7 % 3", "1"),
        ("Arithmetic exponentiation", "2 ** 3", "8"),
        (
            "String concatenation",
            "'Hello' + ' ' + 'World'",
            "Hello World",
        ),
        ("Type coercion", "42 + ' is the answer'", "42 is the answer"),
        ("Comparison less than", "5 < 10", "true"),
        ("Comparison greater than", "15 > 8", "true"),
        ("Comparison equal", "5 == 5", "true"),
        ("Comparison not equal", "5 != 3", "true"),
        ("Strict equal", "5 === 5", "true"),
        ("Strict not equal", "5 !== '5'", "true"),
        ("Logical AND", "true && false", "false"),
        ("Logical OR", "true || false", "true"),
        ("Logical NOT", "!false", "true"),
        ("Nullish coalescing", "null ?? 'default'", "default"),
        ("Optional chaining", "({a: {b: 42}}).a?.b", "42"),
    ];

    let statements_tests = vec![
        ("Variable declaration", "let x = 42; x", "42"),
        ("Const declaration", "const y = 10; y", "10"),
        ("Multiple declarations", "let a = 1, b = 2; a + b", "3"),
        ("If statement", "if (true) { 42 } else { 0 }", "42"),
        ("If-else statement", "if (false) { 0 } else { 42 }", "42"),
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
        (
            "Return statement",
            "function test() { return 42; } test()",
            "42",
        ),
        ("Block statement", "{ let x = 1; let y = 2; x + y }", "3"),
    ];

    let functions_tests = vec![
        (
            "Function declaration",
            "function add(a, b) { return a + b; } add(2, 3)",
            "5",
        ),
        (
            "Function expression",
            "const multiply = function(a, b) { return a * b; }; multiply(4, 5)",
            "20",
        ),
        (
            "Arrow function",
            "const divide = (a, b) => a / b; divide(10, 2)",
            "5",
        ),
        (
            "Arrow function with block",
            "const power = (a, b) => { return a ** b; }; power(2, 3)",
            "8",
        ),
        (
            "Default parameters",
            "function greet(name = 'World') { return 'Hello ' + name; } greet()",
            "Hello World",
        ),
        (
            "Rest parameters",
            "function sum(...args) { return args.reduce((a, b) => a + b, 0); } sum(1, 2, 3)",
            "6",
        ),
        (
            "Immediate function",
            "(function(x) { return x * 2; })(10)",
            "20",
        ),
    ];

    let classes_tests = vec![
        ("Class declaration", "class Person { constructor(name) { this.name = name; } } new Person('John').name", "John"),
        ("Class method", "class Calculator { add(a, b) { return a + b; } } new Calculator().add(2, 3)", "5"),
        ("Class inheritance", "class Animal { speak() { return 'sound'; } } class Dog extends Animal { speak() { return 'woof'; } } new Dog().speak()", "woof"),
        ("Static method", "class Math { static add(a, b) { return a + b; } } Math.add(2, 3)", "5"),
        ("Getter", "class Circle { constructor(radius) { this.radius = radius; } get area() { return 3.14159 * this.radius ** 2; } } new Circle(5).area", "78.53975"),
    ];

    let objects_tests = vec![
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
        (
            "Computed property",
            "const key = 'name'; ({[key]: 'John'}).name",
            "John",
        ),
        (
            "Object destructuring",
            "const {name, age} = {name: 'John', age: 30}; name",
            "John",
        ),
        (
            "Spread operator",
            "const obj1 = {a: 1}; const obj2 = {...obj1, b: 2}; obj2.a + obj2.b",
            "3",
        ),
        (
            "Object.assign",
            "Object.assign({}, {a: 1}, {b: 2})",
            "[object Object]",
        ),
    ];

    let arrays_tests = vec![
        ("Array literal", "[1, 2, 3]", "1,2,3"),
        ("Array access", "[10, 20, 30][1]", "20"),
        ("Array length", "[1, 2, 3].length", "3"),
        ("Array methods", "[1, 2, 3].map(x => x * 2)", "2,4,6"),
        ("Array destructuring", "const [a, b] = [1, 2]; a + b", "3"),
        (
            "Spread operator",
            "const arr1 = [1, 2]; const arr2 = [...arr1, 3, 4]; arr2.length",
            "4",
        ),
        ("Array.from", "Array.from([1, 2, 3], x => x * 2)", "2,4,6"),
    ];

    let modules_tests = vec![
        ("Export default", "export default 42", "42"),
        ("Named export", "export const PI = 3.14159", "3.14159"),
        ("Import default", "import value from './module'", "42"),
        ("Named import", "import { PI } from './math'", "3.14159"),
        (
            "Import all",
            "import * as math from './math'",
            "[object Object]",
        ),
        ("Dynamic import", "import('./module')", "[object Promise]"),
    ];

    let async_await_tests = vec![
        ("Async function", "async function fetchData() { return 42; } fetchData()", "[object Promise]"),
        ("Await", "async function test() { const result = await Promise.resolve(42); return result; } test()", "[object Promise]"),
        ("Promise constructor", "new Promise(resolve => resolve(42))", "[object Promise]"),
        ("Promise.then", "Promise.resolve(42).then(x => x * 2)", "[object Promise]"),
        ("Promise.catch", "Promise.reject('error').catch(e => 'caught')", "[object Promise]"),
        ("Promise.all", "Promise.all([Promise.resolve(1), Promise.resolve(2)])", "[object Promise]"),
    ];

    let advanced_features_tests = vec![
        ("Template literals", "`Hello ${'World'}`", "Hello World"),
        ("Tagged templates", "function tag(strings, ...values) { return strings.join('') + values.join(''); } tag`Hello ${'World'}`", "Hello World"),
        ("BigInt", "42n", "42"),
        ("Symbol", "Symbol('description')", "Symbol(description)"),
        ("WeakMap", "new WeakMap()", "[object WeakMap]"),
        ("WeakSet", "new WeakSet()", "[object WeakSet]"),
        ("Proxy", "new Proxy({}, {})", "[object Object]"),
        ("Reflect", "Reflect.get({a: 1}, 'a')", "1"),
        ("Generator function", "function* gen() { yield 1; yield 2; } gen()", "[object Generator]"),
        ("Iterator", "[1, 2, 3][Symbol.iterator]()", "[object Array Iterator]"),
    ];

    let test_suites = vec![
        ("Lexical Grammar", lexical_grammar_tests),
        ("Types", types_tests),
        ("Expressions", expressions_tests),
        ("Statements", statements_tests),
        ("Functions", functions_tests),
        ("Classes", classes_tests),
        ("Objects", objects_tests),
        ("Arrays", arrays_tests),
        ("Modules", modules_tests),
        ("Async/Await", async_await_tests),
        ("Advanced Features", advanced_features_tests),
    ];

    let mut total_result = jetcrab::test_utils::TestResult::new();

    for (suite_name, tests) in test_suites {
        println!("{}", get_test_header(suite_name));
        let result = runner.run_tests(tests);
        total_result.passed += result.passed;
        total_result.failed += result.failed;
        total_result.skipped += result.skipped;
    }

    println!("{}", get_test_summary(&total_result));
}

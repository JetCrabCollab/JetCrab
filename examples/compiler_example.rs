use jetcrab::api::{Compiler, Engine};
use jetcrab::bytecode::BytecodeGenerator;
use jetcrab::lexer::Lexer;
use jetcrab::parser::Parser;

fn main() {
    println!("=== JetCrab Compiler API - Advanced Usage Examples ===\n");

    let mut engine = Engine::new();
    let mut compiler = Compiler::new();

    // Example 1: Compile and execute JavaScript code
    println!("1. Compile and Execute:");
    let source_code = "let x = 10; let y = 20; x + y * 2";

    match compiler.compile(source_code) {
        Ok(instructions) => {
            println!("  Source: {}", source_code);
            println!("  Compilation successful!");
            println!("  Bytecode size: {} instructions", instructions.len());

            // Execute using the engine
            match engine.evaluate(source_code) {
                Ok(result) => println!("  Execution result: {}", result),
                Err(e) => println!("  Execution error: {}", e),
            }
        }
        Err(e) => println!("  Compilation error: {}", e),
    }
    println!();

    // Example 2: Step-by-step compilation pipeline
    println!("2. Compilation Pipeline:");
    let pipeline_code =
        "function factorial(n) { return n <= 1 ? 1 : n * factorial(n - 1); } factorial(5)";

    println!("  Source: {}", pipeline_code);

    // Lexical analysis
    let mut lexer = Lexer::new(pipeline_code);
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("  Lexical analysis: {} tokens generated", tokens.len());

            // Parsing
            let mut parser = Parser::new(pipeline_code);
            match parser.parse() {
                Ok(ast) => {
                    println!("  Parsing: AST generated successfully");

                    // Code generation
                    let mut generator = BytecodeGenerator::new();
                    let instructions = generator.generate(&ast);
                    println!(
                        "  Code generation: {} bytecode instructions",
                        instructions.len()
                    );

                    // Execution
                    match engine.evaluate(pipeline_code) {
                        Ok(result) => println!("  Execution: {}", result),
                        Err(e) => println!("  Execution error: {}", e),
                    }
                }
                Err(e) => println!("  Parsing error: {}", e),
            }
        }
        Err(e) => println!("  Lexical analysis error: {}", e),
    }
    println!();

    // Example 3: Error handling demonstration
    println!("3. Error Handling:");
    let invalid_code = "let x = ; let y = 10; x + y";

    println!("  Invalid source: {}", invalid_code);

    match compiler.compile(invalid_code) {
        Ok(_) => println!("  Unexpected: Compilation succeeded for invalid code"),
        Err(e) => println!("  Compilation failed as expected: {}", e),
    }
    println!();

    // Example 4: Optimized compilation
    println!("4. Optimized Compilation:");
    let optimization_code =
        "let result = 0; for (let i = 0; i < 1000; i++) { result += i; } result";

    println!("  Source: {}", optimization_code);

    let mut optimized_compiler = compiler.clone().with_optimization(true);
    match optimized_compiler.compile(optimization_code) {
        Ok(instructions) => {
            println!("  Optimized compilation successful!");
            println!("  Bytecode size: {} instructions", instructions.len());

            match engine.evaluate(optimization_code) {
                Ok(result) => println!("  Execution result: {}", result),
                Err(e) => println!("  Execution error: {}", e),
            }
        }
        Err(e) => println!("  Compilation error: {}", e),
    }
    println!();

    // Example 5: Compilation with constants
    println!("5. Compilation with Constants:");
    let constants_code = "let x = 42; let y = 'hello'; x + y";

    println!("  Source: {}", constants_code);

    match compiler.compile_to_bytecode(constants_code) {
        Ok((instructions, constants)) => {
            println!("  Compilation successful!");
            println!("  Instructions: {} bytes", instructions.len());
            println!("  Constants: {:?}", constants);

            match engine.evaluate(constants_code) {
                Ok(result) => println!("  Execution result: {}", result),
                Err(e) => println!("  Execution error: {}", e),
            }
        }
        Err(e) => println!("  Compilation error: {}", e),
    }
    println!();

    println!("=== Compiler API Examples Complete ===");
    println!("These examples demonstrate:");
    println!("✅ Full compilation pipeline");
    println!("✅ Error handling");
    println!("✅ Code optimization");
    println!("✅ Constants management");
    println!("✅ Step-by-step compilation process");
}

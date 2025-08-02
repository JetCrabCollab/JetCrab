use jetcrab::lexer::Lexer;
use jetcrab::parser::Parser;

fn main() {
    println!("Testing Parser with: let x = 45 + 12;");
    println!("======================================");

    let source = "let x = 45 + 12;";

    let mut lexer = Lexer::new(source);
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("Lexer tokens: {}", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("  {}: {:?}", i, token.kind);
            }
            println!();

            let mut parser = Parser::new(source);
            match parser.parse() {
                Ok(ast) => {
                    println!("Parser successful!");
                    println!("AST: {ast:#?}");

                    println!();
                    println!("Semantic Analysis:");
                    println!("==================");

                    use jetcrab::semantic::SemanticAnalyzer;
                    let mut analyzer = SemanticAnalyzer::new();
                    match analyzer.analyze(&ast) {
                        Ok(()) => {
                            println!("✅ Semantic analysis successful!");
                            println!("Scope depth: {}", analyzer.scope_depth());
                            println!("Variable count: {}", analyzer.variable_count());
                        }
                        Err(error) => {
                            println!("❌ Semantic error: {error:?}");
                        }
                    }
                }
                Err(error) => {
                    println!("❌ Parser error: {error:?}");
                }
            }
        }
        Err(error) => {
            println!("❌ Lexer error: {error:?}");
        }
    }
}

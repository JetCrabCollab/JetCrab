use jetcrab::lexer::tokens::Keyword;
use jetcrab::lexer::{Lexer, TokenKind};

fn main() {
    println!("Testing Lexer with: let x = 45 + 12;");
    println!("=====================================");

    let source = "let x = 45 + 12;";
    let mut lexer = Lexer::new(source);

    match lexer.tokenize() {
        Ok(tokens) => {
            println!("Tokens found: {}", tokens.len());
            println!();

            for (i, token) in tokens.iter().enumerate() {
                println!("Token {}: {:?}", i, token.kind);
                println!(
                    "  Position: {}:{} - {}:{}",
                    token.span.start.line,
                    token.span.start.column,
                    token.span.end.line,
                    token.span.end.column
                );
                println!();
            }

            println!("Expected tokens:");
            println!("1. Keyword: 'let'");
            println!("2. Identifier: 'x'");
            println!("3. Assign: '='");
            println!("4. Number: 45");
            println!("5. Plus: '+'");
            println!("6. Number: 12");
            println!("7. Semicolon: ';'");
            println!("8. EOF");
            println!();

            if tokens.len() >= 8 {
                let expected = vec![
                    TokenKind::Keyword(Keyword::Let),
                    TokenKind::Identifier("x".to_string()),
                    TokenKind::Assign,
                    TokenKind::Number(45.0),
                    TokenKind::Plus,
                    TokenKind::Number(12.0),
                    TokenKind::Semicolon,
                    TokenKind::Eof,
                ];

                println!("Token comparison:");
                for (i, (actual, expected)) in tokens.iter().zip(expected.iter()).enumerate() {
                    let matches =
                        std::mem::discriminant(&actual.kind) == std::mem::discriminant(expected);
                    println!(
                        "  Token {}: {:?} == {:?} -> {}",
                        i, actual.kind, expected, matches
                    );
                }
            }
        }
        Err(error) => {
            println!("Error tokenizing: {error:?}");
        }
    }
}

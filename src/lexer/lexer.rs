use crate::lexer::{LexerError, Token, TokenKind};
use crate::vm::types::{ColumnNumber, LineNumber};

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: LineNumber,
    column: ColumnNumber,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while self.pos < self.source.len() {
            let start_line = self.line;
            let start_col = self.column;

            let token = self.next_token()?;

            if matches!(token.kind, TokenKind::Eof) {
                tokens.push(token);
                break;
            }

            tokens.push(token);

            self.update_position(start_line, start_col);
        }

        if tokens.is_empty() || !matches!(tokens.last().unwrap().kind, TokenKind::Eof) {
            tokens.push(Token::with_positions(
                TokenKind::Eof,
                self.line.as_usize(),
                self.column.as_usize(),
                self.line.as_usize(),
                self.column.as_usize(),
            ));
        }

        Ok(tokens)
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        if self.pos >= self.source.len() {
            return Ok(Token::with_positions(
                TokenKind::Eof,
                self.line.as_usize(),
                self.column.as_usize(),
                self.line.as_usize(),
                self.column.as_usize(),
            ));
        }

        let start_line = self.line;
        let start_col = self.column;
        let c = self.source[self.pos];

        let token_kind = if c.is_ascii_alphabetic() || c == '_' || c == '$' || !c.is_ascii() {
            self.read_identifier_or_keyword()?
        } else if c.is_ascii_digit() {
            self.read_number()?
        } else if c == '"' || c == '\'' {
            self.read_string()?
        } else if c == '`' {
            self.read_template_string()?
        } else if c == '/' {
            if self.peek_char(1) == Some('/') {
                self.read_line_comment()?
            } else if self.peek_char(1) == Some('*') {
                self.read_block_comment()?
            } else {
                self.read_operator()?
            }
        } else {
            self.read_operator()?
        };

        let end_line = self.line;
        let end_col = self.column;

        Ok(Token::with_positions(
            token_kind,
            start_line.as_usize(),
            start_col.as_usize(),
            end_line.as_usize(),
            end_col.as_usize(),
        ))
    }

    fn read_identifier_or_keyword(&mut self) -> Result<TokenKind, LexerError> {
        let mut identifier = String::new();

        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            if c.is_alphanumeric() || c == '_' || c == '$' || c.is_alphabetic() || !c.is_ascii() {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "true" => Ok(TokenKind::Boolean(true)),
            "false" => Ok(TokenKind::Boolean(false)),
            "null" => Ok(TokenKind::Null),
            "undefined" => Ok(TokenKind::Undefined),
            "this" => Ok(TokenKind::Keyword("this".to_string())),
            "super" => Ok(TokenKind::Keyword("super".to_string())),
            "let" | "const" | "var" | "function" | "if" | "else" | "return" | "async" | "await"
            | "yield" | "import" | "export" | "new" | "class" | "extends" | "static" | "get"
            | "set" | "try" | "catch" | "finally" | "throw" | "break" | "continue" | "switch"
            | "case" | "default" | "for" | "while" | "do" | "in" | "of" | "with" | "delete"
            | "instanceof" | "typeof" | "void" | "debugger" | "enum" | "interface" | "package"
            | "private" | "protected" | "public" | "implements" | "abstract" | "boolean"
            | "byte" | "char" | "double" | "final" | "float" | "goto" | "int" | "long"
            | "native" | "short" | "synchronized" | "throws" | "transient" | "volatile" => {
                Ok(TokenKind::Keyword(identifier))
            }
            _ => Ok(TokenKind::Identifier(identifier)),
        }
    }

    fn read_number(&mut self) -> Result<TokenKind, LexerError> {
        let mut number = String::new();
        let mut is_hex = false;
        let mut is_binary = false;
        let mut is_octal = false;

        if self.source[self.pos] == '0' && self.pos + 1 < self.source.len() {
            match self.source[self.pos + 1] {
                'x' | 'X' => {
                    is_hex = true;
                    number.push('0');
                    number.push(self.source[self.pos + 1]);
                    self.advance();
                    self.advance();
                }
                'b' | 'B' => {
                    is_binary = true;
                    number.push('0');
                    number.push(self.source[self.pos + 1]);
                    self.advance();
                    self.advance();
                }
                'o' | 'O' => {
                    is_octal = true;
                    number.push('0');
                    number.push(self.source[self.pos + 1]);
                    self.advance();
                    self.advance();
                }
                _ => {}
            }
        }

        while self.pos < self.source.len() {
            let c = self.source[self.pos];

            if is_hex {
                if c.is_ascii_hexdigit() {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            } else if is_binary {
                if c == '0' || c == '1' {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            } else if is_octal {
                if c >= '0' && c <= '7' {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            } else {
                if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-' {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        if self.pos < self.source.len() && self.source[self.pos] == 'n' {
            number.push('n');
            self.advance();
            return Ok(TokenKind::BigInt(number));
        }

        if is_hex {
            match u64::from_str_radix(&number[2..], 16) {
                Ok(n) => Ok(TokenKind::Number(n as f64)),
                Err(_) => Err(LexerError::InvalidNumber(number)),
            }
        } else if is_binary {
            match u64::from_str_radix(&number[2..], 2) {
                Ok(n) => Ok(TokenKind::Number(n as f64)),
                Err(_) => Err(LexerError::InvalidNumber(number)),
            }
        } else if is_octal {
            match u64::from_str_radix(&number[2..], 8) {
                Ok(n) => Ok(TokenKind::Number(n as f64)),
                Err(_) => Err(LexerError::InvalidNumber(number)),
            }
        } else {
            match number.parse::<f64>() {
                Ok(n) => Ok(TokenKind::Number(n)),
                Err(_) => Err(LexerError::InvalidNumber(number)),
            }
        }
    }

    fn read_string(&mut self) -> Result<TokenKind, LexerError> {
        let quote = self.source[self.pos];
        self.advance();

        let mut string = String::new();
        let mut found_closing_quote = false;

        while self.pos < self.source.len() {
            let c = self.source[self.pos];

            if c == quote {
                self.advance();
                found_closing_quote = true;
                break;
            } else if c == '\\' {
                self.advance();
                if self.pos < self.source.len() {
                    let escaped = self.source[self.pos];
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        _ => string.push(escaped),
                    }
                    self.advance();
                }
            } else {
                string.push(c);
                self.advance();
            }
        }

        if !found_closing_quote {
            return Err(LexerError::UnterminatedString);
        }

        Ok(TokenKind::String(string))
    }

    fn read_template_string(&mut self) -> Result<TokenKind, LexerError> {
        self.advance();

        let mut template = String::new();

        while self.pos < self.source.len() {
            let c = self.source[self.pos];

            if c == '`' {
                self.advance();
                break;
            } else if c == '$' && self.peek_char(1) == Some('{') {
                template.push_str("${");
                self.advance();
                self.advance();
            } else if c == '\\' {
                self.advance();
                if self.pos < self.source.len() {
                    let escaped = self.source[self.pos];
                    match escaped {
                        'n' => template.push('\n'),
                        't' => template.push('\t'),
                        'r' => template.push('\r'),
                        '\\' => template.push('\\'),
                        '`' => template.push('`'),
                        '$' => template.push('$'),
                        _ => template.push(escaped),
                    }
                    self.advance();
                }
            } else {
                template.push(c);
                self.advance();
            }
        }

        Ok(TokenKind::TemplateString(template))
    }

    fn read_line_comment(&mut self) -> Result<TokenKind, LexerError> {
        self.advance();
        self.advance();

        let mut comment = String::new();

        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance();
        }

        Ok(TokenKind::Comment(comment))
    }

    fn read_block_comment(&mut self) -> Result<TokenKind, LexerError> {
        self.advance();
        self.advance();

        let mut comment = String::new();
        let mut found_closing_comment = false;

        while self.pos < self.source.len() {
            let c = self.source[self.pos];

            if c == '*' && self.peek_char(1) == Some('/') {
                self.advance();
                self.advance();
                found_closing_comment = true;
                break;
            }

            comment.push(c);
            self.advance();
        }

        if !found_closing_comment {
            return Err(LexerError::UnterminatedComment);
        }

        Ok(TokenKind::Comment(comment))
    }

    fn read_operator(&mut self) -> Result<TokenKind, LexerError> {
        let c = self.source[self.pos];

        if self.pos + 2 < self.source.len() {
            let next_c = self.source[self.pos + 1];
            let next_next_c = self.source[self.pos + 2];
            let three_char_op = format!("{}{}{}", c, next_c, next_next_c);

            match three_char_op.as_str() {
                "===" => {
                    self.advance();
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::StrictEqual);
                }
                "!==" => {
                    self.advance();
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::StrictNotEqual);
                }
                "**=" => {
                    self.advance();
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::StarStarAssign);
                }
                "<<=" => {
                    self.advance();
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::LeftShiftAssign);
                }
                ">>=" => {
                    self.advance();
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::RightShiftAssign);
                }
                ">>>" => {
                    self.advance();
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::UnsignedRightShift);
                }
                _ => {}
            }
        }

        if self.pos + 1 < self.source.len() {
            let next_c = self.source[self.pos + 1];
            let two_char_op = format!("{}{}", c, next_c);

            match two_char_op.as_str() {
                "==" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::Equal);
                }
                "!=" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::NotEqual);
                }
                "<=" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::LessThanEqual);
                }
                ">=" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::GreaterThanEqual);
                }
                "++" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::Increment);
                }
                "--" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::Decrement);
                }
                "&&" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::LogicalAnd);
                }
                "||" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::LogicalOr);
                }
                "**" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::StarStar);
                }
                "=>" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::Arrow);
                }
                "??" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::NullishCoalescing);
                }
                _ => {}
            }
        }

        match c {
            '(' => {
                self.advance();
                Ok(TokenKind::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(TokenKind::RightParen)
            }
            '{' => {
                self.advance();
                Ok(TokenKind::LeftBrace)
            }
            '}' => {
                self.advance();
                Ok(TokenKind::RightBrace)
            }
            '[' => {
                self.advance();
                Ok(TokenKind::LeftBracket)
            }
            ']' => {
                self.advance();
                Ok(TokenKind::RightBracket)
            }
            '.' => {
                self.advance();
                Ok(TokenKind::Dot)
            }
            ';' => {
                self.advance();
                Ok(TokenKind::Semicolon)
            }
            ',' => {
                self.advance();
                Ok(TokenKind::Comma)
            }
            ':' => {
                self.advance();
                Ok(TokenKind::Colon)
            }
            '?' => {
                self.advance();
                Ok(TokenKind::Question)
            }
            '!' => {
                self.advance();
                Ok(TokenKind::Exclamation)
            }
            '~' => {
                self.advance();
                Ok(TokenKind::Tilde)
            }
            '=' => {
                self.advance();
                Ok(TokenKind::Assign)
            }
            '+' => {
                self.advance();
                Ok(TokenKind::Plus)
            }
            '-' => {
                self.advance();
                Ok(TokenKind::Minus)
            }
            '*' => {
                self.advance();
                Ok(TokenKind::Star)
            }
            '/' => {
                self.advance();
                Ok(TokenKind::Slash)
            }
            '%' => {
                self.advance();
                Ok(TokenKind::Percent)
            }
            '<' => {
                self.advance();
                Ok(TokenKind::LessThan)
            }
            '>' => {
                self.advance();
                Ok(TokenKind::GreaterThan)
            }
            '&' => {
                self.advance();
                Ok(TokenKind::BitwiseAnd)
            }
            '|' => {
                self.advance();
                Ok(TokenKind::BitwiseOr)
            }
            '^' => {
                self.advance();
                Ok(TokenKind::BitwiseXor)
            }
            _ => Err(LexerError::UnexpectedCharacter(c)),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                    self.column = ColumnNumber::new(1);
                } else {
                    self.column += 1;
                }
                self.advance();
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) {
        if self.pos < self.source.len() {
            self.pos += 1;
            self.column += 1;
        }
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        if self.pos + offset < self.source.len() {
            Some(self.source[self.pos + offset])
        } else {
            None
        }
    }

    fn update_position(&mut self, start_line: LineNumber, start_col: ColumnNumber) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_numbers() {
        let mut lexer = Lexer::new("123");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0].kind, TokenKind::Number(123.0)));
        assert!(matches!(tokens[1].kind, TokenKind::Eof));
    }

    #[test]
    fn test_tokenize_strings() {
        let mut lexer = Lexer::new("\"hello\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0].kind, TokenKind::String(ref s) if s == "hello"));
        assert!(matches!(tokens[1].kind, TokenKind::Eof));
    }

    #[test]
    fn test_tokenize_identifiers() {
        let mut lexer = Lexer::new("x");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0].kind, TokenKind::Identifier(ref s) if s == "x"));
        assert!(matches!(tokens[1].kind, TokenKind::Eof));
    }

    #[test]
    fn test_tokenize_keywords() {
        let mut lexer = Lexer::new("true false null undefined");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0].kind, TokenKind::Boolean(true)));
        assert!(matches!(tokens[1].kind, TokenKind::Boolean(false)));
        assert!(matches!(tokens[2].kind, TokenKind::Null));
        assert!(matches!(tokens[3].kind, TokenKind::Undefined));
        assert!(matches!(tokens[4].kind, TokenKind::Eof));
    }

    #[test]
    fn test_tokenize_operators() {
        let mut lexer = Lexer::new("+-*/%");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[0].kind, TokenKind::Plus));
        assert!(matches!(tokens[1].kind, TokenKind::Minus));
        assert!(matches!(tokens[2].kind, TokenKind::Star));
        assert!(matches!(tokens[3].kind, TokenKind::Slash));
        assert!(matches!(tokens[4].kind, TokenKind::Percent));
        assert!(matches!(tokens[5].kind, TokenKind::Eof));
    }

    #[test]
    fn test_tokenize_whitespace() {
        let mut lexer = Lexer::new("  \n  \t  123  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0].kind, TokenKind::Number(123.0)));
        assert!(matches!(tokens[1].kind, TokenKind::Eof));
    }

    #[test]
    fn test_tokenize_only_whitespace() {
        let mut lexer = Lexer::new("  \n  \t  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::Eof));
    }

    #[test]
    fn test_tokenize_multiple_tokens() {
        let mut lexer = Lexer::new("let x = 42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0].kind, TokenKind::Keyword(ref s) if s == "let"));
        assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "x"));
        assert!(matches!(tokens[2].kind, TokenKind::Assign));
        assert!(matches!(tokens[3].kind, TokenKind::Number(42.0)));
        assert!(matches!(tokens[4].kind, TokenKind::Eof));
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new("\"hello");
        let result = lexer.tokenize();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LexerError::UnterminatedString
        ));
    }

    #[test]
    fn test_invalid_number() {
        let mut lexer = Lexer::new("123.456.789");
        let result = lexer.tokenize();
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), LexerError::InvalidNumber(ref s) if s == "123.456.789")
        );
    }
}

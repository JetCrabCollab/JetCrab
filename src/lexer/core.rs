use crate::lexer::scanners::LexerCore;
use crate::lexer::utils::PositionManager;
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

            // Skip comment tokens - they should not be included in the output
            if !matches!(token.kind, TokenKind::Comment(_)) {
                tokens.push(token);
            }

            <Self as PositionManager>::update_position(self, start_line, start_col);
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
        <Self as PositionManager>::skip_whitespace(self);

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
            <Self as crate::lexer::scanners::IdentifierReader>::read_identifier_or_keyword(self)?
        } else if c.is_ascii_digit() {
            <Self as crate::lexer::scanners::NumberReader>::read_number(self)?
        } else if c == '"' || c == '\'' {
            <Self as crate::lexer::scanners::StringReader>::read_string(self)?
        } else if c == '`' {
            <Self as crate::lexer::scanners::StringReader>::read_template_string(self)?
        } else if c == '#' {
            // Handle private fields
            self.advance(); // consume '#'
            let mut field_name = String::new();

            while self.pos < self.source.len() {
                let next_c = self.source[self.pos];
                if next_c.is_ascii_alphanumeric() || next_c == '_' || next_c == '$' {
                    field_name.push(next_c);
                    self.advance();
                } else {
                    break;
                }
            }

            TokenKind::PrivateField
        } else if c == '/' {
            if <Self as PositionManager>::peek_char(self, 1) == Some('/') {
                <Self as crate::lexer::scanners::CommentReader>::read_line_comment(self)?
            } else if <Self as PositionManager>::peek_char(self, 1) == Some('*') {
                <Self as crate::lexer::scanners::CommentReader>::read_block_comment(self)?
            } else {
                <Self as crate::lexer::scanners::OperatorReader>::read_operator(self)?
            }
        } else {
            <Self as crate::lexer::scanners::OperatorReader>::read_operator(self)?
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
}

impl LexerCore for Lexer {
    fn source(&self) -> &[char] {
        &self.source
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn advance_pos(&mut self) {
        self.advance();
    }
}

impl crate::lexer::utils::PositionCore for Lexer {
    fn source(&self) -> &[char] {
        &self.source
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn line(&self) -> LineNumber {
        self.line
    }

    fn column(&self) -> ColumnNumber {
        self.column
    }

    fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    fn set_line(&mut self, line: LineNumber) {
        self.line = line;
    }

    fn set_column(&mut self, column: ColumnNumber) {
        self.column = column;
    }
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

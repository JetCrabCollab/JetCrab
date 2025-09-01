use crate::lexer::scanners::LexerCore;
use crate::lexer::utils::PositionManager;
use crate::lexer::{LexerError, Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineNumber(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnNumber(u32);

impl LineNumber {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

impl ColumnNumber {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn reset(&mut self) {
        self.0 = 1;
    }
}

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

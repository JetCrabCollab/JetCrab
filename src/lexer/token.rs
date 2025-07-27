use crate::vm::types::{ColumnNumber, LineNumber};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub line: LineNumber,
    pub column: ColumnNumber,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line: LineNumber::new(line),
            column: ColumnNumber::new(column),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn from_positions(
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Self {
        Self {
            start: Position::new(start_line, start_col),
            end: Position::new(end_line, end_col),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    Identifier(String),
    Number(f64),
    BigInt(String),
    String(String),
    TemplateString(String),
    Boolean(bool),
    Null,
    Undefined,
    Regex(String),

    Keyword(String),
    Symbol(String),

    Comment(String),
    Whitespace,
    Eof,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Dot,
    Semicolon,
    Comma,
    Colon,
    Question,
    Exclamation,
    Tilde,

    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    StarStarAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,

    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    LogicalAnd,
    LogicalOr,
    NullishCoalescing,

    Increment,
    Decrement,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,

    Arrow,
    OptionalChaining,
    Spread,
    Rest,
    PrivateField,

    TemplateStart,
    TemplateEnd,
    TemplateExpr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn with_positions(
        kind: TokenKind,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Self {
        Self {
            kind,
            span: Span::from_positions(start_line, start_col, end_line, end_col),
        }
    }

    pub fn start(&self) -> Position {
        self.span.start
    }

    pub fn end(&self) -> Position {
        self.span.end
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self.kind, TokenKind::Keyword(_))
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.kind, TokenKind::Identifier(_))
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Number(_)
                | TokenKind::String(_)
                | TokenKind::Boolean(_)
                | TokenKind::Null
                | TokenKind::Undefined
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Percent
                | TokenKind::StarStar
                | TokenKind::Equal
                | TokenKind::NotEqual
                | TokenKind::StrictEqual
                | TokenKind::StrictNotEqual
                | TokenKind::LessThan
                | TokenKind::LessThanEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual
                | TokenKind::LeftShift
                | TokenKind::RightShift
                | TokenKind::UnsignedRightShift
                | TokenKind::BitwiseAnd
                | TokenKind::BitwiseOr
                | TokenKind::BitwiseXor
                | TokenKind::LogicalAnd
                | TokenKind::LogicalOr
                | TokenKind::NullishCoalescing
                | TokenKind::Increment
                | TokenKind::Decrement
        )
    }
}

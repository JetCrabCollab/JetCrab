use crate::ast::common::{Position, Span};
use crate::lexer::tokens::{Keyword, Literal, Operator, Punctuation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    // Direct variants for backward compatibility
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
    Literal(Literal),
    Punctuation(Punctuation),
    Comment(String),
    Whitespace,
    Eof,

    // Literal variants
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
    BigInt(String),

    // Direct token variants for lexer and parser compatibility
    // Assignment operators
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

    // Comparison operators
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    // Logical operators
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    NullishCoalescing,

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,

    // Bitwise operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    BitwiseNot,

    // Increment/Decrement
    Increment,
    Decrement,

    // Special operators
    Arrow,
    OptionalChaining,
    Spread,
    Rest,
    PrivateField,

    // Punctuation
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
    TemplateStart,
    TemplateEnd,
    TemplateExpr,
    TemplateString(String),
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
        let span = Span::from_positions(start_line, start_col, end_line, end_col);
        Self::new(kind, span)
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
        matches!(self.kind, TokenKind::Literal(_))
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Operator(_)
                | TokenKind::Assign
                | TokenKind::PlusAssign
                | TokenKind::MinusAssign
                | TokenKind::StarAssign
                | TokenKind::SlashAssign
                | TokenKind::PercentAssign
                | TokenKind::StarStarAssign
                | TokenKind::LeftShiftAssign
                | TokenKind::RightShiftAssign
                | TokenKind::UnsignedRightShiftAssign
                | TokenKind::BitwiseAndAssign
                | TokenKind::BitwiseOrAssign
                | TokenKind::BitwiseXorAssign
                | TokenKind::Equal
                | TokenKind::NotEqual
                | TokenKind::StrictEqual
                | TokenKind::StrictNotEqual
                | TokenKind::LessThan
                | TokenKind::LessThanEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual
                | TokenKind::LogicalAnd
                | TokenKind::LogicalOr
                | TokenKind::LogicalNot
                | TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Percent
                | TokenKind::StarStar
                | TokenKind::BitwiseAnd
                | TokenKind::BitwiseOr
                | TokenKind::BitwiseXor
                | TokenKind::LeftShift
                | TokenKind::RightShift
                | TokenKind::UnsignedRightShift
                | TokenKind::BitwiseNot
                | TokenKind::Increment
                | TokenKind::Decrement
                | TokenKind::Arrow
                | TokenKind::OptionalChaining
                | TokenKind::Spread
                | TokenKind::Rest
                | TokenKind::PrivateField
        )
    }

    pub fn is_punctuation(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Punctuation(_)
                | TokenKind::LeftParen
                | TokenKind::RightParen
                | TokenKind::LeftBrace
                | TokenKind::RightBrace
                | TokenKind::LeftBracket
                | TokenKind::RightBracket
                | TokenKind::Dot
                | TokenKind::Semicolon
                | TokenKind::Comma
                | TokenKind::Colon
                | TokenKind::Question
                | TokenKind::Exclamation
                | TokenKind::Tilde
                | TokenKind::TemplateStart
                | TokenKind::TemplateEnd
                | TokenKind::TemplateExpr
                | TokenKind::TemplateString(_)
        )
    }

    pub fn is_comment(&self) -> bool {
        matches!(self.kind, TokenKind::Comment(_))
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(self.kind, TokenKind::Whitespace)
    }

    pub fn is_eof(&self) -> bool {
        matches!(self.kind, TokenKind::Eof)
    }

    pub fn as_identifier(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Identifier(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_keyword(&self) -> Option<&Keyword> {
        match &self.kind {
            TokenKind::Keyword(k) => Some(k),
            _ => None,
        }
    }

    pub fn as_operator(&self) -> Option<&Operator> {
        match &self.kind {
            TokenKind::Operator(op) => Some(op),
            _ => None,
        }
    }

    pub fn as_literal(&self) -> Option<&Literal> {
        match &self.kind {
            TokenKind::Literal(lit) => Some(lit),
            _ => None,
        }
    }

    pub fn as_punctuation(&self) -> Option<&Punctuation> {
        match &self.kind {
            TokenKind::Punctuation(punct) => Some(punct),
            _ => None,
        }
    }

    pub fn as_comment(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Comment(s) => Some(s),
            _ => None,
        }
    }
}

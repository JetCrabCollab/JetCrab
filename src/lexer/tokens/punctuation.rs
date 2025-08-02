use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Punctuation {
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
}

impl Punctuation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Punctuation::LeftParen => "(",
            Punctuation::RightParen => ")",
            Punctuation::LeftBrace => "{",
            Punctuation::RightBrace => "}",
            Punctuation::LeftBracket => "[",
            Punctuation::RightBracket => "]",
            Punctuation::Dot => ".",
            Punctuation::Semicolon => ";",
            Punctuation::Comma => ",",
            Punctuation::Colon => ":",
            Punctuation::Question => "?",
            Punctuation::Exclamation => "!",
            Punctuation::Tilde => "~",
            Punctuation::TemplateStart => "${",
            Punctuation::TemplateEnd => "}",
            Punctuation::TemplateExpr => "${",
        }
    }

    pub fn is_opening(&self) -> bool {
        matches!(
            self,
            Punctuation::LeftParen | Punctuation::LeftBrace | Punctuation::LeftBracket
        )
    }

    pub fn is_closing(&self) -> bool {
        matches!(
            self,
            Punctuation::RightParen | Punctuation::RightBrace | Punctuation::RightBracket
        )
    }

    pub fn matching_closing(&self) -> Option<Punctuation> {
        match self {
            Punctuation::LeftParen => Some(Punctuation::RightParen),
            Punctuation::LeftBrace => Some(Punctuation::RightBrace),
            Punctuation::LeftBracket => Some(Punctuation::RightBracket),
            _ => None,
        }
    }

    pub fn matching_opening(&self) -> Option<Punctuation> {
        match self {
            Punctuation::RightParen => Some(Punctuation::LeftParen),
            Punctuation::RightBrace => Some(Punctuation::LeftBrace),
            Punctuation::RightBracket => Some(Punctuation::LeftBracket),
            _ => None,
        }
    }
}

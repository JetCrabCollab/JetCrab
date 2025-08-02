use crate::lexer::scanners::LexerCore;
use crate::lexer::{LexerError, TokenKind};

pub trait OperatorReader {
    fn read_operator(&mut self) -> Result<TokenKind, LexerError>;
}

impl<T> OperatorReader for T
where
    T: LexerCore,
{
    fn read_operator(&mut self) -> Result<TokenKind, LexerError> {
        let c = self.source()[self.pos()];

        match c {
            '(' => {
                self.advance_pos();
                Ok(TokenKind::LeftParen)
            }
            ')' => {
                self.advance_pos();
                Ok(TokenKind::RightParen)
            }
            '{' => {
                self.advance_pos();
                Ok(TokenKind::LeftBrace)
            }
            '}' => {
                self.advance_pos();
                Ok(TokenKind::RightBrace)
            }
            '[' => {
                self.advance_pos();
                Ok(TokenKind::LeftBracket)
            }
            ']' => {
                self.advance_pos();
                Ok(TokenKind::RightBracket)
            }
            '.' => {
                self.advance_pos();
                Ok(TokenKind::Dot)
            }
            ';' => {
                self.advance_pos();
                Ok(TokenKind::Semicolon)
            }
            ',' => {
                self.advance_pos();
                Ok(TokenKind::Comma)
            }
            ':' => {
                self.advance_pos();
                Ok(TokenKind::Colon)
            }
            '?' => {
                self.advance_pos();
                Ok(TokenKind::Question)
            }
            '!' => {
                self.advance_pos();
                Ok(TokenKind::Exclamation)
            }
            '~' => {
                self.advance_pos();
                Ok(TokenKind::Tilde)
            }
            '=' => {
                self.advance_pos();
                Ok(TokenKind::Assign)
            }
            '+' => {
                self.advance_pos();
                Ok(TokenKind::Plus)
            }
            '-' => {
                self.advance_pos();
                Ok(TokenKind::Minus)
            }
            '*' => {
                self.advance_pos();
                Ok(TokenKind::Star)
            }
            '/' => {
                self.advance_pos();
                Ok(TokenKind::Slash)
            }
            '%' => {
                self.advance_pos();
                Ok(TokenKind::Percent)
            }
            '<' => {
                self.advance_pos();
                Ok(TokenKind::LessThan)
            }
            '>' => {
                self.advance_pos();
                Ok(TokenKind::GreaterThan)
            }
            '&' => {
                self.advance_pos();
                Ok(TokenKind::BitwiseAnd)
            }
            '|' => {
                self.advance_pos();
                Ok(TokenKind::BitwiseOr)
            }
            '^' => {
                self.advance_pos();
                Ok(TokenKind::BitwiseXor)
            }
            _ => Err(LexerError::UnexpectedCharacter(c)),
        }
    }
}

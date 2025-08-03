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

        // Check for three-character operators first
        if self.pos() + 2 < self.source().len() {
            let next_c = self.source()[self.pos() + 1];
            let next_next_c = self.source()[self.pos() + 2];
            let three_char_op = format!("{}{}{}", c, next_c, next_next_c);

            match three_char_op.as_str() {
                "===" => {
                    self.advance_pos();
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::StrictEqual);
                }
                "!==" => {
                    self.advance_pos();
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::StrictNotEqual);
                }
                "..." => {
                    self.advance_pos();
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::Spread);
                }
                _ => {}
            }
        }

        // Check for two-character operators
        if self.pos() + 1 < self.source().len() {
            let next_c = self.source()[self.pos() + 1];
            let two_char_op = format!("{}{}", c, next_c);

            match two_char_op.as_str() {
                "==" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::Equal);
                }
                "!=" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::NotEqual);
                }
                "<=" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::LessThanEqual);
                }
                ">=" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::GreaterThanEqual);
                }
                "++" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::Increment);
                }
                "--" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::Decrement);
                }
                "&&" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::LogicalAnd);
                }
                "||" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::LogicalOr);
                }
                "**" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::StarStar);
                }
                "=>" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::Arrow);
                }
                "??" => {
                    self.advance_pos();
                    self.advance_pos();
                    return Ok(TokenKind::NullishCoalescing);
                }
                _ => {}
            }
        }

        // Single character operators
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

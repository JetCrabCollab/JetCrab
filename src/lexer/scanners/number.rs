use crate::lexer::{LexerError, TokenKind};

pub trait NumberReader {
    fn read_number(&mut self) -> Result<TokenKind, LexerError>;
}

impl<T> NumberReader for T
where
    T: LexerCore,
{
    fn read_number(&mut self) -> Result<TokenKind, LexerError> {
        let mut number = String::new();
        let mut is_hex = false;
        let mut is_binary = false;
        let mut is_octal = false;

        if self.source()[self.pos()] == '0' && self.pos() + 1 < self.source().len() {
            match self.source()[self.pos() + 1] {
                'x' | 'X' => {
                    is_hex = true;
                    number.push('0');
                    number.push(self.source()[self.pos() + 1]);
                    self.advance_pos();
                    self.advance_pos();
                }
                'b' | 'B' => {
                    is_binary = true;
                    number.push('0');
                    number.push(self.source()[self.pos() + 1]);
                    self.advance_pos();
                    self.advance_pos();
                }
                'o' | 'O' => {
                    is_octal = true;
                    number.push('0');
                    number.push(self.source()[self.pos() + 1]);
                    self.advance_pos();
                    self.advance_pos();
                }
                _ => {}
            }
        }

        while self.pos() < self.source().len() {
            let c = self.source()[self.pos()];

            if is_hex {
                if c.is_ascii_hexdigit() {
                    number.push(c);
                    self.advance_pos();
                } else {
                    break;
                }
            } else if is_binary {
                if c == '0' || c == '1' {
                    number.push(c);
                    self.advance_pos();
                } else {
                    break;
                }
            } else if is_octal {
                if ('0'..='7').contains(&c) {
                    number.push(c);
                    self.advance_pos();
                } else {
                    break;
                }
            } else if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-'
            {
                number.push(c);
                self.advance_pos();
            } else {
                break;
            }
        }

        if self.pos() < self.source().len() && self.source()[self.pos()] == 'n' {
            number.push('n');
            self.advance_pos();
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
}

use crate::lexer::scanners::LexerCore;

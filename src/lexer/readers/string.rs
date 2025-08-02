use crate::lexer::{LexerError, TokenKind};

pub trait StringReader {
    fn read_string(&mut self) -> Result<TokenKind, LexerError>;
    fn read_template_string(&mut self) -> Result<TokenKind, LexerError>;
}

impl<T> StringReader for T
where
    T: LexerCore,
{
    fn read_string(&mut self) -> Result<TokenKind, LexerError> {
        let quote = self.source()[self.pos()];
        self.advance_pos();

        let mut string = String::new();
        let mut found_closing_quote = false;

        while self.pos() < self.source().len() {
            let c = self.source()[self.pos()];

            if c == quote {
                self.advance_pos();
                found_closing_quote = true;
                break;
            } else if c == '\\' {
                self.advance_pos();
                if self.pos() < self.source().len() {
                    let escaped = self.source()[self.pos()];
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        _ => string.push(escaped),
                    }
                    self.advance_pos();
                }
            } else {
                string.push(c);
                self.advance_pos();
            }
        }

        if !found_closing_quote {
            return Err(LexerError::UnterminatedString);
        }

        Ok(TokenKind::String(string))
    }

    fn read_template_string(&mut self) -> Result<TokenKind, LexerError> {
        self.advance_pos();

        let mut template = String::new();

        while self.pos() < self.source().len() {
            let c = self.source()[self.pos()];

            if c == '`' {
                self.advance_pos();
                break;
            } else if c == '$' && self.peek_char(1) == Some('{') {
                template.push_str("${");
                self.advance_pos();
                self.advance_pos();
            } else if c == '\\' {
                self.advance_pos();
                if self.pos() < self.source().len() {
                    let escaped = self.source()[self.pos()];
                    match escaped {
                        'n' => template.push('\n'),
                        't' => template.push('\t'),
                        'r' => template.push('\r'),
                        '\\' => template.push('\\'),
                        '`' => template.push('`'),
                        '$' => template.push('$'),
                        _ => template.push(escaped),
                    }
                    self.advance_pos();
                }
            } else {
                template.push(c);
                self.advance_pos();
            }
        }

        Ok(TokenKind::TemplateString(template))
    }
}

use crate::lexer::readers::LexerCore;

pub trait LexerCoreExt {
    fn peek_char(&self, offset: usize) -> Option<char>;
}

impl<T> LexerCoreExt for T
where
    T: LexerCore,
{
    fn peek_char(&self, offset: usize) -> Option<char> {
        let pos = self.pos() + offset;
        if pos < self.source().len() {
            Some(self.source()[pos])
        } else {
            None
        }
    }
}

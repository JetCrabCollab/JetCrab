use crate::lexer::{LexerError, TokenKind};

pub trait CommentReader {
    fn read_line_comment(&mut self) -> Result<TokenKind, LexerError>;
    fn read_block_comment(&mut self) -> Result<TokenKind, LexerError>;
}

impl<T> CommentReader for T
where
    T: LexerCore + LexerCoreExt,
{
    fn read_line_comment(&mut self) -> Result<TokenKind, LexerError> {
        self.advance_pos();
        self.advance_pos();

        let mut comment = String::new();

        while self.pos() < self.source().len() {
            let c = self.source()[self.pos()];
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance_pos();
        }

        Ok(TokenKind::Comment(comment))
    }

    fn read_block_comment(&mut self) -> Result<TokenKind, LexerError> {
        self.advance_pos();
        self.advance_pos();

        let mut comment = String::new();
        let mut found_closing_comment = false;

        while self.pos() < self.source().len() {
            let c = self.source()[self.pos()];

            if c == '*' && self.peek_char(1) == Some('/') {
                self.advance_pos();
                self.advance_pos();
                found_closing_comment = true;
                break;
            }

            comment.push(c);
            self.advance_pos();
        }

        if !found_closing_comment {
            return Err(LexerError::UnterminatedComment);
        }

        Ok(TokenKind::Comment(comment))
    }
}

use crate::lexer::scanners::{LexerCore, LexerCoreExt};

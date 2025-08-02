use crate::vm::types::{ColumnNumber, LineNumber};

pub trait PositionManager {
    fn skip_whitespace(&mut self);
    fn advance(&mut self);
    fn peek_char(&self, offset: usize) -> Option<char>;
    fn update_position(&mut self, start_line: LineNumber, start_col: ColumnNumber);
}

pub trait PositionCore {
    fn source(&self) -> &[char];
    fn pos(&self) -> usize;
    fn line(&self) -> LineNumber;
    fn column(&self) -> ColumnNumber;
    fn set_pos(&mut self, pos: usize);
    fn set_line(&mut self, line: LineNumber);
    fn set_column(&mut self, column: ColumnNumber);
}

impl<T> PositionManager for T
where
    T: PositionCore,
{
    fn skip_whitespace(&mut self) {
        while self.pos() < self.source().len() {
            let c = self.source()[self.pos()];
            if c.is_whitespace() {
                if c == '\n' {
                    let mut new_line = self.line();
                    new_line += 1;
                    self.set_line(new_line);
                    self.set_column(ColumnNumber::new(1));
                } else {
                    let mut new_column = self.column();
                    new_column += 1;
                    self.set_column(new_column);
                }
                self.advance();
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) {
        if self.pos() < self.source().len() {
            self.set_pos(self.pos() + 1);
            let mut new_column = self.column();
            new_column += 1;
            self.set_column(new_column);
        }
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        if self.pos() + offset < self.source().len() {
            Some(self.source()[self.pos() + offset])
        } else {
            None
        }
    }

    fn update_position(&mut self, _start_line: LineNumber, _start_col: ColumnNumber) {}
}

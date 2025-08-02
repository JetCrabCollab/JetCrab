use crate::vm::types::{ColumnNumber, LineNumber, SourcePosition};
use serde::{Deserialize, Serialize};
use std::fmt;

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

    pub fn new_typed(line: LineNumber, column: ColumnNumber) -> Self {
        Self { line, column }
    }

    pub fn from_source_position(pos: SourcePosition) -> Self {
        Self {
            line: pos.line,
            column: pos.column,
        }
    }

    pub fn to_source_position(&self) -> SourcePosition {
        SourcePosition {
            line: self.line,
            column: self.column,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
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

    pub fn from_positions_typed(
        start_line: LineNumber,
        start_col: ColumnNumber,
        end_line: LineNumber,
        end_col: ColumnNumber,
    ) -> Self {
        Self {
            start: Position::new_typed(start_line, start_col),
            end: Position::new_typed(end_line, end_col),
        }
    }
}

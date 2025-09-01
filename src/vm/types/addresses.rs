use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CodeAddress(usize);

impl CodeAddress {
    pub fn new(address: usize) -> Self {
        Self(address)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn add(&self, offset: usize) -> Self {
        Self(self.0 + offset)
    }
}

impl From<usize> for CodeAddress {
    fn from(address: usize) -> Self {
        Self(address)
    }
}

impl From<CodeAddress> for usize {
    fn from(addr: CodeAddress) -> Self {
        addr.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineNumber(usize);

impl LineNumber {
    pub fn new(line: usize) -> Self {
        Self(line)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0 > 0
    }
}

impl From<usize> for LineNumber {
    fn from(line: usize) -> Self {
        Self(line)
    }
}

impl From<LineNumber> for usize {
    fn from(line: LineNumber) -> Self {
        line.0
    }
}

impl fmt::Display for LineNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::AddAssign<usize> for LineNumber {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColumnNumber(usize);

impl ColumnNumber {
    pub fn new(column: usize) -> Self {
        Self(column)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0 > 0
    }
}

impl From<usize> for ColumnNumber {
    fn from(column: usize) -> Self {
        Self(column)
    }
}

impl From<ColumnNumber> for usize {
    fn from(col: ColumnNumber) -> Self {
        col.0
    }
}

impl fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::AddAssign<usize> for ColumnNumber {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourcePosition {
    pub line: LineNumber,
    pub column: ColumnNumber,
}

impl SourcePosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line: LineNumber::new(line),
            column: ColumnNumber::new(column),
        }
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.line.as_usize(), self.column.as_usize())
    }
}

impl From<(usize, usize)> for SourcePosition {
    fn from((line, column): (usize, usize)) -> Self {
        Self::new(line, column)
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CharOffset(usize);

impl CharOffset {
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    pub fn add(&self, other: usize) -> Self {
        Self(self.0 + other)
    }

    pub fn sub(&self, other: usize) -> Self {
        Self(self.0.saturating_sub(other))
    }
}

impl From<usize> for CharOffset {
    fn from(offset: usize) -> Self {
        Self(offset)
    }
}

impl From<CharOffset> for usize {
    fn from(offset: CharOffset) -> Self {
        offset.0
    }
}

impl std::ops::Add<usize> for CharOffset {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        Self(self.0 + other)
    }
}

impl std::ops::Sub<usize> for CharOffset {
    type Output = Self;

    fn sub(self, other: usize) -> Self {
        Self(self.0.saturating_sub(other))
    }
}

impl std::ops::AddAssign<usize> for CharOffset {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl std::ops::SubAssign<usize> for CharOffset {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 = self.0.saturating_sub(rhs);
    }
}

impl fmt::Display for CharOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

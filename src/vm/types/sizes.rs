use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct MemorySize(usize);

impl MemorySize {
    pub fn new(size: usize) -> Self {
        Self(size)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn add(&self, other: MemorySize) -> Self {
        Self(self.0 + other.0)
    }

    pub fn sub(&self, other: MemorySize) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl From<usize> for MemorySize {
    fn from(size: usize) -> Self {
        Self(size)
    }
}

impl From<MemorySize> for usize {
    fn from(size: MemorySize) -> Self {
        size.0
    }
}

impl std::ops::Add for MemorySize {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for MemorySize {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl fmt::Display for MemorySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bytes", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct AllocationCount(usize);

impl AllocationCount {
    pub fn new(count: usize) -> Self {
        Self(count)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

impl From<usize> for AllocationCount {
    fn from(count: usize) -> Self {
        Self(count)
    }
}

impl From<AllocationCount> for usize {
    fn from(count: AllocationCount) -> Self {
        count.0
    }
}

impl fmt::Display for AllocationCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeCount(usize);

impl NodeCount {
    pub fn new(count: usize) -> Self {
        Self(count)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

impl From<usize> for NodeCount {
    fn from(count: usize) -> Self {
        Self(count)
    }
}

impl From<NodeCount> for usize {
    fn from(count: NodeCount) -> Self {
        count.0
    }
}

impl fmt::Display for NodeCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IndentLevel(usize);

impl IndentLevel {
    pub fn new(level: usize) -> Self {
        Self(level)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

impl From<usize> for IndentLevel {
    fn from(level: usize) -> Self {
        Self(level)
    }
}

impl From<IndentLevel> for usize {
    fn from(level: IndentLevel) -> Self {
        level.0
    }
}

impl fmt::Display for IndentLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::AddAssign<usize> for IndentLevel {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl std::ops::SubAssign<usize> for IndentLevel {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 = self.0.saturating_sub(rhs);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectCount(usize);

impl ObjectCount {
    pub fn new(count: usize) -> Self {
        Self(count)
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

    pub fn add(&self, other: ObjectCount) -> Self {
        Self(self.0 + other.0)
    }

    pub fn sub(&self, other: ObjectCount) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl From<usize> for ObjectCount {
    fn from(count: usize) -> Self {
        Self(count)
    }
}

impl From<ObjectCount> for usize {
    fn from(count: ObjectCount) -> Self {
        count.0
    }
}

impl std::ops::Add for ObjectCount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for ObjectCount {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl fmt::Display for ObjectCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScopeDepth(usize);

impl ScopeDepth {
    pub fn new(depth: usize) -> Self {
        Self(depth)
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

    pub fn add(&self, other: ScopeDepth) -> Self {
        Self(self.0 + other.0)
    }

    pub fn sub(&self, other: ScopeDepth) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl From<usize> for ScopeDepth {
    fn from(depth: usize) -> Self {
        Self(depth)
    }
}

impl From<ScopeDepth> for usize {
    fn from(depth: ScopeDepth) -> Self {
        depth.0
    }
}

impl std::ops::Add for ScopeDepth {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for ScopeDepth {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl fmt::Display for ScopeDepth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VariableCount(usize);

impl VariableCount {
    pub fn new(count: usize) -> Self {
        Self(count)
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

    pub fn add(&self, other: VariableCount) -> Self {
        Self(self.0 + other.0)
    }

    pub fn sub(&self, other: VariableCount) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl From<usize> for VariableCount {
    fn from(count: usize) -> Self {
        Self(count)
    }
}

impl From<VariableCount> for usize {
    fn from(count: VariableCount) -> Self {
        count.0
    }
}

impl std::ops::Add for VariableCount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for VariableCount {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl fmt::Display for VariableCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectSize(usize);

impl ObjectSize {
    pub fn new(size: usize) -> Self {
        Self(size)
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

    pub fn add(&self, other: ObjectSize) -> Self {
        Self(self.0 + other.0)
    }

    pub fn sub(&self, other: ObjectSize) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl From<usize> for ObjectSize {
    fn from(size: usize) -> Self {
        Self(size)
    }
}

impl From<ObjectSize> for usize {
    fn from(size: ObjectSize) -> Self {
        size.0
    }
}

impl std::ops::Add for ObjectSize {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for ObjectSize {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl fmt::Display for ObjectSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ErrorCount(usize);

impl ErrorCount {
    pub fn new(count: usize) -> Self {
        Self(count)
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

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    pub fn add(&self, other: ErrorCount) -> Self {
        Self(self.0 + other.0)
    }

    pub fn sub(&self, other: ErrorCount) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl From<usize> for ErrorCount {
    fn from(count: usize) -> Self {
        Self(count)
    }
}

impl From<ErrorCount> for usize {
    fn from(count: ErrorCount) -> Self {
        count.0
    }
}

impl std::ops::Add for ErrorCount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for ErrorCount {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl std::ops::AddAssign for ErrorCount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl fmt::Display for ErrorCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

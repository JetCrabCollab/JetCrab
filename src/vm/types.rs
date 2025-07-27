use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConstantIndex(usize);

impl ConstantIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for ConstantIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<ConstantIndex> for usize {
    fn from(idx: ConstantIndex) -> Self {
        idx.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GlobalIndex(usize);

impl GlobalIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for GlobalIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<GlobalIndex> for usize {
    fn from(idx: GlobalIndex) -> Self {
        idx.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalIndex(usize);

impl LocalIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for LocalIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<LocalIndex> for usize {
    fn from(idx: LocalIndex) -> Self {
        idx.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArgIndex(usize);

impl ArgIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for ArgIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<ArgIndex> for usize {
    fn from(idx: ArgIndex) -> Self {
        idx.0
    }
}

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
pub struct FunctionIndex(usize);

impl FunctionIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for FunctionIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<FunctionIndex> for usize {
    fn from(idx: FunctionIndex) -> Self {
        idx.0
    }
}

impl fmt::Display for FunctionIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function({})", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArraySize(usize);

impl ArraySize {
    pub fn new(size: usize) -> Self {
        Self(size)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for ArraySize {
    fn from(size: usize) -> Self {
        Self(size)
    }
}

impl From<ArraySize> for usize {
    fn from(size: ArraySize) -> Self {
        size.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StackIndex(usize);

impl StackIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
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
}

impl From<usize> for StackIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<StackIndex> for usize {
    fn from(idx: StackIndex) -> Self {
        idx.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FramePointer(usize);

impl FramePointer {
    pub fn new(pointer: usize) -> Self {
        Self(pointer)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for FramePointer {
    fn from(pointer: usize) -> Self {
        Self(pointer)
    }
}

impl From<FramePointer> for usize {
    fn from(ptr: FramePointer) -> Self {
        ptr.0
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectId(usize);

impl ObjectId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0 != usize::MAX
    }
}

impl From<usize> for ObjectId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<ObjectId> for usize {
    fn from(id: ObjectId) -> Self {
        id.0
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object({})", self.0)
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

impl std::ops::AddAssign<usize> for LineNumber {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VariableName(String);

impl VariableName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        !self.0.is_empty() && self.0.chars().all(|c| c.is_alphanumeric() || c == '_')
    }
}

impl From<String> for VariableName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<&str> for VariableName {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<VariableName> for String {
    fn from(name: VariableName) -> Self {
        name.0
    }
}

impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PropertyName(String);

impl PropertyName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn is_computed(&self) -> bool {
        self.0.starts_with('[') && self.0.ends_with(']')
    }
}

impl From<String> for PropertyName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<&str> for PropertyName {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<PropertyName> for String {
    fn from(name: PropertyName) -> Self {
        name.0
    }
}

impl fmt::Display for PropertyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
}

impl BinaryOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Exponent => "**",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::StrictEqual => "===",
            BinaryOperator::StrictNotEqual => "!==",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessThanEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterThanEqual => ">=",
            BinaryOperator::LogicalAnd => "&&",
            BinaryOperator::LogicalOr => "||",
            BinaryOperator::BitwiseAnd => "&",
            BinaryOperator::BitwiseOr => "|",
            BinaryOperator::BitwiseXor => "^",
            BinaryOperator::LeftShift => "<<",
            BinaryOperator::RightShift => ">>",
            BinaryOperator::UnsignedRightShift => ">>>",
        }
    }

    pub fn is_assignment(&self) -> bool {
        matches!(
            self,
            BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
                | BinaryOperator::Modulo
                | BinaryOperator::Exponent
                | BinaryOperator::BitwiseAnd
                | BinaryOperator::BitwiseOr
                | BinaryOperator::BitwiseXor
                | BinaryOperator::LeftShift
                | BinaryOperator::RightShift
                | BinaryOperator::UnsignedRightShift
        )
    }

    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            BinaryOperator::Equal
                | BinaryOperator::NotEqual
                | BinaryOperator::StrictEqual
                | BinaryOperator::StrictNotEqual
                | BinaryOperator::LessThan
                | BinaryOperator::LessThanEqual
                | BinaryOperator::GreaterThan
                | BinaryOperator::GreaterThanEqual
        )
    }

    pub fn is_logical(&self) -> bool {
        matches!(self, BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr)
    }
}

impl From<&str> for BinaryOperator {
    fn from(op: &str) -> Self {
        match op {
            "+" => BinaryOperator::Add,
            "-" => BinaryOperator::Subtract,
            "*" => BinaryOperator::Multiply,
            "/" => BinaryOperator::Divide,
            "%" => BinaryOperator::Modulo,
            "**" => BinaryOperator::Exponent,
            "==" => BinaryOperator::Equal,
            "!=" => BinaryOperator::NotEqual,
            "===" => BinaryOperator::StrictEqual,
            "!==" => BinaryOperator::StrictNotEqual,
            "<" => BinaryOperator::LessThan,
            "<=" => BinaryOperator::LessThanEqual,
            ">" => BinaryOperator::GreaterThan,
            ">=" => BinaryOperator::GreaterThanEqual,
            "&&" => BinaryOperator::LogicalAnd,
            "||" => BinaryOperator::LogicalOr,
            "&" => BinaryOperator::BitwiseAnd,
            "|" => BinaryOperator::BitwiseOr,
            "^" => BinaryOperator::BitwiseXor,
            "<<" => BinaryOperator::LeftShift,
            ">>" => BinaryOperator::RightShift,
            ">>>" => BinaryOperator::UnsignedRightShift,
            _ => panic!("Unknown binary operator: {op}"),
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    BitwiseNot,
    Increment,
    Decrement,
    TypeOf,
    Void,
    Delete,
}

impl UnaryOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnaryOperator::Plus => "+",
            UnaryOperator::Minus => "-",
            UnaryOperator::Not => "!",
            UnaryOperator::BitwiseNot => "~",
            UnaryOperator::Increment => "++",
            UnaryOperator::Decrement => "--",
            UnaryOperator::TypeOf => "typeof",
            UnaryOperator::Void => "void",
            UnaryOperator::Delete => "delete",
        }
    }

    pub fn is_prefix(&self) -> bool {
        matches!(
            self,
            UnaryOperator::Plus
                | UnaryOperator::Minus
                | UnaryOperator::Not
                | UnaryOperator::BitwiseNot
                | UnaryOperator::TypeOf
                | UnaryOperator::Void
                | UnaryOperator::Delete
        )
    }

    pub fn is_postfix(&self) -> bool {
        matches!(self, UnaryOperator::Increment | UnaryOperator::Decrement)
    }
}

impl From<&str> for UnaryOperator {
    fn from(op: &str) -> Self {
        match op {
            "+" => UnaryOperator::Plus,
            "-" => UnaryOperator::Minus,
            "!" => UnaryOperator::Not,
            "~" => UnaryOperator::BitwiseNot,
            "++" => UnaryOperator::Increment,
            "--" => UnaryOperator::Decrement,
            "typeof" => UnaryOperator::TypeOf,
            "void" => UnaryOperator::Void,
            "delete" => UnaryOperator::Delete,
            _ => panic!("Unknown unary operator: {op}"),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Integer(i64);

impl Integer {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn as_i64(&self) -> i64 {
        self.0
    }

    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl From<usize> for Integer {
    fn from(value: usize) -> Self {
        Self(value as i64)
    }
}

impl From<Integer> for i64 {
    fn from(int: Integer) -> Self {
        int.0
    }
}

impl From<Integer> for f64 {
    fn from(int: Integer) -> Self {
        int.0 as f64
    }
}

impl std::ops::Add for Integer {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for Integer {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl std::ops::Mul for Integer {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl std::ops::Div for Integer {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Float(f64);

impl Float {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }

    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }

    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0.0
    }

    pub fn is_negative(&self) -> bool {
        self.0 < 0.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self(value as f64)
    }
}

impl From<Integer> for Float {
    fn from(int: Integer) -> Self {
        Self(int.as_f64())
    }
}

impl From<Float> for f64 {
    fn from(float: Float) -> Self {
        float.0
    }
}

impl std::ops::Add for Float {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for Float {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl std::ops::Mul for Float {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl std::ops::Div for Float {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0)
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ErrorCount(usize);

impl ErrorCount {
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

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
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

impl std::ops::AddAssign for ErrorCount {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl fmt::Display for ErrorCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HeapId(usize);

impl HeapId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0 != usize::MAX
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<usize> for HeapId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<HeapId> for usize {
    fn from(id: HeapId) -> Self {
        id.0
    }
}

impl fmt::Display for HeapId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct ObjectCount(usize);

impl ObjectCount {
    pub fn new(count: usize) -> Self {
        Self(count)
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

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct ScopeDepth(usize);

impl ScopeDepth {
    pub fn new(depth: usize) -> Self {
        Self(depth)
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

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct VariableCount(usize);

impl VariableCount {
    pub fn new(count: usize) -> Self {
        Self(count)
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

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct CharOffset(usize);

impl CharOffset {
    pub fn new(offset: usize) -> Self {
        Self(offset)
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

    pub fn add(&self, offset: usize) -> Self {
        Self(self.0 + offset)
    }

    pub fn sub(&self, offset: usize) -> Self {
        Self(self.0.saturating_sub(offset))
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
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
    fn add_assign(&mut self, other: usize) {
        self.0 += other;
    }
}

impl std::ops::SubAssign<usize> for CharOffset {
    fn sub_assign(&mut self, other: usize) {
        self.0 = self.0.saturating_sub(other);
    }
}

impl fmt::Display for CharOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct ObjectSize(usize);

impl ObjectSize {
    pub fn new(size: usize) -> Self {
        Self(size)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn add(&self, other: Self) -> Self {
        Self(self.0 + other.0)
    }

    pub fn sub(&self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_index() {
        let idx = ConstantIndex::new(42);
        assert_eq!(idx.as_usize(), 42);

        let from_usize: ConstantIndex = 123.into();
        assert_eq!(from_usize.as_usize(), 123);

        let back_to_usize: usize = idx.into();
        assert_eq!(back_to_usize, 42);
    }

    #[test]
    fn test_code_address() {
        let mut addr = CodeAddress::new(10);
        assert_eq!(addr.as_usize(), 10);

        addr.increment();
        assert_eq!(addr.as_usize(), 11);

        let new_addr = addr.add(5);
        assert_eq!(new_addr.as_usize(), 16);
    }

    #[test]
    fn test_stack_index() {
        let mut idx = StackIndex::new(5);
        assert_eq!(idx.as_usize(), 5);

        idx.increment();
        assert_eq!(idx.as_usize(), 6);

        idx.decrement();
        assert_eq!(idx.as_usize(), 5);
    }

    #[test]
    fn test_line_number() {
        let line = LineNumber::new(42);
        assert_eq!(line.as_usize(), 42);
        assert!(line.is_valid());
        assert_eq!(line.to_string(), "42");
    }

    #[test]
    fn test_source_position() {
        let pos = SourcePosition::new(5, 10);
        assert_eq!(pos.line.as_usize(), 5);
        assert_eq!(pos.column.as_usize(), 10);
        assert_eq!(pos.as_tuple(), (5, 10));
        assert_eq!(pos.to_string(), "5:10");
    }

    #[test]
    fn test_variable_name() {
        let name = VariableName::new("myVar".to_string());
        assert_eq!(name.as_str(), "myVar");
        assert!(name.is_valid());
        assert_eq!(name.to_string(), "myVar");

        let invalid = VariableName::new("my-var".to_string());
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_property_name() {
        let name = PropertyName::new("property".to_string());
        assert_eq!(name.as_str(), "property");
        assert!(!name.is_computed());

        let computed = PropertyName::new("[computed]".to_string());
        assert!(computed.is_computed());
    }

    #[test]
    fn test_binary_operator() {
        let op = BinaryOperator::Add;
        assert_eq!(op.as_str(), "+");
        assert!(!op.is_comparison());
        assert!(!op.is_logical());

        let comp_op = BinaryOperator::Equal;
        assert!(comp_op.is_comparison());

        let logical_op = BinaryOperator::LogicalAnd;
        assert!(logical_op.is_logical());
    }

    #[test]
    fn test_unary_operator() {
        let op = UnaryOperator::Plus;
        assert_eq!(op.as_str(), "+");
        assert!(op.is_prefix());
        assert!(!op.is_postfix());

        let postfix_op = UnaryOperator::Increment;
        assert!(postfix_op.is_postfix());
    }

    #[test]
    fn test_integer() {
        let int = Integer::new(42);
        assert_eq!(int.as_i64(), 42);
        assert_eq!(int.as_f64(), 42.0);
        assert!(int.is_positive());
        assert!(!int.is_negative());
        assert!(!int.is_zero());

        let zero = Integer::new(0);
        assert!(zero.is_zero());

        let sum = int + Integer::new(8);
        assert_eq!(sum.as_i64(), 50);
    }

    #[test]
    fn test_float() {
        let float = Float::new(std::f64::consts::PI);
        assert_eq!(float.as_f64(), std::f64::consts::PI);
        assert!(float.is_finite());
        assert!(!float.is_nan());
        assert!(!float.is_infinite());
        assert!(float.is_positive());

        let nan = Float::new(f64::NAN);
        assert!(nan.is_nan());

        let sum = float + Float::new(2.0);
        assert!((sum.as_f64() - 5.14).abs() < 1e-10);
    }

    #[test]
    fn test_error_count() {
        let mut count = ErrorCount::new(0);
        assert!(count.is_zero());
        assert!(!count.is_positive());

        count.increment();
        assert!(!count.is_zero());
        assert!(count.is_positive());
        assert_eq!(count.as_usize(), 1);

        count.reset();
        assert!(count.is_zero());

        let sum = ErrorCount::new(5) + ErrorCount::new(3);
        assert_eq!(sum.as_usize(), 8);
    }

    #[test]
    fn test_heap_id() {
        let id = HeapId::new(42);
        assert!(id.is_valid());
        assert_eq!(id.as_usize(), 42);

        let next = id.next();
        assert_eq!(next.as_usize(), 43);

        let invalid = HeapId::new(usize::MAX);
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_object_count() {
        let mut count = ObjectCount::new(0);
        assert!(count.is_zero());
        assert!(!count.is_positive());

        count.increment();
        assert!(!count.is_zero());
        assert!(count.is_positive());
        assert_eq!(count.as_usize(), 1);

        count.decrement();
        assert!(count.is_zero());

        let sum = ObjectCount::new(5) + ObjectCount::new(3);
        assert_eq!(sum.as_usize(), 8);

        let diff = ObjectCount::new(5) - ObjectCount::new(3);
        assert_eq!(diff.as_usize(), 2);

        let underflow = ObjectCount::new(2) - ObjectCount::new(5);
        assert_eq!(underflow.as_usize(), 0);
    }

    #[test]
    fn test_scope_depth() {
        let mut depth = ScopeDepth::new(0);
        assert!(depth.is_zero());
        assert!(!depth.is_positive());

        depth.increment();
        assert!(!depth.is_zero());
        assert!(depth.is_positive());
        assert_eq!(depth.as_usize(), 1);

        depth.decrement();
        assert!(depth.is_zero());

        let sum = ScopeDepth::new(5) + ScopeDepth::new(3);
        assert_eq!(sum.as_usize(), 8);

        let diff = ScopeDepth::new(5) - ScopeDepth::new(3);
        assert_eq!(diff.as_usize(), 2);

        let underflow = ScopeDepth::new(2) - ScopeDepth::new(5);
        assert_eq!(underflow.as_usize(), 0);
    }

    #[test]
    fn test_variable_count() {
        let mut count = VariableCount::new(0);
        assert!(count.is_zero());
        assert!(!count.is_positive());

        count.increment();
        assert!(!count.is_zero());
        assert!(count.is_positive());
        assert_eq!(count.as_usize(), 1);

        count.decrement();
        assert!(count.is_zero());

        let sum = VariableCount::new(5) + VariableCount::new(3);
        assert_eq!(sum.as_usize(), 8);

        let diff = VariableCount::new(5) - VariableCount::new(3);
        assert_eq!(diff.as_usize(), 2);

        let underflow = VariableCount::new(2) - VariableCount::new(5);
        assert_eq!(underflow.as_usize(), 0);
    }

    #[test]
    fn test_char_offset() {
        let mut offset = CharOffset::new(0);
        assert!(offset.is_zero());
        assert!(!offset.is_positive());

        offset.increment();
        assert!(!offset.is_zero());
        assert!(offset.is_positive());
        assert_eq!(offset.as_usize(), 1);

        offset.decrement();
        assert!(offset.is_zero());

        let sum = CharOffset::new(5) + 3;
        assert_eq!(sum.as_usize(), 8);

        let diff = CharOffset::new(5) - 3;
        assert_eq!(diff.as_usize(), 2);

        let underflow = CharOffset::new(2) - 5;
        assert_eq!(underflow.as_usize(), 0);

        offset += 5;
        assert_eq!(offset.as_usize(), 5);

        offset -= 2;
        assert_eq!(offset.as_usize(), 3);
    }

    #[test]
    fn test_object_size() {
        let size = ObjectSize::new(0);
        assert!(size.is_zero());
        assert!(!size.is_positive());

        let size = ObjectSize::new(8);
        assert!(!size.is_zero());
        assert!(size.is_positive());
        assert_eq!(size.as_usize(), 8);

        let sum = ObjectSize::new(5) + ObjectSize::new(3);
        assert_eq!(sum.as_usize(), 8);

        let diff = ObjectSize::new(5) - ObjectSize::new(3);
        assert_eq!(diff.as_usize(), 2);

        let underflow = ObjectSize::new(2) - ObjectSize::new(5);
        assert_eq!(underflow.as_usize(), 0);
    }
}

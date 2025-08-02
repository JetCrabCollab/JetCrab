use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    // Assignment operators
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    StarStarAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,

    // Comparison operators
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    // Logical operators
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,

    // Bitwise operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    BitwiseNot,

    // Increment/Decrement
    Increment,
    Decrement,

    // Special operators
    Arrow,
    OptionalChaining,
    Spread,
    Rest,
    PrivateField,
}

impl Operator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Operator::Assign => "=",
            Operator::PlusAssign => "+=",
            Operator::MinusAssign => "-=",
            Operator::StarAssign => "*=",
            Operator::SlashAssign => "/=",
            Operator::PercentAssign => "%=",
            Operator::StarStarAssign => "**=",
            Operator::LeftShiftAssign => "<<=",
            Operator::RightShiftAssign => ">>=",
            Operator::UnsignedRightShiftAssign => ">>>=",
            Operator::BitwiseAndAssign => "&=",
            Operator::BitwiseOrAssign => "|=",
            Operator::BitwiseXorAssign => "^=",
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::StrictEqual => "===",
            Operator::StrictNotEqual => "!==",
            Operator::LessThan => "<",
            Operator::LessThanEqual => "<=",
            Operator::GreaterThan => ">",
            Operator::GreaterThanEqual => ">=",
            Operator::LogicalAnd => "&&",
            Operator::LogicalOr => "||",
            Operator::LogicalNot => "!",
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Slash => "/",
            Operator::Percent => "%",
            Operator::StarStar => "**",
            Operator::BitwiseAnd => "&",
            Operator::BitwiseOr => "|",
            Operator::BitwiseXor => "^",
            Operator::LeftShift => "<<",
            Operator::RightShift => ">>",
            Operator::UnsignedRightShift => ">>>",
            Operator::BitwiseNot => "~",
            Operator::Increment => "++",
            Operator::Decrement => "--",
            Operator::Arrow => "=>",
            Operator::OptionalChaining => "?.",
            Operator::Spread => "...",
            Operator::Rest => "...",
            Operator::PrivateField => "#",
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Operator::LogicalOr => 1,
            Operator::LogicalAnd => 2,
            Operator::BitwiseOr => 3,
            Operator::BitwiseXor => 4,
            Operator::BitwiseAnd => 5,
            Operator::Equal
            | Operator::NotEqual
            | Operator::StrictEqual
            | Operator::StrictNotEqual => 6,
            Operator::LessThan
            | Operator::LessThanEqual
            | Operator::GreaterThan
            | Operator::GreaterThanEqual => 7,
            Operator::LeftShift | Operator::RightShift | Operator::UnsignedRightShift => 8,
            Operator::Plus | Operator::Minus => 9,
            Operator::Star | Operator::Slash | Operator::Percent => 10,
            Operator::StarStar => 11,
            Operator::LogicalNot | Operator::BitwiseNot => 12,
            Operator::Increment | Operator::Decrement => 13,
            _ => 0,
        }
    }

    pub fn is_assignment(&self) -> bool {
        matches!(
            self,
            Operator::Assign
                | Operator::PlusAssign
                | Operator::MinusAssign
                | Operator::StarAssign
                | Operator::SlashAssign
                | Operator::PercentAssign
                | Operator::StarStarAssign
                | Operator::LeftShiftAssign
                | Operator::RightShiftAssign
                | Operator::UnsignedRightShiftAssign
                | Operator::BitwiseAndAssign
                | Operator::BitwiseOrAssign
                | Operator::BitwiseXorAssign
        )
    }

    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            Operator::Equal
                | Operator::NotEqual
                | Operator::StrictEqual
                | Operator::StrictNotEqual
                | Operator::LessThan
                | Operator::LessThanEqual
                | Operator::GreaterThan
                | Operator::GreaterThanEqual
        )
    }

    pub fn is_logical(&self) -> bool {
        matches!(
            self,
            Operator::LogicalAnd | Operator::LogicalOr | Operator::LogicalNot
        )
    }

    pub fn is_arithmetic(&self) -> bool {
        matches!(
            self,
            Operator::Plus
                | Operator::Minus
                | Operator::Star
                | Operator::Slash
                | Operator::Percent
                | Operator::StarStar
        )
    }
}

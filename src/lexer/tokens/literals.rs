use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Number(f64),
    BigInt(String),
    String(String),
    TemplateString(String),
    Boolean(bool),
    Null,
    Undefined,
    Regex(String),
}

impl Literal {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Literal::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Literal::String(s) => Some(s),
            Literal::TemplateString(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Literal::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Literal::Null)
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, Literal::Undefined)
    }

    pub fn is_falsy(&self) -> bool {
        match self {
            Literal::Boolean(false) => true,
            Literal::Null => true,
            Literal::Undefined => true,
            Literal::Number(n) => *n == 0.0,
            Literal::String(s) => s.is_empty(),
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        !self.is_falsy()
    }
}

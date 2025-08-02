use crate::ast::common::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left: Box<crate::ast::node::Node>,
    pub operator: String,
    pub right: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: String,
    pub argument: Box<crate::ast::node::Node>,
    pub prefix: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateExpression {
    pub operator: String,
    pub argument: Box<crate::ast::node::Node>,
    pub prefix: bool,
    pub span: Option<Span>,
}

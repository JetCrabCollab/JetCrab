use crate::ast::common::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalExpression {
    pub left: Box<crate::ast::node::Node>,
    pub operator: String,
    pub right: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalExpression {
    pub test: Box<crate::ast::node::Node>,
    pub consequent: Box<crate::ast::node::Node>,
    pub alternate: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

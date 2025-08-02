use crate::ast::common::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignmentExpression {
    pub left: Box<crate::ast::node::Node>,
    pub operator: String,
    pub right: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpression {
    pub callee: Box<crate::ast::node::Node>,
    pub arguments: Vec<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewExpression {
    pub callee: Box<crate::ast::node::Node>,
    pub arguments: Vec<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberExpression {
    pub object: Box<crate::ast::node::Node>,
    pub property: Box<crate::ast::node::Node>,
    pub computed: bool,
    pub optional: bool,
    pub span: Option<Span>,
}

use crate::ast::common::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Super {
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaProperty {
    pub meta: Box<crate::ast::node::Node>,
    pub property: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YieldExpression {
    pub argument: Option<Box<crate::ast::node::Node>>,
    pub delegate: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwaitExpression {
    pub argument: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegExp {
    pub pattern: String,
    pub flags: String,
    pub span: Option<Span>,
}

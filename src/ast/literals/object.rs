use crate::ast::common::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectLiteral {
    pub properties: Vec<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub key: Box<crate::ast::node::Node>,
    pub value: Box<crate::ast::node::Node>,
    pub kind: String,
    pub computed: bool,
    pub method: bool,
    pub shorthand: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadElement {
    pub argument: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestElement {
    pub argument: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

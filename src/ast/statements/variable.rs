use crate::ast::common::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub kind: String,
    pub declarations: Vec<VariableDeclarator>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclarator {
    pub id: Box<crate::ast::node::Node>,
    pub init: Option<Box<crate::ast::node::Node>>,
    pub span: Option<Span>,
}

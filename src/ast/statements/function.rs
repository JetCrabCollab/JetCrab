use crate::ast::common::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub id: Option<Box<crate::ast::node::Node>>,
    pub params: Vec<crate::ast::node::Node>,
    pub body: Box<crate::ast::node::Node>,
    pub generator: bool,
    pub r#async: bool,
    pub span: Option<Span>,
}

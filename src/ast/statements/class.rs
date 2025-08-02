use crate::ast::common::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassDeclaration {
    pub id: Option<Box<crate::ast::node::Node>>,
    pub super_class: Option<Box<crate::ast::node::Node>>,
    pub body: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

use crate::ast::common::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrowFunctionExpression {
    pub params: Vec<crate::ast::node::Node>,
    pub body: Box<crate::ast::node::Node>,
    pub expression: bool,
    pub r#async: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionExpression {
    pub id: Option<Box<crate::ast::node::Node>>,
    pub params: Vec<crate::ast::node::Node>,
    pub body: Box<crate::ast::node::Node>,
    pub generator: bool,
    pub r#async: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassExpression {
    pub id: Option<Box<crate::ast::node::Node>>,
    pub super_class: Option<Box<crate::ast::node::Node>>,
    pub body: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateLiteral {
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateElement {
    pub value: String,
    pub tail: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaggedTemplateExpression {
    pub tag: Box<crate::ast::node::Node>,
    pub quasi: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

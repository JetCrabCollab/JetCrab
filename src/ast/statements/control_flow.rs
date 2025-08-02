use crate::ast::common::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockStatement {
    pub body: Vec<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub test: Box<crate::ast::node::Node>,
    pub consequent: Box<crate::ast::node::Node>,
    pub alternate: Option<Box<crate::ast::node::Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStatement {
    pub init: Option<Box<crate::ast::node::Node>>,
    pub test: Option<Box<crate::ast::node::Node>>,
    pub update: Option<Box<crate::ast::node::Node>>,
    pub body: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStatement {
    pub test: Box<crate::ast::node::Node>,
    pub body: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoWhileStatement {
    pub body: Box<crate::ast::node::Node>,
    pub test: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchStatement {
    pub discriminant: Box<crate::ast::node::Node>,
    pub cases: Vec<SwitchCase>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchCase {
    pub test: Option<Box<crate::ast::node::Node>>,
    pub consequent: Vec<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TryStatement {
    pub block: Box<crate::ast::node::Node>,
    pub handler: Option<Box<crate::ast::node::Node>>,
    pub finalizer: Option<Box<crate::ast::node::Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatchClause {
    pub param: Box<crate::ast::node::Node>,
    pub body: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrowStatement {
    pub argument: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub argument: Option<Box<crate::ast::node::Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakStatement {
    pub label: Option<Box<crate::ast::node::Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueStatement {
    pub label: Option<Box<crate::ast::node::Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabeledStatement {
    pub label: Box<crate::ast::node::Node>,
    pub body: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithStatement {
    pub object: Box<crate::ast::node::Node>,
    pub body: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebuggerStatement {
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Box<crate::ast::node::Node>,
    pub span: Option<Span>,
}

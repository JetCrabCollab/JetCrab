use crate::ast::common::Span;
use crate::ast::expressions::*;
use crate::ast::literals::*;
use crate::ast::statements::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub body: Vec<Node>,
    pub source_type: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportDeclaration {
    pub specifiers: Vec<Node>,
    pub source: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportSpecifier {
    pub local: Box<Node>,
    pub imported: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportDefaultSpecifier {
    pub local: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportNamespaceSpecifier {
    pub local: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportDeclaration {
    pub declaration: Option<Box<Node>>,
    pub specifiers: Vec<Node>,
    pub source: Option<Box<Node>>,
    pub default: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportSpecifier {
    pub local: Box<Node>,
    pub exported: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Node {
    Program(Program),

    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),

    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    UpdateExpression(UpdateExpression),
    LogicalExpression(LogicalExpression),
    ConditionalExpression(ConditionalExpression),
    AssignmentExpression(AssignmentExpression),
    CallExpression(CallExpression),
    NewExpression(NewExpression),
    MemberExpression(MemberExpression),
    ArrowFunctionExpression(ArrowFunctionExpression),
    FunctionExpression(FunctionExpression),
    ClassExpression(ClassExpression),
    YieldExpression(YieldExpression),
    AwaitExpression(AwaitExpression),

    BlockStatement(BlockStatement),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    SwitchStatement(SwitchStatement),
    TryStatement(TryStatement),
    CatchClause(CatchClause),
    ThrowStatement(ThrowStatement),
    ReturnStatement(ReturnStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    LabeledStatement(LabeledStatement),
    WithStatement(WithStatement),
    DebuggerStatement(DebuggerStatement),
    ExpressionStatement(ExpressionStatement),

    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    TemplateLiteral(TemplateLiteral),
    TaggedTemplateExpression(TaggedTemplateExpression),

    Property(Property),
    SpreadElement(SpreadElement),
    RestElement(RestElement),
    Super(Super),
    MetaProperty(MetaProperty),
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
    This,
    RegExp(RegExp),
    BigInt(String),
}

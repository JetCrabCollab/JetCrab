pub mod common;
pub mod error;
pub mod expressions;
pub mod literals;
pub mod node;
pub mod serialization;
pub mod statements;
pub mod visitor;

pub use common::{Position, Span};
pub use error::AstError;
pub use node::{
    ExportDeclaration, ExportSpecifier, ImportDeclaration, ImportDefaultSpecifier,
    ImportNamespaceSpecifier, ImportSpecifier, Node, Program,
};
pub use visitor::Visitor;

// Re-export all statement types
pub use statements::{
    BlockStatement, BreakStatement, CatchClause, ClassDeclaration, ContinueStatement,
    DebuggerStatement, DoWhileStatement, ExpressionStatement, ForStatement, FunctionDeclaration,
    IfStatement, LabeledStatement, ReturnStatement, SwitchCase, SwitchStatement, ThrowStatement,
    TryStatement, VariableDeclaration, VariableDeclarator, WhileStatement, WithStatement,
};

// Re-export all expression types
pub use expressions::{
    AssignmentExpression, AwaitExpression, BinaryExpression, CallExpression, ConditionalExpression,
    LogicalExpression, MemberExpression, MetaProperty, NewExpression, RegExp, Super,
    UnaryExpression, UpdateExpression, YieldExpression,
};

// Re-export all literal types
pub use literals::{
    ArrayLiteral, ArrowFunctionExpression, ClassExpression, FunctionExpression, ObjectLiteral,
    Property, RestElement, SpreadElement, TaggedTemplateExpression, TemplateElement,
    TemplateLiteral,
};

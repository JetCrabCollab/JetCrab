//! # Abstract Syntax Tree (AST) Module
//!
//! Defines the data structures representing JavaScript code as an abstract
//! syntax tree, enabling programmatic manipulation and analysis of code.
//!
//! ## Overview
//!
//! The AST module provides comprehensive node types for:
//!
//! - **Program Structure**: Scripts, modules, and declarations
//! - **Statements**: Control flow, loops, and declarations
//! - **Expressions**: Operations, calls, and assignments
//! - **Literals**: Values, objects, and functions
//! - **Common Elements**: Positions, spans, and metadata
//!
//! ## Node Types
//!
//! Each AST node includes:
//! - **Position Information**: Line and column numbers
//! - **Type Safety**: Strongly typed node variants
//! - **Visitor Pattern**: Traversal and transformation support
//! - **Serialization**: JSON export capabilities
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::ast::{Node, Program, Visitor};
//!
//! let program = Program::new(vec![]);
//! let node = Node::Program(program);
//!
//! // Use visitor pattern for traversal
//! struct MyVisitor;
//! impl Visitor for MyVisitor {
//!     // Implementation details...
//! }
//! ```

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

pub use statements::{
    BlockStatement, BreakStatement, CatchClause, ClassDeclaration, ContinueStatement,
    DebuggerStatement, DoWhileStatement, ExpressionStatement, ForStatement, FunctionDeclaration,
    IfStatement, LabeledStatement, ReturnStatement, SwitchCase, SwitchStatement, ThrowStatement,
    TryStatement, VariableDeclaration, VariableDeclarator, WhileStatement, WithStatement,
};

pub use expressions::{
    AssignmentExpression, AwaitExpression, BinaryExpression, CallExpression, ConditionalExpression,
    LogicalExpression, MemberExpression, MetaProperty, NewExpression, RegExp, Super,
    UnaryExpression, UpdateExpression, YieldExpression,
};

pub use literals::{
    ArrayLiteral, ArrowFunctionExpression, ClassExpression, FunctionExpression, ObjectLiteral,
    Property, RestElement, SpreadElement, TaggedTemplateExpression, TemplateElement,
    TemplateLiteral,
};

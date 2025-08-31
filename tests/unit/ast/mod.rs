//! AST Unit Tests - Mirroring src/ast/ structure
//! 
//! This module contains unit tests for abstract syntax tree:
//! - node.rs: AST node definitions
//! - error.rs: AST errors
//! - serialization.rs: AST serialization
//! - common/: Common AST components
//! - expressions/: Expression nodes
//! - literals/: Literal nodes
//! - statements/: Statement nodes
//! - visitor/: AST visitors

pub mod node;
pub mod error;
pub mod serialization;
pub mod common;
pub mod expressions;
pub mod literals;
pub mod statements;
pub mod visitor;

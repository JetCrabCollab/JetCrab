//! VM Compiler Unit Tests - Mirroring src/vm/compiler/ structure
//! 
//! This module contains unit tests for compiler components:
//! - generator.rs: Bytecode generation
//! - optimizer.rs: Bytecode optimization
//! - bytecode.rs: Bytecode structure
//! - error.rs: Compiler errors
//! - scope/: Scope management
//! - expressions/: Expression compilation
//! - statements/: Statement compilation
//! - literals/: Literal compilation

pub mod generator;
pub mod optimizer;
pub mod bytecode;
pub mod error;
pub mod scope;
pub mod expressions;
pub mod statements;
pub mod literals;

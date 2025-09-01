//! # Semantic Analysis Module
//!
//! Performs semantic analysis of JavaScript code, including type checking,
//! scope validation, and semantic error detection.
//!
//! ## Overview
//!
//! The semantic analyzer validates:
//!
//! - **Type Safety**: Type checking and validation
//! - **Scope Rules**: Variable declaration and usage
//! - **Semantic Rules**: Language-specific constraints
//! - **Error Detection**: Semantic error identification
//! - **Symbol Resolution**: Name binding and resolution
//!
//! ## Analysis Phases
//!
//! 1. **Scope Analysis**: Build symbol tables
//! 2. **Type Checking**: Validate type compatibility
//! 3. **Semantic Validation**: Check language rules
//! 4. **Error Reporting**: Collect and report issues
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::semantic::SemanticAnalyzer;
//! use jetcrab::ast::Node;
//! use jetcrab::parser::parse;
//!
//! let mut analyzer = SemanticAnalyzer::new();
//! let ast = parse("let x = 42;").unwrap();
//! let result = analyzer.analyze(&ast).unwrap();
//! ```

pub use crate::semantic::analyzer::SemanticAnalyzer;
pub use crate::semantic::error::SemanticError;
pub use crate::semantic::scope::{Scope, VariableInfo};

pub mod analyzer;
pub mod error;
pub mod scope;

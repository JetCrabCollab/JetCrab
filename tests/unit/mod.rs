//! Unit tests for JetCrab - Mirroring the src/ structure
//! 
//! This module contains unit tests organized to match the src/ directory structure:
//! - vm/: Virtual machine components
//! - api/: Public API
//! - lexer/: Lexical analysis
//! - parser/: Syntax parsing
//! - ast/: Abstract syntax tree
//! - semantic/: Semantic analysis

// VM Components
pub mod vm;

// API Components
pub mod api;

// Lexical Analysis
pub mod lexer;

// Syntax Parsing
pub mod parser;

// Abstract Syntax Tree
pub mod ast;

// Semantic Analysis
pub mod semantic;

// Memory Management Integration
pub mod gc_spaces_integration_tests;

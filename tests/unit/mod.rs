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
pub mod vm_builtin_calls_tests;
pub mod vm_control_flow_tests;
pub mod vm_core_tests;
pub mod vm_error_tests;
pub mod vm_executor_tests;
pub mod vm_generational_heap_tests;
pub mod vm_instruction_dispatcher_tests;
pub mod vm_memory_allocator_tests;
pub mod vm_object_shapes_tests;
pub mod vm_string_interning_tests;

// API Components
pub mod api;
pub mod api_debug_tests;
pub mod api_error_tests;
pub mod api_events_tests;
pub mod api_modules_tests;

// Lexical Analysis
pub mod lexer;
pub mod lexer_tokens_tests;

// Syntax Parsing
pub mod parser;

// Abstract Syntax Tree
pub mod ast;

// Semantic Analysis
pub mod semantic;

// Error Types
pub mod error_tests;

// Memory Management Integration
pub mod gc_spaces_integration_tests;

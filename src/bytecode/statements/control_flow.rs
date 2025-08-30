use crate::ast::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::CodeAddress;

pub trait ControlFlowGenerator {
    fn generate_if_statement(&mut self, node: &Node);
    fn generate_for_statement(&mut self, node: &Node);
    fn generate_while_statement(&mut self, node: &Node);
    fn generate_do_while_statement(&mut self, node: &Node);
    fn generate_switch_statement(&mut self, node: &Node);
    fn generate_try_statement(&mut self, node: &Node);
    fn generate_catch_clause(&mut self, node: &Node);
    fn generate_labeled_statement(&mut self, node: &Node);
    fn generate_break_statement(&mut self, node: &Node);
    fn generate_continue_statement(&mut self, node: &Node);
    fn generate_return_statement(&mut self, node: &Node);
    fn generate_throw_statement(&mut self, node: &Node);
    fn generate_for_in_statement(&mut self, node: &Node);
    fn generate_for_of_statement(&mut self, node: &Node);
    fn generate_import_declaration(&mut self, node: &Node);
    fn generate_export_declaration(&mut self, node: &Node);
}

pub trait LabelManager {
    fn add_label(&mut self, name: String, address: CodeAddress);
    fn get_label_address(&self, name: &str) -> Option<CodeAddress>;
    fn get_label_start_address(&self, label_name: &str) -> Option<CodeAddress>;
    fn get_label_end_address(&self, label_name: &str) -> Option<CodeAddress>;
    fn push_current_label(&mut self, name: String);
    fn pop_current_label(&mut self);
    fn get_current_labels(&self) -> &[String];
}

pub trait ControlFlowCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
    fn push_loop_label(&mut self, break_address: CodeAddress);
    fn pop_loop_label(&mut self);
    fn get_current_break_address(&self) -> Option<CodeAddress>;
}

// Import all statement modules
use super::control_statements;
use super::if_statement;
use super::loop_statements::{self, generate_for_in_statement, generate_for_of_statement};
use super::modules::{generate_export_declaration, generate_import_declaration};
use super::switch_statement;
use super::try_catch;

impl<T> ControlFlowGenerator for T
where
    T: ControlFlowCore + LabelManager,
{
    fn generate_if_statement(&mut self, node: &Node) {
        if_statement::generate_if_statement(self, node);
    }

    fn generate_for_statement(&mut self, node: &Node) {
        loop_statements::generate_for_statement(self, node);
    }

    fn generate_while_statement(&mut self, node: &Node) {
        loop_statements::generate_while_statement(self, node);
    }

    fn generate_do_while_statement(&mut self, node: &Node) {
        loop_statements::generate_do_while_statement(self, node);
    }

    fn generate_switch_statement(&mut self, node: &Node) {
        switch_statement::generate_switch_statement(self, node);
    }

    fn generate_try_statement(&mut self, node: &Node) {
        try_catch::generate_try_statement(self, node);
    }

    fn generate_catch_clause(&mut self, node: &Node) {
        try_catch::generate_catch_clause(self, node);
    }

    fn generate_labeled_statement(&mut self, node: &Node) {
        control_statements::generate_labeled_statement(self, node);
    }

    fn generate_break_statement(&mut self, node: &Node) {
        control_statements::generate_break_statement(self, node);
    }

    fn generate_continue_statement(&mut self, node: &Node) {
        control_statements::generate_continue_statement(self, node);
    }

    fn generate_return_statement(&mut self, node: &Node) {
        control_statements::generate_return_statement(self, node);
    }

    fn generate_throw_statement(&mut self, node: &Node) {
        try_catch::generate_throw_statement(self, node);
    }

    fn generate_for_in_statement(&mut self, node: &Node) {
        generate_for_in_statement(self, node);
    }

    fn generate_for_of_statement(&mut self, node: &Node) {
        generate_for_of_statement(self, node);
    }

    fn generate_import_declaration(&mut self, node: &Node) {
        generate_import_declaration(self, node);
    }

    fn generate_export_declaration(&mut self, node: &Node) {
        generate_export_declaration(self, node);
    }
}

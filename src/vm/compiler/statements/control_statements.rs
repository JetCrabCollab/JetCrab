use crate::ast::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::CodeAddress;

use super::{ControlFlowCore, LabelManager};

pub fn generate_break_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore + LabelManager,
{
    if let Node::BreakStatement(stmt) = node {
        if let Some(label) = &stmt.label {
            // Break with label - jump to the labeled statement end
            if let Node::Identifier(label_name) = &**label {
                if let Some(label_end_address) = this.get_label_end_address(label_name) {
                    // Break with specific label - jump to the end of the labeled statement
                    this.instructions()
                        .push(Instruction::Jump(label_end_address));
                } else {
                    // Label not found, use default break behavior
                    this.instructions()
                        .push(Instruction::Jump(CodeAddress::new(9999)));
                }
            } else {
                // Invalid label type, use default break behavior
                this.instructions()
                    .push(Instruction::Jump(CodeAddress::new(9999)));
            }
        } else {
            // Break without label - use default break behavior
            this.instructions()
                .push(Instruction::Jump(CodeAddress::new(9999)));
        }
    }
}

pub fn generate_continue_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore + LabelManager,
{
    if let Node::ContinueStatement(stmt) = node {
        if let Some(label) = &stmt.label {
            // Continue with label - jump to the labeled statement start
            if let Node::Identifier(label_name) = &**label {
                if let Some(label_start_address) = this.get_label_start_address(label_name) {
                    // Continue with specific label - jump to the start of the labeled statement
                    this.instructions()
                        .push(Instruction::Jump(label_start_address));
                } else {
                    // Label not found, use default continue behavior
                    this.instructions()
                        .push(Instruction::Jump(CodeAddress::new(8888)));
                }
            } else {
                // Invalid label type, use default continue behavior
                this.instructions()
                    .push(Instruction::Jump(CodeAddress::new(8888)));
            }
        } else {
            // Continue without label - use default continue behavior
            this.instructions()
                .push(Instruction::Jump(CodeAddress::new(8888)));
        }
    }
}

pub fn generate_return_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::ReturnStatement(stmt) = node {
        if let Some(arg) = &stmt.argument {
            this.visit_node(arg);
        }
        this.instructions().push(Instruction::Return);
    }
}

pub fn generate_labeled_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore + LabelManager,
{
    if let Node::LabeledStatement(stmt) = node {
        // Extract label name
        let label_name = if let Node::Identifier(name) = &*stmt.label {
            name.clone()
        } else {
            "unknown".to_string()
        };

        // Mark the start of the labeled statement
        let label_start = this.instructions().len();

        // Store the label start address
        this.add_label(format!("{label_name}_start"), CodeAddress::new(label_start));

        // Push current label for nested statements
        this.push_current_label(label_name.clone());

        // Generate the labeled statement body
        this.visit_node(&stmt.body);

        // Mark the end of the labeled statement
        let label_end = this.instructions().len();

        // Store the label end address
        this.add_label(format!("{label_name}_end"), CodeAddress::new(label_end));

        // Pop current label
        this.pop_current_label();
    }
}

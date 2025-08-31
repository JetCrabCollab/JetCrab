use crate::ast::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::CodeAddress;

use super::ControlFlowCore;

pub fn generate_if_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::IfStatement(stmt) = node {
        // Generate test condition
        this.visit_node(&stmt.test);

        // Jump to else block if condition is false
        let jump_to_else_pos = this.instructions().len();
        this.instructions()
            .push(Instruction::JumpIfFalse(CodeAddress::new(0))); // Placeholder

        // Generate consequent block
        this.visit_node(&stmt.consequent);

        // Jump over else block
        let jump_over_else_pos = this.instructions().len();
        this.instructions()
            .push(Instruction::Jump(CodeAddress::new(0))); // Placeholder

        // Update jump to else address
        let else_start_pos = this.instructions().len();
        this.instructions()[jump_to_else_pos] =
            Instruction::JumpIfFalse(CodeAddress::new(else_start_pos));

        // Generate alternate block (else)
        if let Some(alt) = &stmt.alternate {
            this.visit_node(alt);
        }

        // Update jump over else address
        let end_pos = this.instructions().len();
        this.instructions()[jump_over_else_pos] = Instruction::Jump(CodeAddress::new(end_pos));
    }
}

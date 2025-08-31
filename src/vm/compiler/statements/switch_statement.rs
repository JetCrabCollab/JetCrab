use crate::ast::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::CodeAddress;

use super::ControlFlowCore;

pub fn generate_switch_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::SwitchStatement(stmt) = node {
        // Generate discriminant expression
        this.visit_node(&stmt.discriminant);

        // Store the discriminant value for comparison
        this.instructions().push(Instruction::Dup);

        // Generate all case tests and bodies
        let mut case_jumps = Vec::new();
        let mut case_bodies = Vec::new();
        let mut case_tests = Vec::new();

        for case in &stmt.cases {
            if let Some(test) = &case.test {
                // Generate test expression
                this.visit_node(test);

                // Compare with discriminant
                this.instructions().push(Instruction::Eq);

                // Jump to case body if equal
                let jump_pos = this.instructions().len();
                this.instructions()
                    .push(Instruction::JumpIfTrue(CodeAddress::new(0))); // Placeholder
                case_jumps.push(jump_pos);
                case_tests.push(true);
            } else {
                // Default case - no test needed
                case_jumps.push(0);
                case_tests.push(false);
            }

            // Mark case body start
            let body_start = this.instructions().len();
            case_bodies.push(body_start);

            // Generate case body
            for cons in &case.consequent {
                this.visit_node(cons);
            }

            // Jump to end of switch (to avoid fall-through)
            let _jump_to_end_pos = this.instructions().len();
            this.instructions()
                .push(Instruction::Jump(CodeAddress::new(0))); // Placeholder

            // Update case jump address
            if let Some(jump_pos) = case_jumps.last_mut() {
                if *jump_pos > 0 {
                    this.instructions()[*jump_pos] =
                        Instruction::JumpIfTrue(CodeAddress::new(body_start));
                }
            }
        }

        // Mark end of switch
        let switch_end = this.instructions().len();

        // Update all jump-to-end addresses
        for i in case_bodies {
            // Find the jump instruction after each case body
            if i < this.instructions().len() - 1 {
                if let Instruction::Jump(addr) = &this.instructions()[i + 1] {
                    if addr.as_usize() == 0 {
                        this.instructions()[i + 1] =
                            Instruction::Jump(CodeAddress::new(switch_end));
                    }
                }
            }
        }

        // Pop the duplicated discriminant
        this.instructions().push(Instruction::Pop);
    }
}

use crate::ast::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::CodeAddress;

use super::ControlFlowCore;

pub fn generate_for_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::ForStatement(stmt) = node {
        // Generate initialization
        if let Some(init) = &stmt.init {
            this.visit_node(init);
        }

        // Mark the start of the loop (test condition)
        let loop_start = this.instructions().len();

        // Generate test condition
        if let Some(test) = &stmt.test {
            this.visit_node(test);

            // Jump out of loop if condition is false
            let jump_out_pos = this.instructions().len();
            this.instructions()
                .push(Instruction::JumpIfFalse(CodeAddress::new(0))); // Placeholder

            // Generate loop body
            this.visit_node(&stmt.body);

            // Generate update
            if let Some(update) = &stmt.update {
                this.visit_node(update);
            }

            // Jump back to loop start (test condition)
            this.instructions()
                .push(Instruction::Jump(CodeAddress::new(loop_start)));

            // Mark the end of the loop
            let loop_end = this.instructions().len();

            // Update jump out address
            this.instructions()[jump_out_pos] =
                Instruction::JumpIfFalse(CodeAddress::new(loop_end));

            // Patch all break statements (Jump to 9999) in this loop
            for i in loop_start..loop_end {
                if let Instruction::Jump(addr) = &this.instructions()[i] {
                    if addr.as_usize() == 9999 {
                        this.instructions()[i] = Instruction::Jump(CodeAddress::new(loop_end));
                    }
                }
            }

            // Patch all continue statements (Jump to 8888) in this loop
            let continue_target = loop_start;

            for i in loop_start..loop_end {
                if let Instruction::Jump(addr) = &this.instructions()[i] {
                    if addr.as_usize() == 8888 {
                        this.instructions()[i] =
                            Instruction::Jump(CodeAddress::new(continue_target));
                    }
                }
            }
        } else {
            // Infinite loop (no test condition)
            this.visit_node(&stmt.body);

            // Generate update
            if let Some(update) = &stmt.update {
                this.visit_node(update);
            }

            // Jump back to loop start
            this.instructions()
                .push(Instruction::Jump(CodeAddress::new(loop_start)));
        }
    }
}

pub fn generate_while_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::WhileStatement(stmt) = node {
        // Mark the start of the loop
        let loop_start = this.instructions().len();

        // Check if condition is a literal true
        let is_true_literal = matches!(&*stmt.test, Node::Boolean(true));

        let jump_out_pos = if !is_true_literal {
            // Generate test condition
            this.visit_node(&stmt.test);

            // Jump out of loop if condition is false
            let pos = this.instructions().len();
            this.instructions()
                .push(Instruction::JumpIfFalse(CodeAddress::new(0))); // Placeholder
            Some(pos)
        } else {
            None
        };

        // Generate loop body
        this.visit_node(&stmt.body);

        // Jump back to loop start
        this.instructions()
            .push(Instruction::Jump(CodeAddress::new(loop_start)));

        // Mark the end of the loop (for break statements)
        let loop_end = this.instructions().len();

        // Update jump out address if we have one
        if let Some(jump_pos) = jump_out_pos {
            this.instructions()[jump_pos] = Instruction::JumpIfFalse(CodeAddress::new(loop_end));
        }

        // Patch all break statements (Jump to 9999) in this loop
        for i in loop_start..loop_end {
            if let Instruction::Jump(addr) = &this.instructions()[i] {
                if addr.as_usize() == 9999 {
                    this.instructions()[i] = Instruction::Jump(CodeAddress::new(loop_end));
                }
            }
        }

        let continue_target = loop_start;

        for i in loop_start..loop_end {
            if let Instruction::Jump(addr) = &this.instructions()[i] {
                if addr.as_usize() == 8888 {
                    this.instructions()[i] = Instruction::Jump(CodeAddress::new(continue_target));
                }
            }
        }
    }
}

pub fn generate_do_while_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::DoWhileStatement(stmt) = node {
        // Mark the start of the loop body
        let loop_start = this.instructions().len();

        // Generate loop body (executes at least once)
        this.visit_node(&stmt.body);

        // Generate test condition
        this.visit_node(&stmt.test);

        // Jump back to loop start if condition is true
        this.instructions()
            .push(Instruction::JumpIfTrue(CodeAddress::new(loop_start)));

        // Mark the end of the loop (for break statements)
        let loop_end = this.instructions().len();

        // Patch all break statements (Jump to 9999) in this loop
        for i in loop_start..loop_end {
            if let Instruction::Jump(addr) = &this.instructions()[i] {
                if addr.as_usize() == 9999 {
                    this.instructions()[i] = Instruction::Jump(CodeAddress::new(loop_end));
                }
            }
        }

        // Patch all continue statements (Jump to 8888) in this loop
        // Continue should jump to the test condition
        for i in loop_start..loop_end {
            if let Instruction::Jump(addr) = &this.instructions()[i] {
                if addr.as_usize() == 8888 {
                    // Find the position where the test condition starts
                    let test_start = loop_start + 1; // After the body
                    this.instructions()[i] = Instruction::Jump(CodeAddress::new(test_start));
                }
            }
        }
    }
}

pub fn generate_for_in_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::ForInStatement(stmt) = node {
        this.visit_node(&stmt.left);
        this.visit_node(&stmt.right);
        this.visit_node(&stmt.body);
    }
}

pub fn generate_for_of_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::ForOfStatement(stmt) = node {
        this.visit_node(&stmt.left);
        this.visit_node(&stmt.right);
        this.visit_node(&stmt.body);
    }
}

use crate::ast::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::CodeAddress;

pub trait ControlFlowGenerator {
    fn generate_if_statement(&mut self, node: &Node);
    fn generate_for_statement(&mut self, node: &Node);
    fn generate_while_statement(&mut self, node: &Node);
    fn generate_do_while_statement(&mut self, node: &Node);
    fn generate_break_statement(&mut self, node: &Node);
    fn generate_continue_statement(&mut self, node: &Node);
    fn generate_return_statement(&mut self, node: &Node);
    fn generate_throw_statement(&mut self, node: &Node);
}

pub trait ControlFlowCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> ControlFlowGenerator for T
where
    T: ControlFlowCore,
{
    fn generate_if_statement(&mut self, node: &Node) {
        if let Node::IfStatement(stmt) = node {
            // Generate test condition
            self.visit_node(&stmt.test);

            // Jump to else block if condition is false
            let jump_to_else_pos = self.instructions().len();
            self.instructions()
                .push(Instruction::JumpIfFalse(CodeAddress::new(0))); // Placeholder

            // Generate consequent block
            self.visit_node(&stmt.consequent);

            // Jump over else block
            let jump_over_else_pos = self.instructions().len();
            self.instructions()
                .push(Instruction::Jump(CodeAddress::new(0))); // Placeholder

            // Update jump to else address
            let else_start_pos = self.instructions().len();
            self.instructions()[jump_to_else_pos] =
                Instruction::JumpIfFalse(CodeAddress::new(else_start_pos));

            // Generate alternate block (else)
            if let Some(alt) = &stmt.alternate {
                self.visit_node(alt);
            }

            // Update jump over else address
            let end_pos = self.instructions().len();
            self.instructions()[jump_over_else_pos] = Instruction::Jump(CodeAddress::new(end_pos));
        }
    }

    fn generate_for_statement(&mut self, node: &Node) {
        if let Node::ForStatement(stmt) = node {
            // Generate initialization
            if let Some(init) = &stmt.init {
                self.visit_node(init);
            }

            // Mark the start of the loop (test condition)
            let loop_start = self.instructions().len();

            // Generate test condition
            if let Some(test) = &stmt.test {
                self.visit_node(test);

                // Jump out of loop if condition is false
                let jump_out_pos = self.instructions().len();
                self.instructions()
                    .push(Instruction::JumpIfFalse(CodeAddress::new(0))); // Placeholder

                // Generate loop body
                self.visit_node(&stmt.body);

                // Generate update
                if let Some(update) = &stmt.update {
                    self.visit_node(update);
                }

                // Jump back to loop start (test condition)
                self.instructions()
                    .push(Instruction::Jump(CodeAddress::new(loop_start)));

                // Update jump out address
                let end_pos = self.instructions().len();
                self.instructions()[jump_out_pos] =
                    Instruction::JumpIfFalse(CodeAddress::new(end_pos));
            } else {
                // Infinite loop (no test condition)
                self.visit_node(&stmt.body);

                // Generate update
                if let Some(update) = &stmt.update {
                    self.visit_node(update);
                }

                // Jump back to loop start
                self.instructions()
                    .push(Instruction::Jump(CodeAddress::new(loop_start)));
            }
        }
    }

    fn generate_while_statement(&mut self, node: &Node) {
        if let Node::WhileStatement(stmt) = node {
            // Mark the start of the loop
            let loop_start = self.instructions().len();

            // Generate test condition
            self.visit_node(&stmt.test);

            // Jump out of loop if condition is false
            let jump_out_pos = self.instructions().len();
            self.instructions()
                .push(Instruction::JumpIfFalse(CodeAddress::new(0))); // Placeholder

            // Generate loop body
            self.visit_node(&stmt.body);

            // Jump back to loop start
            self.instructions()
                .push(Instruction::Jump(CodeAddress::new(loop_start)));

            // Update jump out address
            let end_pos = self.instructions().len();
            self.instructions()[jump_out_pos] = Instruction::JumpIfFalse(CodeAddress::new(end_pos));
        }
    }

    fn generate_do_while_statement(&mut self, node: &Node) {
        if let Node::DoWhileStatement(stmt) = node {
            self.visit_node(&stmt.body);
            self.visit_node(&stmt.test);
        }
    }

    fn generate_break_statement(&mut self, _node: &Node) {
        self.instructions()
            .push(Instruction::Jump(CodeAddress::new(0)));
    }

    fn generate_continue_statement(&mut self, _node: &Node) {
        self.instructions()
            .push(Instruction::Jump(CodeAddress::new(0)));
    }

    fn generate_return_statement(&mut self, node: &Node) {
        if let Node::ReturnStatement(stmt) = node {
            if let Some(arg) = &stmt.argument {
                self.visit_node(arg);
            }
            self.instructions().push(Instruction::Return);
        }
    }

    fn generate_throw_statement(&mut self, node: &Node) {
        if let Node::ThrowStatement(stmt) = node {
            self.visit_node(&stmt.argument);
            self.instructions().push(Instruction::Throw);
        }
    }
}

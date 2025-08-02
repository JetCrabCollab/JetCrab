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
            self.visit_node(&stmt.test);
            self.visit_node(&stmt.consequent);
            if let Some(alt) = &stmt.alternate {
                self.visit_node(alt);
            }
        }
    }

    fn generate_for_statement(&mut self, node: &Node) {
        if let Node::ForStatement(stmt) = node {
            if let Some(init) = &stmt.init {
                self.visit_node(init);
            }
            if let Some(test) = &stmt.test {
                self.visit_node(test);
            }
            if let Some(update) = &stmt.update {
                self.visit_node(update);
            }
            self.visit_node(&stmt.body);
        }
    }

    fn generate_while_statement(&mut self, node: &Node) {
        if let Node::WhileStatement(stmt) = node {
            self.visit_node(&stmt.test);
            self.visit_node(&stmt.body);
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

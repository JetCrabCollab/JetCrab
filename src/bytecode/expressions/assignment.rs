use crate::ast::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::{CodeAddress, LocalIndex};

pub trait AssignmentGenerator {
    fn generate_assignment_expression(&mut self, node: &Node);
    fn generate_conditional_expression(&mut self, node: &Node);
    fn generate_call_expression(&mut self, node: &Node);
    fn generate_new_expression(&mut self, node: &Node);
    fn generate_member_expression(&mut self, node: &Node);
}

pub trait AssignmentCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> AssignmentGenerator for T
where
    T: AssignmentCore,
{
    fn generate_assignment_expression(&mut self, node: &Node) {
        if let Node::AssignmentExpression(expr) = node {
            self.visit_node(&expr.right);
            self.visit_node(&expr.left);
            self.instructions()
                .push(Instruction::StoreLocal(LocalIndex::new(0)));
        }
    }

    fn generate_conditional_expression(&mut self, node: &Node) {
        if let Node::ConditionalExpression(expr) = node {
            self.visit_node(&expr.test);

            let jump_to_alternate = self.instructions().len();
            self.instructions()
                .push(Instruction::JumpIfFalse(CodeAddress::new(0)));

            self.visit_node(&expr.consequent);

            let jump_to_end = self.instructions().len();
            self.instructions()
                .push(Instruction::Jump(CodeAddress::new(0)));

            let alternate_start = self.instructions().len();
            self.instructions()[jump_to_alternate] =
                Instruction::JumpIfFalse(CodeAddress::new(alternate_start));

            self.visit_node(&expr.alternate);

            let end_pos = self.instructions().len();
            self.instructions()[jump_to_end] = Instruction::Jump(CodeAddress::new(end_pos));
        }
    }

    fn generate_call_expression(&mut self, node: &Node) {
        if let Node::CallExpression(expr) = node {
            for arg in &expr.arguments {
                self.visit_node(arg);
            }
            self.visit_node(&expr.callee);
            self.instructions()
                .push(Instruction::Call(crate::vm::types::FunctionIndex::new(
                    expr.arguments.len(),
                )));
        }
    }

    fn generate_new_expression(&mut self, node: &Node) {
        if let Node::NewExpression(expr) = node {
            for arg in &expr.arguments {
                self.visit_node(arg);
            }
            self.visit_node(&expr.callee);
            self.instructions().push(Instruction::New);
        }
    }

    fn generate_member_expression(&mut self, node: &Node) {
        if let Node::MemberExpression(expr) = node {
            self.visit_node(&expr.object);
            self.visit_node(&expr.property);
            self.instructions().push(Instruction::GetProperty);
        }
    }
}

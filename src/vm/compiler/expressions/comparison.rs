use crate::ast::Node;
use crate::vm::instructions::Instruction;

pub trait ComparisonGenerator {
    fn generate_comparison_expression(&mut self, node: &Node);
}

pub trait ComparisonCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> ComparisonGenerator for T
where
    T: ComparisonCore,
{
    fn generate_comparison_expression(&mut self, node: &Node) {
        if let Node::BinaryExpression(expr) = node {
            self.visit_node(&expr.left);
            self.visit_node(&expr.right);
            match expr.operator.as_str() {
                "<" => self.instructions().push(Instruction::Lt),
                ">" => self.instructions().push(Instruction::Gt),
                "<=" => self.instructions().push(Instruction::Le),
                ">=" => self.instructions().push(Instruction::Ge),
                "==" => self.instructions().push(Instruction::Eq),
                "!=" => self.instructions().push(Instruction::Ne),
                "===" => self.instructions().push(Instruction::StrictEq),
                "!==" => self.instructions().push(Instruction::StrictNe),
                _ => {}
            }
        }
    }
}

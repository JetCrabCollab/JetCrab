use crate::ast::Node;
use crate::vm::instructions::Instruction;

pub trait LogicalGenerator {
    fn generate_logical_expression(&mut self, node: &Node);
}

pub trait LogicalCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> LogicalGenerator for T
where
    T: LogicalCore,
{
    fn generate_logical_expression(&mut self, node: &Node) {
        if let Node::LogicalExpression(expr) = node {
            self.visit_node(&expr.left);
            self.visit_node(&expr.right);
            match expr.operator.as_str() {
                "&&" => self.instructions().push(Instruction::And),
                "||" => self.instructions().push(Instruction::Or),
                "??" => self.instructions().push(Instruction::NullishCoalesce),
                _ => {
                    self.instructions().push(Instruction::And);
                }
            }
        }
    }
}

use crate::ast::Node;
use crate::vm::instructions::Instruction;

pub trait ArithmeticGenerator {
    fn generate_binary_expression(&mut self, node: &Node);
}

pub trait ArithmeticCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> ArithmeticGenerator for T
where
    T: ArithmeticCore,
{
    fn generate_binary_expression(&mut self, node: &Node) {
        if let Node::BinaryExpression(expr) = node {
            self.visit_node(&expr.left);
            self.visit_node(&expr.right);
            match expr.operator.as_str() {
                "+" => self.instructions().push(Instruction::Add),
                "-" => self.instructions().push(Instruction::Sub),
                "*" => self.instructions().push(Instruction::Mul),
                "/" => self.instructions().push(Instruction::Div),
                "%" => self.instructions().push(Instruction::Mod),
                "**" => self.instructions().push(Instruction::Exp),
                _ => {
                    self.instructions().push(Instruction::Add);
                }
            }
        }
    }
}

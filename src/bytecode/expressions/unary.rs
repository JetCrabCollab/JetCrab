use crate::ast::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::ConstantIndex;

pub trait UnaryGenerator {
    fn generate_unary_expression(&mut self, node: &Node);
    fn generate_update_expression(&mut self, node: &Node);
}

pub trait UnaryCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> UnaryGenerator for T
where
    T: UnaryCore,
{
    fn generate_unary_expression(&mut self, node: &Node) {
        if let Node::UnaryExpression(expr) = node {
            self.visit_node(&expr.argument);
            match expr.operator.as_str() {
                "!" => self.instructions().push(Instruction::Not),
                "-" => {
                    self.instructions()
                        .push(Instruction::PushConst(ConstantIndex::new(0)));
                    self.instructions().push(Instruction::Sub);
                }
                "+" => {}
                "~" => {
                    self.instructions()
                        .push(Instruction::PushConst(ConstantIndex::new(0)));
                    self.instructions().push(Instruction::Sub);
                    self.instructions().push(Instruction::Inc);
                }
                "typeof" => self.instructions().push(Instruction::TypeOf),
                "void" => {
                    self.instructions().push(Instruction::Pop);
                    self.instructions().push(Instruction::PushUndefined);
                }
                "delete" => self.instructions().push(Instruction::Delete),
                _ => {}
            }
        }
    }

    fn generate_update_expression(&mut self, node: &Node) {
        if let Node::UpdateExpression(expr) = node {
            self.visit_node(&expr.argument);
            match expr.operator.as_str() {
                "++" => {
                    if expr.prefix {
                        self.instructions().push(Instruction::Inc);
                    } else {
                        self.instructions().push(Instruction::Dup);
                        self.instructions().push(Instruction::Inc);
                    }
                }
                "--" => {
                    if expr.prefix {
                        self.instructions().push(Instruction::Dec);
                    } else {
                        self.instructions().push(Instruction::Dup);
                        self.instructions().push(Instruction::Dec);
                    }
                }
                _ => {
                    self.instructions().push(Instruction::Inc);
                }
            }
        }
    }
}

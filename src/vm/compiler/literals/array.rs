use crate::ast::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::ArraySize;

pub trait ArrayGenerator {
    fn generate_array_literal(&mut self, node: &Node);
}

pub trait ArrayCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> ArrayGenerator for T
where
    T: ArrayCore,
{
    fn generate_array_literal(&mut self, node: &Node) {
        if let Node::ArrayLiteral(lit) = node {
            for e in lit.elements.iter().flatten() {
                self.visit_node(e);
            }
            self.instructions()
                .push(Instruction::NewArray(ArraySize::new(lit.elements.len())));
        }
    }
}

use crate::ast::Node;
use crate::vm::instructions::Instruction;

pub trait ObjectGenerator {
    fn generate_object_literal(&mut self, node: &Node);
    fn generate_property(&mut self, node: &Node);
}

pub trait ObjectCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> ObjectGenerator for T
where
    T: ObjectCore,
{
    fn generate_object_literal(&mut self, node: &Node) {
        if let Node::ObjectLiteral(lit) = node {
            self.instructions().push(Instruction::NewObject);
            for prop in &lit.properties {
                if let Node::Property(property) = prop {
                    self.visit_node(&property.key);
                    self.visit_node(&property.value);
                    self.instructions().push(Instruction::SetProperty);
                }
            }
        }
    }

    fn generate_property(&mut self, node: &Node) {
        if let Node::Property(prop) = node {
            self.visit_node(&prop.key);
            self.visit_node(&prop.value);
        }
    }
}

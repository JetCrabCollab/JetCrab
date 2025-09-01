use crate::ast::Node;
use crate::vm::instructions::Instruction;

pub trait ClassGenerator {
    fn generate_class_declaration(&mut self, node: &Node);
    fn generate_class_expression(&mut self, node: &Node);
}

pub trait ClassCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> ClassGenerator for T
where
    T: ClassCore,
{
    fn generate_class_declaration(&mut self, node: &Node) {
        if let Node::ClassDeclaration(decl) = node {
            if let Some(id) = &decl.id {
                self.visit_node(id);
            }
            if let Some(super_class) = &decl.super_class {
                self.visit_node(super_class);
            }
            self.visit_node(&decl.body);
            self.instructions().push(Instruction::NewClass);
        }
    }

    fn generate_class_expression(&mut self, node: &Node) {
        if let Node::ClassExpression(expr) = node {
            if let Some(id) = &expr.id {
                self.visit_node(id);
            }
            if let Some(super_class) = &expr.super_class {
                self.visit_node(super_class);
            }
            self.visit_node(&expr.body);
            self.instructions().push(Instruction::NewClass);
        }
    }
}

use crate::ast::Node;
use crate::vm::instructions::Instruction;

pub trait FunctionGenerator {
    fn generate_function_declaration(&mut self, node: &Node);
}

pub trait FunctionCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> FunctionGenerator for T
where
    T: FunctionCore,
{
    fn generate_function_declaration(&mut self, node: &Node) {
        if let Node::FunctionDeclaration(decl) = node {
            if let Some(id) = &decl.id {
                self.visit_node(id);
            }
            for param in &decl.params {
                self.visit_node(param);
            }
            self.visit_node(&decl.body);
        }
    }
}

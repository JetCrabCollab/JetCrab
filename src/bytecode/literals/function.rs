use crate::ast::Node;
use crate::vm::instructions::Instruction;

pub trait FunctionLiteralGenerator {
    fn generate_function_expression(&mut self, node: &Node);
    fn generate_arrow_function_expression(&mut self, node: &Node);
}

pub trait FunctionLiteralCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> FunctionLiteralGenerator for T
where
    T: FunctionLiteralCore,
{
    fn generate_function_expression(&mut self, node: &Node) {
        if let Node::FunctionExpression(expr) = node {
            if let Some(id) = &expr.id {
                self.visit_node(id);
            }
            for param in &expr.params {
                self.visit_node(param);
            }
            self.visit_node(&expr.body);
        }
    }

    fn generate_arrow_function_expression(&mut self, node: &Node) {
        if let Node::ArrowFunctionExpression(expr) = node {
            for param in &expr.params {
                self.visit_node(param);
            }
            self.visit_node(&expr.body);
        }
    }
}

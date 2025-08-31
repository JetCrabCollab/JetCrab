use crate::ast::Node;
use crate::vm::instructions::Instruction;

pub trait FunctionGenerator {
    fn generate_function_declaration(&mut self, node: &Node);
    fn generate_function_expression(&mut self, node: &Node);
}

pub trait FunctionCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> FunctionGenerator for T
where
    T: FunctionCore + crate::vm::compiler::scope::constants::ConstantManager,
{
    fn generate_function_declaration(&mut self, node: &Node) {
        if let Node::FunctionDeclaration(decl) = node {
            // For now, implement a simplified version
            // Store function name as a special constant
            if let Some(id) = &decl.id {
                if let Node::Identifier(name) = &**id {
                    // Create a special function identifier
                    let function_id = self.add_constant(format!("__FUNCTION_{}", name));
                    self.instructions()
                        .push(Instruction::PushConst(function_id));

                    // Store the function in global scope (for now, just push it)
                    // TODO: Implement proper global variable storage
                    self.instructions().push(Instruction::PushUndefined);
                }
            }

            // Generate function body for later execution
            self.visit_node(&decl.body);
        }
    }

    fn generate_function_expression(&mut self, node: &Node) {
        if let Node::FunctionExpression(expr) = node {
            // For now, implement a simplified version
            // Generate function body
            self.visit_node(&expr.body);

            // Store the function as a callable function
            // We'll use a special constant to mark this as a function
            let function_id = format!(
                "__FUNCTION_EXPR_{}",
                expr.id.as_ref().map(|_id| "named").unwrap_or("anonymous")
            );
            let constant_index = self.add_constant(function_id);
            self.instructions()
                .push(Instruction::PushConst(constant_index));
        }
    }
}

use crate::ast::Node;
use crate::vm::instructions::Instruction;

use crate::vm::function::Function;

pub trait FunctionGenerator {
    fn generate_function_declaration(&mut self, node: &Node);
    fn generate_function_expression(&mut self, node: &Node);
}

pub trait FunctionCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

pub trait FunctionManagerCore {
    fn register_function(&mut self, function: Function);
}

impl<T> FunctionGenerator for T
where
    T: FunctionCore + crate::vm::compiler::scope::constants::ConstantManager + FunctionManagerCore,
{
    fn generate_function_declaration(&mut self, node: &Node) {
        if let Node::FunctionDeclaration(decl) = node {
            if let Some(id) = &decl.id {
                if let Node::Identifier(name) = &**id {
                    let function_name = name.clone();

                    // Extract parameters
                    let params: Vec<String> = decl
                        .params
                        .iter()
                        .filter_map(|param| {
                            if let Node::Identifier(param_name) = param {
                                Some(param_name.clone())
                            } else {
                                None
                            }
                        })
                        .collect();

                    // Create function
                    let mut function = Function::new(function_name.clone(), params.clone());

                    // Generate function body bytecode
                    // First, add LoadArg instructions for parameters
                    for (i, param) in params.iter().enumerate() {
                        // Add LoadArg instruction for each parameter
                        function.add_instruction(Instruction::LoadArg(
                            crate::vm::types::ArgIndex::new(i),
                        ));
                        // Store the parameter in local variables
                        let local_index = function.add_local(param.clone());
                        function.add_instruction(Instruction::StoreLocal(local_index));
                    }

                    // Visit the function body to generate bytecode
                    self.visit_node(&decl.body);

                    // Get the generated instructions
                    let instructions = self.instructions().clone();
                    function.bytecode = instructions;

                    // Clear the instructions from the main generator
                    self.instructions().clear();

                    // Register the function
                    self.register_function(function);

                    // Push undefined as the result of function declaration
                    self.instructions().push(Instruction::PushUndefined);
                }
            }
        }
    }

    fn generate_function_expression(&mut self, node: &Node) {
        if let Node::FunctionExpression(expr) = node {
            // For arrow functions and function expressions
            let function_name = expr
                .id
                .as_ref()
                .and_then(|id| {
                    if let Node::Identifier(name) = &**id {
                        Some(name.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "anonymous".to_string());

            // Extract parameters
            let params: Vec<String> = expr
                .params
                .iter()
                .filter_map(|param| {
                    if let Node::Identifier(param_name) = param {
                        Some(param_name.clone())
                    } else {
                        None
                    }
                })
                .collect();

            // Create function
            let mut function = Function::new(function_name.clone(), params.clone());

            // Generate function body bytecode
            // First, add LoadArg instructions for parameters
            for (i, param) in params.iter().enumerate() {
                // Add LoadArg instruction for each parameter
                function.add_instruction(Instruction::LoadArg(crate::vm::types::ArgIndex::new(i)));
                // Store the parameter in local variables
                let local_index = function.add_local(param.clone());
                function.add_instruction(Instruction::StoreLocal(local_index));
            }

            // Visit the function body to generate bytecode
            self.visit_node(&expr.body);

            // Get the generated instructions
            let instructions = self.instructions().clone();
            function.bytecode = instructions;

            // Clear the instructions from the main generator
            self.instructions().clear();

            // Register the function
            self.register_function(function);

            // Push undefined as the result of function expression
            self.instructions().push(Instruction::PushUndefined);
        }
    }
}

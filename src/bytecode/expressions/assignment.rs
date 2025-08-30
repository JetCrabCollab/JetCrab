use crate::ast::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::{CodeAddress, LocalIndex};

pub trait AssignmentGenerator {
    fn generate_assignment_expression(&mut self, node: &Node);
    fn generate_conditional_expression(&mut self, node: &Node);
    fn generate_call_expression(&mut self, node: &Node);
    fn generate_new_expression(&mut self, node: &Node);
    fn generate_member_expression(&mut self, node: &Node);
}

pub trait AssignmentCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> AssignmentGenerator for T
where
    T: AssignmentCore + crate::bytecode::scope::constants::ConstantCore + crate::bytecode::scope::local_vars::ScopeManager,
{
    fn generate_assignment_expression(&mut self, node: &Node) {
        if let Node::AssignmentExpression(expr) = node {
            match expr.operator.as_str() {
                "=" => {
                    // Simple assignment
                    match &*expr.left {
                        Node::Identifier(ident) => {
                            // Variable assignment: x = value
                            self.visit_node(&expr.right);
                            let local_idx = self.get_or_create_local(ident);
                            self.instructions().push(Instruction::StoreLocal(local_idx));
                        }
                        Node::MemberExpression(member) => {
                            // Property assignment: obj.prop = value or obj[prop] = value
                            self.visit_node(&member.object);  // Push object
                            
                            // Handle property names correctly
                            match &*member.property {
                                Node::Identifier(name) if !member.computed => {
                                    // obj.prop = value (dot notation)
                                    let constant_id = <Self as crate::bytecode::scope::constants::ConstantManager>::add_constant(self, name.clone());
                                    self.instructions().push(Instruction::PushConst(constant_id));
                                }
                                _ => {
                                    // obj[prop] = value (bracket notation)
                                    self.visit_node(&member.property);
                                }
                            }
                            
                            self.visit_node(&expr.right);  // Push value
                            self.instructions().push(Instruction::SetPropertyAssign);
                        }
                        _ => {
                            // Fallback for other types
                            self.visit_node(&expr.right);
                            self.instructions().push(Instruction::StoreLocal(LocalIndex::new(0)));
                        }
                    }
                }
                "+=" => {
                    // Compound assignment: a += b is equivalent to a = a + b
                    if let Node::Identifier(ident) = &*expr.left {
                        let local_idx = self.get_or_create_local(ident);
                        
                        // Load the current value of the variable
                        self.instructions().push(Instruction::LoadLocal(local_idx));
                        
                        // Generate the right-hand side expression
                        self.visit_node(&expr.right);
                        
                        // Add the values
                        self.instructions().push(Instruction::Add);
                        
                        // Store the result back
                        self.instructions().push(Instruction::StoreLocal(local_idx));
                    } else {
                        self.instructions().push(Instruction::PushUndefined);
                    }
                }
                "-=" => {
                    // Compound assignment: a -= b is equivalent to a = a - b
                    if let Node::Identifier(ident) = &*expr.left {
                        let local_idx = self.get_or_create_local(ident);
                        
                        // Load the current value of the variable
                        self.instructions().push(Instruction::LoadLocal(local_idx));
                        
                        // Generate the right-hand side expression
                        self.visit_node(&expr.right);
                        
                        // Subtract the values
                        self.instructions().push(Instruction::Sub);
                        
                        // Store the result back
                        self.instructions().push(Instruction::StoreLocal(local_idx));
                    } else {
                        self.instructions().push(Instruction::PushUndefined);
                    }
                }
                "*=" => {
                    // Compound assignment: a *= b is equivalent to a = a * b
                    if let Node::Identifier(ident) = &*expr.left {
                        let local_idx = self.get_or_create_local(ident);
                        
                        // Load the current value of the variable
                        self.instructions().push(Instruction::LoadLocal(local_idx));
                        
                        // Generate the right-hand side expression
                        self.visit_node(&expr.right);
                        
                        // Multiply the values
                        self.instructions().push(Instruction::Mul);
                        
                        // Store the result back
                        self.instructions().push(Instruction::StoreLocal(local_idx));
                    } else {
                        self.instructions().push(Instruction::PushUndefined);
                    }
                }
                "/=" => {
                    // Compound assignment: a /= b is equivalent to a = a / b
                    if let Node::Identifier(ident) = &*expr.left {
                        let local_idx = self.get_or_create_local(ident);
                        
                        // Load the current value of the variable
                        self.instructions().push(Instruction::LoadLocal(local_idx));
                        
                        // Generate the right-hand side expression
                        self.visit_node(&expr.right);
                        
                        // Divide the values
                        self.instructions().push(Instruction::Div);
                        
                        // Store the result back
                        self.instructions().push(Instruction::StoreLocal(local_idx));
                    } else {
                        self.instructions().push(Instruction::PushUndefined);
                    }
                }
                _ => {
                    // Unsupported operator, fallback to simple assignment
                    self.visit_node(&expr.right);
                    
                    if let Node::Identifier(ident) = &*expr.left {
                        let local_idx = self.get_or_create_local(ident);
                        self.instructions().push(Instruction::StoreLocal(local_idx));
                    } else {
                        self.instructions().push(Instruction::StoreLocal(LocalIndex::new(0)));
                    }
                }
            }
        }
    }

    fn generate_conditional_expression(&mut self, node: &Node) {
        if let Node::ConditionalExpression(expr) = node {
            self.visit_node(&expr.test);

            let jump_to_alternate = self.instructions().len();
            self.instructions()
                .push(Instruction::JumpIfFalse(CodeAddress::new(0)));

            self.visit_node(&expr.consequent);

            let jump_to_end = self.instructions().len();
            self.instructions()
                .push(Instruction::Jump(CodeAddress::new(0)));

            let alternate_start = self.instructions().len();
            self.instructions()[jump_to_alternate] =
                Instruction::JumpIfFalse(CodeAddress::new(alternate_start));

            self.visit_node(&expr.alternate);

            let end_pos = self.instructions().len();
            self.instructions()[jump_to_end] = Instruction::Jump(CodeAddress::new(end_pos));
        }
    }

    fn generate_call_expression(&mut self, node: &Node) {
        if let Node::CallExpression(expr) = node {
            // Check if this is a built-in function call
            if let Node::MemberExpression(member) = &*expr.callee {
                if let (Node::Identifier(obj_name), Node::Identifier(prop_name)) = 
                    (&*member.object, &*member.property) {
                    
                    // Check for Math functions
                    if obj_name == "Math" {
                        let builtin_name = format!("Math.{}", prop_name);
                        // Push arguments first
                        for arg in &expr.arguments {
                            self.visit_node(arg);
                        }
                        // Call built-in
                        self.instructions().push(Instruction::CallBuiltin(
                            builtin_name,
                            crate::vm::types::ArgIndex::new(expr.arguments.len())
                        ));
                        return;
                    }
                    
                    // Check for String prototype methods
                    if let Node::String(_) = &*member.object {
                        let builtin_name = format!("String.prototype.{}", prop_name);
                        // Push the string first, then arguments
                        self.visit_node(&*member.object);
                        for arg in &expr.arguments {
                            self.visit_node(arg);
                        }
                        // Call built-in
                        self.instructions().push(Instruction::CallBuiltin(
                            builtin_name,
                            crate::vm::types::ArgIndex::new(expr.arguments.len() + 1)
                        ));
                        return;
                    }
                    
                    // Check for Array prototype methods
                    if let Node::Identifier(obj_name) = &*member.object {
                        if self.is_array_variable(obj_name) && (prop_name == "push" || prop_name == "pop") {
                            let builtin_name = format!("Array.prototype.{}", prop_name);
                            // Push the array first, then arguments
                            self.visit_node(&*member.object);
                            for arg in &expr.arguments {
                                self.visit_node(arg);
                            }
                            // Call built-in
                            self.instructions().push(Instruction::CallBuiltin(
                                builtin_name,
                                crate::vm::types::ArgIndex::new(expr.arguments.len() + 1)
                            ));
                            return;
                        }
                    }
                }
            }
            
            // Default call expression handling
            for arg in &expr.arguments {
                self.visit_node(arg);
            }
            self.visit_node(&expr.callee);
            self.instructions()
                .push(Instruction::Call(crate::vm::types::FunctionIndex::new(
                    expr.arguments.len(),
                )));
        }
    }

    fn generate_new_expression(&mut self, node: &Node) {
        if let Node::NewExpression(expr) = node {
            for arg in &expr.arguments {
                self.visit_node(arg);
            }
            self.visit_node(&expr.callee);
            self.instructions().push(Instruction::New);
        }
    }

    fn generate_member_expression(&mut self, node: &Node) {
        if let Node::MemberExpression(expr) = node {
            self.visit_node(&expr.object);
            
            // Handle property names correctly
            match &*expr.property {
                Node::Identifier(name) => {
                    // Push the string value directly for property names
                    let constant_id = <Self as crate::bytecode::scope::constants::ConstantManager>::add_constant(self, name.clone());
                    self.instructions().push(Instruction::PushConst(constant_id));
                }
                _ => {
                    // For other property types, visit normally
                    self.visit_node(&expr.property);
                }
            }
            
            self.instructions().push(Instruction::GetProperty);
        }
    }
}

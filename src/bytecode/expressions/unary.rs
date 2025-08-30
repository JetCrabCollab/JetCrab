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
    T: UnaryCore + crate::bytecode::scope::local_vars::ScopeManager,
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
            // For update expressions on variables, we need to load, modify, and store
            if let Node::Identifier(name) = &*expr.argument {
                let local_idx = self.get_or_create_local(name);

                // Load the current value
                self.instructions().push(Instruction::LoadLocal(local_idx));

                match expr.operator.as_str() {
                    "++" => {
                        if expr.prefix {
                            // ++i: increment, store, and leave incremented value on stack
                            self.instructions().push(Instruction::Inc);
                            self.instructions().push(Instruction::Dup);
                            self.instructions().push(Instruction::StoreLocal(local_idx));
                        } else {
                            // i++: duplicate original, increment, store incremented
                            self.instructions().push(Instruction::Dup);
                            self.instructions().push(Instruction::Inc);
                            self.instructions().push(Instruction::StoreLocal(local_idx));
                        }
                    }
                    "--" => {
                        if expr.prefix {
                            // --i: decrement, store, and leave decremented value on stack
                            self.instructions().push(Instruction::Dec);
                            self.instructions().push(Instruction::Dup);
                            self.instructions().push(Instruction::StoreLocal(local_idx));
                        } else {
                            // i--: duplicate original, decrement, store decremented
                            self.instructions().push(Instruction::Dup);
                            self.instructions().push(Instruction::Dec);
                            self.instructions().push(Instruction::StoreLocal(local_idx));
                        }
                    }
                    _ => {
                        self.instructions().push(Instruction::Inc);
                        self.instructions().push(Instruction::StoreLocal(local_idx));
                    }
                }
            } else {
                // For non-variable expressions, just do the operation
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
}

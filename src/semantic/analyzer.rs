use crate::ast::Node;
use crate::semantic::error::SemanticError;
use crate::semantic::scope::{Scope, VariableInfo};

pub struct SemanticAnalyzer {
    current_scope: Scope,
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            current_scope: Scope::new(),
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, ast: &Node) -> Result<(), Vec<SemanticError>> {
        self.errors.clear();
        self.analyze_node(ast);
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn analyze_node(&mut self, node: &Node) {
        match node {
            Node::Program(program) => {
                for stmt in &program.body {
                    self.analyze_node(stmt);
                }
            }
            Node::VariableDeclaration(decl) => {
                for declarator in &decl.declarations {
                    if let Node::VariableDeclarator(var_decl) = declarator {
                        if let Some(id) = &var_decl.id {
                            if let Node::Identifier(name) = &**id {
                                if let Err(e) = self.current_scope.declare_variable(
                                    name.clone(), 
                                    None
                                ) {
                                    self.errors.push(e);
                                }
                            }
                        }
                    }
                }
            }
            Node::Identifier(name) => {
                if let Err(e) = self.current_scope.use_variable(name) {
                    self.errors.push(e);
                }
            }
            Node::FunctionDeclaration(decl) => {
                if let Some(id) = &decl.id {
                    if let Node::Identifier(name) = &**id {
                        if let Err(e) = self.current_scope.declare_variable(
                            name.clone(), 
                            None
                        ) {
                            self.errors.push(e);
                        }
                    }
                }
                
                let new_scope = Scope::with_parent(self.current_scope.clone());
                let old_scope = std::mem::replace(&mut self.current_scope, new_scope);
                
                self.analyze_node(&decl.body);
                
                self.current_scope = old_scope;
            }
            Node::BlockStatement(block) => {
                let new_scope = Scope::with_parent(self.current_scope.clone());
                let old_scope = std::mem::replace(&mut self.current_scope, new_scope);
                
                for stmt in &block.body {
                    self.analyze_node(stmt);
                }
                
                self.current_scope = old_scope;
            }
            _ => {
                self.analyze_children(node);
            }
        }
    }

    fn analyze_children(&mut self, node: &Node) {
        match node {
            Node::BinaryExpression(expr) => {
                self.analyze_node(&expr.left);
                self.analyze_node(&expr.right);
            }
            Node::AssignmentExpression(expr) => {
                self.analyze_node(&expr.left);
                self.analyze_node(&expr.right);
            }
            Node::CallExpression(expr) => {
                self.analyze_node(&expr.callee);
                for arg in &expr.arguments {
                    self.analyze_node(arg);
                }
            }
            Node::IfStatement(stmt) => {
                self.analyze_node(&stmt.test);
                self.analyze_node(&stmt.consequent);
                if let Some(alt) = &stmt.alternate {
                    self.analyze_node(alt);
                }
            }
            Node::WhileStatement(stmt) => {
                self.analyze_node(&stmt.test);
                self.analyze_node(&stmt.body);
            }
            Node::ForStatement(stmt) => {
                if let Some(init) = &stmt.init {
                    self.analyze_node(init);
                }
                if let Some(test) = &stmt.test {
                    self.analyze_node(test);
                }
                if let Some(update) = &stmt.update {
                    self.analyze_node(update);
                }
                self.analyze_node(&stmt.body);
            }
            _ => {
            }
        }
    }
}

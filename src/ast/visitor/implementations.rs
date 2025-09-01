use crate::ast::*;
use crate::vm::types::{IndentLevel, NodeCount};

pub struct NodeCounter {
    pub count: NodeCount,
}

impl NodeCounter {
    pub fn new() -> Self {
        Self {
            count: NodeCount::new(0),
        }
    }
}

impl Default for NodeCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for NodeCounter {
    type Output = ();

    fn default_output(&self) -> Self::Output {}

    fn visit_node(&mut self, node: &Node) {
        self.count.increment();
        match node {
            Node::Program(program) => {
                for node in &program.body {
                    self.visit_node(node);
                }
            }
            Node::VariableDeclaration(decl) => {
                for var_decl in &decl.declarations {
                    self.visit_node(&var_decl.id);
                    if let Some(init) = &var_decl.init {
                        self.visit_node(init);
                    }
                }
            }
            Node::FunctionDeclaration(decl) => {
                if let Some(id) = &decl.id {
                    self.visit_node(id);
                }
                for param in &decl.params {
                    self.visit_node(param);
                }
                self.visit_node(&decl.body);
            }
            Node::BinaryExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
            }
            Node::UnaryExpression(expr) => {
                self.visit_node(&expr.argument);
            }
            Node::CallExpression(expr) => {
                self.visit_node(&expr.callee);
                for arg in &expr.arguments {
                    self.visit_node(arg);
                }
            }
            Node::MemberExpression(expr) => {
                self.visit_node(&expr.object);
                self.visit_node(&expr.property);
            }
            Node::BlockStatement(stmt) => {
                for node in &stmt.body {
                    self.visit_node(node);
                }
            }
            Node::IfStatement(stmt) => {
                self.visit_node(&stmt.test);
                self.visit_node(&stmt.consequent);
                if let Some(alternate) = &stmt.alternate {
                    self.visit_node(alternate);
                }
            }
            Node::WhileStatement(stmt) => {
                self.visit_node(&stmt.test);
                self.visit_node(&stmt.body);
            }
            Node::ForStatement(stmt) => {
                if let Some(init) = &stmt.init {
                    self.visit_node(init);
                }
                if let Some(test) = &stmt.test {
                    self.visit_node(test);
                }
                if let Some(update) = &stmt.update {
                    self.visit_node(update);
                }
                self.visit_node(&stmt.body);
            }
            Node::ReturnStatement(stmt) => {
                if let Some(argument) = &stmt.argument {
                    self.visit_node(argument);
                }
            }
            Node::ExpressionStatement(stmt) => {
                self.visit_node(&stmt.expression);
            }
            Node::ArrayLiteral(lit) => {
                for elem in lit.elements.iter().flatten() {
                    self.visit_node(elem);
                }
            }
            Node::ObjectLiteral(lit) => {
                for prop in &lit.properties {
                    self.visit_node(prop);
                }
            }
            Node::Property(prop) => {
                self.visit_node(&prop.key);
                self.visit_node(&prop.value);
            }
            _ => {}
        }
    }
}

pub struct AstPrinter {
    pub indent: IndentLevel,
}

impl AstPrinter {
    pub fn new() -> Self {
        Self {
            indent: IndentLevel::new(0),
        }
    }

    fn print_indent(&self) {
        for _ in 0..self.indent.as_usize() {
            print!("  ");
        }
    }
}

impl Default for AstPrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for AstPrinter {
    type Output = ();

    fn default_output(&self) -> Self::Output {}

    fn visit_node(&mut self, node: &Node) {
        self.print_indent();
        match node {
            Node::Program(_) => {}
            Node::VariableDeclaration(_) => {}
            Node::FunctionDeclaration(_) => {}
            Node::BinaryExpression(_) => {}
            Node::UnaryExpression(_) => {}
            Node::CallExpression(_) => {}
            Node::MemberExpression(_) => {}
            Node::BlockStatement(_) => {}
            Node::IfStatement(_) => {}
            Node::WhileStatement(_) => {}
            Node::ForStatement(_) => {}
            Node::ReturnStatement(_) => {}
            Node::ExpressionStatement(_) => {}
            Node::ArrayLiteral(_) => {}
            Node::ObjectLiteral(_) => {}
            Node::Property(_) => {}
            Node::Identifier(_id) => {}
            Node::Number(_num) => {}
            Node::String(_s) => {}
            Node::Boolean(_b) => {}
            Node::Null => {}
            Node::Undefined => {}
            Node::This => {}
            _ => {}
        }

        self.indent += 1;
        match node {
            Node::Program(program) => {
                for node in &program.body {
                    self.visit_node(node);
                }
            }
            Node::VariableDeclaration(decl) => {
                for var_decl in &decl.declarations {
                    self.visit_node(&var_decl.id);
                    if let Some(init) = &var_decl.init {
                        self.visit_node(init);
                    }
                }
            }
            Node::FunctionDeclaration(decl) => {
                if let Some(id) = &decl.id {
                    self.visit_node(id);
                }
                for param in &decl.params {
                    self.visit_node(param);
                }
                self.visit_node(&decl.body);
            }
            Node::BinaryExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
            }
            Node::UnaryExpression(expr) => {
                self.visit_node(&expr.argument);
            }
            Node::CallExpression(expr) => {
                self.visit_node(&expr.callee);
                for arg in &expr.arguments {
                    self.visit_node(arg);
                }
            }
            Node::MemberExpression(expr) => {
                self.visit_node(&expr.object);
                self.visit_node(&expr.property);
            }
            Node::BlockStatement(stmt) => {
                for node in &stmt.body {
                    self.visit_node(node);
                }
            }
            Node::IfStatement(stmt) => {
                self.visit_node(&stmt.test);
                self.visit_node(&stmt.consequent);
                if let Some(alternate) = &stmt.alternate {
                    self.visit_node(alternate);
                }
            }
            Node::WhileStatement(stmt) => {
                self.visit_node(&stmt.test);
                self.visit_node(&stmt.body);
            }
            Node::ForStatement(stmt) => {
                if let Some(init) = &stmt.init {
                    self.visit_node(init);
                }
                if let Some(test) = &stmt.test {
                    self.visit_node(test);
                }
                if let Some(update) = &stmt.update {
                    self.visit_node(update);
                }
                self.visit_node(&stmt.body);
            }
            Node::ReturnStatement(stmt) => {
                if let Some(argument) = &stmt.argument {
                    self.visit_node(argument);
                }
            }
            Node::ExpressionStatement(stmt) => {
                self.visit_node(&stmt.expression);
            }
            Node::ArrayLiteral(lit) => {
                for elem in lit.elements.iter().flatten() {
                    self.visit_node(elem);
                }
            }
            Node::ObjectLiteral(lit) => {
                for prop in &lit.properties {
                    self.visit_node(prop);
                }
            }
            Node::Property(prop) => {
                self.visit_node(&prop.key);
                self.visit_node(&prop.value);
            }
            Node::ForInStatement(stmt) => {
                self.visit_node(&stmt.left);
                self.visit_node(&stmt.right);
                self.visit_node(&stmt.body);
            }
            Node::ForOfStatement(stmt) => {
                self.visit_node(&stmt.left);
                self.visit_node(&stmt.right);
                self.visit_node(&stmt.body);
            }
            _ => {}
        }
        self.indent -= 1;
    }
}

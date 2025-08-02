use crate::ast::Node;
use crate::semantic::errors::SemanticError;
use crate::semantic::scope::Scope;
use crate::semantic::types::Type;
use crate::vm::types::{ColumnNumber, LineNumber, ScopeDepth, VariableCount};
use std::collections::HashMap;

pub struct SemanticAnalyzer {
    scope_stack: Vec<Scope>,
    #[allow(dead_code)]
    type_env: HashMap<String, Type>,
    errors: Vec<SemanticError>,
    #[allow(dead_code)]
    strict_mode: bool,
    scope_depth: ScopeDepth,
    variable_count: VariableCount,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            scope_stack: Vec::new(),
            type_env: HashMap::new(),
            errors: Vec::new(),
            strict_mode: false,
            scope_depth: ScopeDepth::new(0),
            variable_count: VariableCount::new(0),
        };

        analyzer.scope_stack.push(Scope::new());
        analyzer
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    #[allow(dead_code)]
    fn get_line_number(&self, node: &Node) -> LineNumber {
        match node {
            Node::Program(program) => program
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::VariableDeclaration(decl) => decl
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::FunctionDeclaration(decl) => decl
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ClassDeclaration(decl) => decl
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::BinaryExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::UnaryExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::CallExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::NewExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::MemberExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::AssignmentExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ConditionalExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::LogicalExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::UpdateExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::BlockStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::IfStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ForStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::WhileStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::DoWhileStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::SwitchStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::TryStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::CatchClause(clause) => clause
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ThrowStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ReturnStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::BreakStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ContinueStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::LabeledStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::WithStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::DebuggerStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ExpressionStatement(stmt) => stmt
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ArrayLiteral(lit) => lit
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ObjectLiteral(lit) => lit
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::TemplateLiteral(lit) => lit
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::TaggedTemplateExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::Property(prop) => prop
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::SpreadElement(elem) => elem
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::RestElement(elem) => elem
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::Super(super_expr) => super_expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::MetaProperty(prop) => prop
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ArrowFunctionExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::FunctionExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ClassExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::YieldExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::AwaitExpression(expr) => expr
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::RegExp(regexp) => regexp
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ImportDeclaration(decl) => decl
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),
            Node::ExportDeclaration(decl) => decl
                .span
                .as_ref()
                .map(|s| s.start.line)
                .unwrap_or(LineNumber::new(1)),

            Node::Identifier(_) => LineNumber::new(1),
            Node::Number(_) => LineNumber::new(1),
            Node::String(_) => LineNumber::new(1),
            Node::Boolean(_) => LineNumber::new(1),
            Node::Null => LineNumber::new(1),
            Node::Undefined => LineNumber::new(1),
            Node::This => LineNumber::new(1),
            Node::BigInt(_) => LineNumber::new(1),
        }
    }

    pub fn analyze(&mut self, ast: &Node) -> Result<(), SemanticError> {
        self.visit_node(ast)?;

        if !self.errors.is_empty() {
            return Err(self.errors.remove(0));
        }

        Ok(())
    }

    pub fn scope_depth(&self) -> ScopeDepth {
        self.scope_depth
    }

    pub fn variable_count(&self) -> VariableCount {
        self.variable_count
    }

    pub fn enter_scope(&mut self) {
        self.scope_depth.increment();
    }

    pub fn exit_scope(&mut self) {
        self.scope_depth.decrement();
    }

    pub fn increment_variable_count(&mut self) {
        self.variable_count.increment();
    }

    fn visit_node(&mut self, node: &Node) -> Result<Type, SemanticError> {
        match node {
            Node::Program(program) => self.visit_program(program),
            Node::VariableDeclaration(decl) => self.visit_variable_declaration(decl),
            Node::FunctionDeclaration(func) => self.visit_function_declaration(func),
            Node::ExpressionStatement(stmt) => self.visit_expression_statement(stmt),
            Node::BinaryExpression(expr) => self.visit_binary_expression(expr),
            Node::UnaryExpression(expr) => self.visit_unary_expression(expr),
            Node::Identifier(id) => self.visit_identifier(id),
            Node::Number(_) => Ok(Type::Number),
            Node::String(_) => Ok(Type::String),
            Node::Boolean(_) => Ok(Type::Boolean),
            Node::Null => Ok(Type::Null),
            Node::Undefined => Ok(Type::Undefined),
            Node::This => self.visit_this(),
            Node::CallExpression(call) => self.visit_call_expression(call),
            Node::AssignmentExpression(assign) => self.visit_assignment_expression(assign),
            Node::IfStatement(if_stmt) => self.visit_if_statement(if_stmt),
            Node::WhileStatement(while_stmt) => self.visit_while_statement(while_stmt),
            Node::ReturnStatement(return_stmt) => self.visit_return_statement(return_stmt),
            Node::BlockStatement(block) => self.visit_block_statement(block),
            Node::ArrayLiteral(array) => self.visit_array_literal(array),
            Node::ObjectLiteral(obj) => self.visit_object_literal(obj),
            Node::Property(prop) => self.visit_property(prop),
            Node::MemberExpression(member) => self.visit_member_expression(member),
            Node::LogicalExpression(logical) => self.visit_logical_expression(logical),
            Node::ConditionalExpression(conditional) => {
                self.visit_conditional_expression(conditional)
            }
            Node::ArrowFunctionExpression(arrow) => self.visit_arrow_function_expression(arrow),
            _ => Ok(Type::Unknown),
        }
    }

    fn visit_program(&mut self, program: &crate::ast::Program) -> Result<Type, SemanticError> {
        for statement in &program.body {
            self.visit_node(statement)?;
        }
        Ok(Type::Undefined)
    }

    fn visit_variable_declaration(
        &mut self,
        decl: &crate::ast::VariableDeclaration,
    ) -> Result<Type, SemanticError> {
        for var_decl in &decl.declarations {
            if let Node::Identifier(var_name) = &*var_decl.id {
                let var_type = if let Some(init) = &var_decl.init {
                    self.visit_node(init)?
                } else {
                    Type::Undefined
                };

                let current_scope = self.scope_stack.last_mut().unwrap();

                if current_scope.has_variable(var_name) {
                    self.errors.push(SemanticError::DuplicateDeclaration {
                        name: var_name.clone(),
                        position: None,
                    });
                    continue;
                }

                let line_number = decl
                    .span
                    .as_ref()
                    .map(|s| s.start.line)
                    .unwrap_or(LineNumber::new(1));
                current_scope.declare_variable(var_name.clone(), var_type.clone(), line_number);
            }
        }

        Ok(Type::Undefined)
    }

    fn visit_function_declaration(
        &mut self,
        func: &crate::ast::FunctionDeclaration,
    ) -> Result<Type, SemanticError> {
        let func_name = if let Some(id) = &func.id {
            if let Node::Identifier(name) = &**id {
                name.clone()
            } else {
                return Ok(Type::Unknown);
            }
        } else {
            return Ok(Type::Unknown);
        };

        let _current_scope = self.scope_stack.last().unwrap();
        let function_scope = Scope::new();
        self.scope_stack.push(function_scope);

        for param in &func.params {
            if let Node::Identifier(param_name) = param {
                let current_scope = self.scope_stack.last_mut().unwrap();
                let line_number = func
                    .span
                    .as_ref()
                    .map(|s| s.start.line)
                    .unwrap_or(LineNumber::new(1));
                current_scope.declare_variable(param_name.clone(), Type::Unknown, line_number);
            }
        }

        let return_type = self.visit_node(&func.body)?;

        self.scope_stack.pop();

        let current_scope = self.scope_stack.last_mut().unwrap();
        let line_number = func
            .span
            .as_ref()
            .map(|s| s.start.line)
            .unwrap_or(LineNumber::new(1));
        current_scope.declare_variable(
            func_name,
            Type::Function {
                params: vec![],
                return_type: Box::new(Type::Unknown),
            },
            line_number,
        );

        Ok(Type::Function {
            params: vec![],
            return_type: Box::new(return_type),
        })
    }

    fn visit_expression_statement(
        &mut self,
        stmt: &crate::ast::ExpressionStatement,
    ) -> Result<Type, SemanticError> {
        self.visit_node(&stmt.expression)
    }

    fn visit_binary_expression(
        &mut self,
        expr: &crate::ast::BinaryExpression,
    ) -> Result<Type, SemanticError> {
        let left_type = self.visit_node(&expr.left)?;
        let right_type = self.visit_node(&expr.right)?;

        match expr.operator.as_str() {
            "+" => {
                if left_type == Type::String || right_type == Type::String {
                    Ok(Type::String)
                } else if left_type == Type::Number && right_type == Type::Number {
                    Ok(Type::Number)
                } else {
                    Ok(Type::String)
                }
            }
            "-" | "*" | "/" | "%" => {
                if left_type != Type::Number || right_type != Type::Number {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "number".to_string(),
                        found: format!("{left_type:?} and {right_type:?}"),
                        position: None,
                    });
                }
                Ok(Type::Number)
            }
            "==" | "!=" | "===" | "!==" => Ok(Type::Boolean),
            "<" | ">" | "<=" | ">=" => {
                if left_type != Type::Number || right_type != Type::Number {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "number".to_string(),
                        found: format!("{left_type:?} and {right_type:?}"),
                        position: None,
                    });
                }
                Ok(Type::Boolean)
            }
            "&&" | "||" => {
                if left_type != Type::Boolean || right_type != Type::Boolean {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "boolean".to_string(),
                        found: format!("{left_type:?} and {right_type:?}"),
                        position: None,
                    });
                }
                Ok(Type::Boolean)
            }
            _ => Ok(Type::Unknown),
        }
    }

    fn visit_unary_expression(
        &mut self,
        expr: &crate::ast::UnaryExpression,
    ) -> Result<Type, SemanticError> {
        let operand_type = self.visit_node(&expr.argument)?;

        match expr.operator.as_str() {
            "!" => {
                if operand_type != Type::Boolean {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "boolean".to_string(),
                        found: format!("{operand_type:?}"),
                        position: None,
                    });
                }
                Ok(Type::Boolean)
            }
            "+" | "-" => {
                if operand_type != Type::Number {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "number".to_string(),
                        found: format!("{operand_type:?}"),
                        position: None,
                    });
                }
                Ok(Type::Number)
            }
            _ => Ok(Type::Unknown),
        }
    }

    fn visit_identifier(&mut self, id: &str) -> Result<Type, SemanticError> {
        let current_scope = self.scope_stack.last().unwrap();

        if let Some(var_type) = current_scope.get_variable_type(id) {
            Ok(var_type)
        } else {
            self.errors.push(SemanticError::UndefinedVariable(
                id.to_string(),
                LineNumber::new(0),
                ColumnNumber::new(0),
            ));
            Ok(Type::Unknown)
        }
    }

    fn visit_this(&mut self) -> Result<Type, SemanticError> {
        Ok(Type::Object)
    }

    fn visit_call_expression(
        &mut self,
        call: &crate::ast::CallExpression,
    ) -> Result<Type, SemanticError> {
        if let Node::Identifier(func_name) = &*call.callee {
            let current_scope = self.scope_stack.last().unwrap();

            if let Some(_func_type) = current_scope.get_variable_type(func_name) {
                for arg in &call.arguments {
                    self.visit_node(arg)?;
                }

                Ok(Type::Unknown)
            } else {
                self.errors.push(SemanticError::UndefinedVariable(
                    func_name.clone(),
                    LineNumber::new(0),
                    ColumnNumber::new(0),
                ));
                Ok(Type::Unknown)
            }
        } else {
            let callee_type = self.visit_node(&call.callee)?;

            for arg in &call.arguments {
                self.visit_node(arg)?;
            }

            if !matches!(callee_type, Type::Function { .. }) {
                self.errors.push(SemanticError::TypeMismatch {
                    expected: "function".to_string(),
                    found: format!("{callee_type:?}"),
                    position: None,
                });
            }
            Ok(Type::Unknown)
        }
    }

    fn visit_assignment_expression(
        &mut self,
        assign: &crate::ast::AssignmentExpression,
    ) -> Result<Type, SemanticError> {
        let value_type = self.visit_node(&assign.right)?;

        if let Node::Identifier(var_name) = &*assign.left {
            let current_scope = self.scope_stack.last().unwrap();

            if current_scope.get_variable_type(var_name).is_none() {
                self.errors.push(SemanticError::UndefinedVariable(
                    var_name.clone(),
                    LineNumber::new(0),
                    ColumnNumber::new(0),
                ));
            }
        }

        Ok(value_type)
    }

    fn visit_if_statement(
        &mut self,
        if_stmt: &crate::ast::IfStatement,
    ) -> Result<Type, SemanticError> {
        let condition_type = self.visit_node(&if_stmt.test)?;

        if condition_type != Type::Boolean {
            self.errors.push(SemanticError::TypeMismatch {
                expected: "boolean".to_string(),
                found: format!("{condition_type:?}"),
                position: None,
            });
        }

        let _current_scope = self.scope_stack.last().unwrap();
        let block_scope = Scope::new();
        self.scope_stack.push(block_scope);
        self.visit_node(&if_stmt.consequent)?;
        self.scope_stack.pop();

        if let Some(alternate) = &if_stmt.alternate {
            let _current_scope = self.scope_stack.last().unwrap();
            let block_scope = Scope::new();
            self.scope_stack.push(block_scope);
            self.visit_node(alternate)?;
            self.scope_stack.pop();
        }

        Ok(Type::Undefined)
    }

    fn visit_while_statement(
        &mut self,
        while_stmt: &crate::ast::WhileStatement,
    ) -> Result<Type, SemanticError> {
        let condition_type = self.visit_node(&while_stmt.test)?;

        if condition_type != Type::Boolean {
            self.errors.push(SemanticError::TypeMismatch {
                expected: "boolean".to_string(),
                found: format!("{condition_type:?}"),
                position: None,
            });
        }

        let _current_scope = self.scope_stack.last().unwrap();
        let block_scope = Scope::new();
        self.scope_stack.push(block_scope);
        self.visit_node(&while_stmt.body)?;
        self.scope_stack.pop();

        Ok(Type::Undefined)
    }

    fn visit_return_statement(
        &mut self,
        return_stmt: &crate::ast::ReturnStatement,
    ) -> Result<Type, SemanticError> {
        if let Some(argument) = &return_stmt.argument {
            self.visit_node(argument)
        } else {
            Ok(Type::Undefined)
        }
    }

    fn visit_block_statement(
        &mut self,
        block: &crate::ast::BlockStatement,
    ) -> Result<Type, SemanticError> {
        let _current_scope = self.scope_stack.last().unwrap();
        let block_scope = Scope::new();
        self.scope_stack.push(block_scope);

        let mut last_type = Type::Undefined;

        for statement in &block.body {
            last_type = self.visit_node(statement)?;
        }

        self.scope_stack.pop();

        Ok(last_type)
    }

    fn visit_array_literal(
        &mut self,
        array: &crate::ast::ArrayLiteral,
    ) -> Result<Type, SemanticError> {
        let _element_types: Vec<_> = array
            .elements
            .iter()
            .flatten()
            .map(|elem| self.visit_node(elem))
            .collect::<Result<_, _>>()?;
        Ok(Type::Array(Box::new(Type::Unknown)))
    }

    fn visit_object_literal(
        &mut self,
        obj: &crate::ast::ObjectLiteral,
    ) -> Result<Type, SemanticError> {
        for property in &obj.properties {
            self.visit_node(property)?;
        }
        Ok(Type::Object)
    }

    fn visit_property(&mut self, prop: &crate::ast::Property) -> Result<Type, SemanticError> {
        let _key_type = match &*prop.key {
            Node::Identifier(_) => Ok(Type::String),
            _ => self.visit_node(&prop.key),
        }?;

        let value_type = self.visit_node(&prop.value)?;

        Ok(value_type)
    }

    fn visit_member_expression(
        &mut self,
        member: &crate::ast::MemberExpression,
    ) -> Result<Type, SemanticError> {
        let _object_type = self.visit_node(&member.object)?;

        let _property_type = match &*member.property {
            Node::Identifier(_) => Ok(Type::String),
            _ => self.visit_node(&member.property),
        }?;

        Ok(Type::Unknown)
    }

    fn visit_logical_expression(
        &mut self,
        logical: &crate::ast::LogicalExpression,
    ) -> Result<Type, SemanticError> {
        let left_type = self.visit_node(&logical.left)?;
        let _right_type = self.visit_node(&logical.right)?;

        match logical.operator.as_str() {
            "&&" | "||" => Ok(left_type),
            _ => Ok(Type::Boolean),
        }
    }

    fn visit_conditional_expression(
        &mut self,
        conditional: &crate::ast::ConditionalExpression,
    ) -> Result<Type, SemanticError> {
        let test_type = self.visit_node(&conditional.test)?;

        if test_type != Type::Boolean {
            self.errors.push(SemanticError::TypeMismatch {
                expected: "boolean".to_string(),
                found: format!("{test_type:?}"),
                position: None,
            });
        }

        let consequent_type = self.visit_node(&conditional.consequent)?;
        let _alternate_type = self.visit_node(&conditional.alternate)?;

        Ok(consequent_type)
    }

    fn visit_arrow_function_expression(
        &mut self,
        arrow: &crate::ast::ArrowFunctionExpression,
    ) -> Result<Type, SemanticError> {
        let _current_scope = self.scope_stack.last().unwrap();
        let function_scope = Scope::new();
        self.scope_stack.push(function_scope);

        for param in &arrow.params {
            if let Node::Identifier(param_name) = param {
                let current_scope = self.scope_stack.last_mut().unwrap();
                let line_number = arrow
                    .span
                    .as_ref()
                    .map(|s| s.start.line)
                    .unwrap_or(LineNumber::new(1));
                current_scope.declare_variable(param_name.clone(), Type::Unknown, line_number);
            }
        }

        let _return_type = self.visit_node(&arrow.body)?;

        self.scope_stack.pop();

        Ok(Type::Function {
            params: vec![],
            return_type: Box::new(Type::Unknown),
        })
    }
}

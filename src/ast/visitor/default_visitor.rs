use crate::ast::statements::control_flow::{ForInStatement, ForOfStatement};
use crate::ast::*;
use crate::vm::types::NodeCount;

pub struct DefaultVisitor;

impl Visitor for DefaultVisitor {
    type Output = ();

    fn visit_program(&mut self, program: &Program) -> Self::Output {
        for statement in &program.body {
            self.visit_node(statement);
        }
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) -> Self::Output {
        for declarator in &decl.declarations {
            self.visit_node(&declarator.id);
            if let Some(init) = &declarator.init {
                self.visit_node(init);
            }
        }
    }

    fn visit_function_declaration(&mut self, decl: &FunctionDeclaration) -> Self::Output {
        if let Some(id) = &decl.id {
            self.visit_node(id);
        }
        for param in &decl.params {
            self.visit_node(param);
        }
        self.visit_node(&decl.body);
    }

    fn visit_class_declaration(&mut self, decl: &ClassDeclaration) -> Self::Output {
        if let Some(id) = &decl.id {
            self.visit_node(id);
        }
        if let Some(super_class) = &decl.super_class {
            self.visit_node(super_class);
        }
        self.visit_node(&decl.body);
    }

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) -> Self::Output {
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
    }

    fn visit_unary_expression(&mut self, expr: &UnaryExpression) -> Self::Output {
        self.visit_node(&expr.argument);
    }

    fn visit_call_expression(&mut self, expr: &CallExpression) -> Self::Output {
        self.visit_node(&expr.callee);
        for arg in &expr.arguments {
            self.visit_node(arg);
        }
    }

    fn visit_new_expression(&mut self, expr: &NewExpression) -> Self::Output {
        self.visit_node(&expr.callee);
        for arg in &expr.arguments {
            self.visit_node(arg);
        }
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression) -> Self::Output {
        self.visit_node(&expr.object);
        if expr.computed {
            self.visit_node(&expr.property);
        }
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression) -> Self::Output {
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
    }

    fn visit_conditional_expression(&mut self, expr: &ConditionalExpression) -> Self::Output {
        self.visit_node(&expr.test);
        self.visit_node(&expr.consequent);
        self.visit_node(&expr.alternate);
    }

    fn visit_logical_expression(&mut self, expr: &LogicalExpression) -> Self::Output {
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression) -> Self::Output {
        self.visit_node(&expr.argument);
    }

    fn visit_block_statement(&mut self, stmt: &BlockStatement) -> Self::Output {
        for statement in &stmt.body {
            self.visit_node(statement);
        }
    }

    fn visit_if_statement(&mut self, stmt: &IfStatement) -> Self::Output {
        self.visit_node(&stmt.test);
        self.visit_node(&stmt.consequent);
        if let Some(alternate) = &stmt.alternate {
            self.visit_node(alternate);
        }
    }

    fn visit_for_statement(&mut self, stmt: &ForStatement) -> Self::Output {
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

    fn visit_for_in_statement(&mut self, stmt: &ForInStatement) -> Self::Output {
        self.visit_node(&stmt.left);
        self.visit_node(&stmt.right);
        self.visit_node(&stmt.body);
    }

    fn visit_for_of_statement(&mut self, stmt: &ForOfStatement) -> Self::Output {
        self.visit_node(&stmt.left);
        self.visit_node(&stmt.right);
        self.visit_node(&stmt.body);
    }

    fn visit_while_statement(&mut self, stmt: &WhileStatement) -> Self::Output {
        self.visit_node(&stmt.test);
        self.visit_node(&stmt.body);
    }

    fn visit_do_while_statement(&mut self, stmt: &DoWhileStatement) -> Self::Output {
        self.visit_node(&stmt.body);
        self.visit_node(&stmt.test);
    }

    fn visit_switch_statement(&mut self, stmt: &SwitchStatement) -> Self::Output {
        self.visit_node(&stmt.discriminant);
        for case in &stmt.cases {
            if let Some(test) = &case.test {
                self.visit_node(test);
            }
            for consequent in &case.consequent {
                self.visit_node(consequent);
            }
        }
    }

    fn visit_try_statement(&mut self, stmt: &TryStatement) -> Self::Output {
        self.visit_node(&stmt.block);
        if let Some(handler) = &stmt.handler {
            self.visit_node(handler);
        }
        if let Some(finalizer) = &stmt.finalizer {
            self.visit_node(finalizer);
        }
    }

    fn visit_catch_clause(&mut self, clause: &CatchClause) -> Self::Output {
        self.visit_node(&clause.param);
        self.visit_node(&clause.body);
    }

    fn visit_throw_statement(&mut self, stmt: &ThrowStatement) -> Self::Output {
        self.visit_node(&stmt.argument);
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStatement) -> Self::Output {
        if let Some(argument) = &stmt.argument {
            self.visit_node(argument);
        }
    }

    fn visit_break_statement(&mut self, _stmt: &BreakStatement) -> Self::Output {}

    fn visit_continue_statement(&mut self, _stmt: &ContinueStatement) -> Self::Output {}

    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement) -> Self::Output {
        self.visit_node(&stmt.expression);
    }

    fn visit_array_literal(&mut self, lit: &ArrayLiteral) -> Self::Output {
        for elem in lit.elements.iter().flatten() {
            self.visit_node(elem);
        }
    }

    fn visit_object_literal(&mut self, lit: &ObjectLiteral) -> Self::Output {
        for property in &lit.properties {
            self.visit_node(property);
        }
    }

    fn visit_property(&mut self, prop: &Property) -> Self::Output {
        if prop.computed {
            self.visit_node(&prop.key);
        }
        self.visit_node(&prop.value);
    }

    fn visit_identifier(&mut self, _id: &str) -> Self::Output {}

    fn visit_number(&mut self, _num: f64) -> Self::Output {}

    fn visit_string(&mut self, _s: &str) -> Self::Output {}

    fn visit_boolean(&mut self, _b: bool) -> Self::Output {}

    fn visit_null(&mut self) -> Self::Output {}

    fn visit_undefined(&mut self) -> Self::Output {}

    fn visit_this(&mut self) -> Self::Output {}

    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression) -> Self::Output {
        for param in &expr.params {
            self.visit_node(param);
        }
        self.visit_node(&expr.body);
    }

    fn visit_function_expression(&mut self, expr: &FunctionExpression) -> Self::Output {
        if let Some(id) = &expr.id {
            self.visit_node(id);
        }
        for param in &expr.params {
            self.visit_node(param);
        }
        self.visit_node(&expr.body);
    }

    fn visit_class_expression(&mut self, expr: &ClassExpression) -> Self::Output {
        if let Some(id) = &expr.id {
            self.visit_node(id);
        }
        if let Some(super_class) = &expr.super_class {
            self.visit_node(super_class);
        }
        self.visit_node(&expr.body);
    }

    fn visit_yield_expression(&mut self, expr: &YieldExpression) -> Self::Output {
        if let Some(argument) = &expr.argument {
            self.visit_node(argument);
        }
    }

    fn visit_await_expression(&mut self, expr: &AwaitExpression) -> Self::Output {
        self.visit_node(&expr.argument);
    }

    fn visit_super(&mut self, _super_expr: &Super) -> Self::Output {}

    fn visit_meta_property(&mut self, _prop: &MetaProperty) -> Self::Output {}

    fn visit_spread_element(&mut self, elem: &SpreadElement) -> Self::Output {
        self.visit_node(&elem.argument);
    }

    fn visit_rest_element(&mut self, elem: &RestElement) -> Self::Output {
        self.visit_node(&elem.argument);
    }

    fn visit_template_literal(&mut self, lit: &TemplateLiteral) -> Self::Output {
        for expr in &lit.expressions {
            self.visit_node(expr);
        }
    }

    fn visit_tagged_template_expression(
        &mut self,
        expr: &TaggedTemplateExpression,
    ) -> Self::Output {
        self.visit_node(&expr.tag);
        self.visit_node(&expr.quasi);
    }

    fn visit_import_declaration(&mut self, decl: &ImportDeclaration) -> Self::Output {
        for specifier in &decl.specifiers {
            self.visit_node(specifier);
        }
    }

    fn visit_export_declaration(&mut self, decl: &ExportDeclaration) -> Self::Output {
        if let Some(declaration) = &decl.declaration {
            self.visit_node(declaration);
        }
        for specifier in &decl.specifiers {
            self.visit_node(specifier);
        }
        if let Some(source) = &decl.source {
            self.visit_node(source);
        }
    }

    fn visit_labeled_statement(&mut self, stmt: &LabeledStatement) -> Self::Output {
        self.visit_node(&stmt.label);
        self.visit_node(&stmt.body);
    }

    fn visit_with_statement(&mut self, stmt: &WithStatement) -> Self::Output {
        self.visit_node(&stmt.object);
        self.visit_node(&stmt.body);
    }

    fn visit_debugger_statement(&mut self, _stmt: &DebuggerStatement) -> Self::Output {}

    fn visit_bigint(&mut self, _bigint: &str) -> Self::Output {}

    fn visit_regexp(&mut self, _regexp: &RegExp) -> Self::Output {}
}

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

    fn visit_node(&mut self, node: &Node) {
        self.count.increment();
        match node {
            Node::Program(program) => self.visit_program(program),
            Node::VariableDeclaration(decl) => self.visit_variable_declaration(decl),
            Node::FunctionDeclaration(decl) => self.visit_function_declaration(decl),
            Node::ClassDeclaration(decl) => self.visit_class_declaration(decl),
            Node::BinaryExpression(expr) => self.visit_binary_expression(expr),
            Node::UnaryExpression(expr) => self.visit_unary_expression(expr),
            Node::CallExpression(expr) => self.visit_call_expression(expr),
            Node::NewExpression(expr) => self.visit_new_expression(expr),
            Node::MemberExpression(expr) => self.visit_member_expression(expr),
            Node::AssignmentExpression(expr) => self.visit_assignment_expression(expr),
            Node::ConditionalExpression(expr) => self.visit_conditional_expression(expr),
            Node::LogicalExpression(expr) => self.visit_logical_expression(expr),
            Node::UpdateExpression(expr) => self.visit_update_expression(expr),
            Node::BlockStatement(stmt) => self.visit_block_statement(stmt),
            Node::IfStatement(stmt) => self.visit_if_statement(stmt),
            Node::ForStatement(stmt) => self.visit_for_statement(stmt),
            Node::ForInStatement(stmt) => self.visit_for_in_statement(stmt),
            Node::ForOfStatement(stmt) => self.visit_for_of_statement(stmt),
            Node::WhileStatement(stmt) => self.visit_while_statement(stmt),
            Node::DoWhileStatement(stmt) => self.visit_do_while_statement(stmt),
            Node::SwitchStatement(stmt) => self.visit_switch_statement(stmt),
            Node::TryStatement(stmt) => self.visit_try_statement(stmt),
            Node::CatchClause(clause) => self.visit_catch_clause(clause),
            Node::ThrowStatement(stmt) => self.visit_throw_statement(stmt),
            Node::ReturnStatement(stmt) => self.visit_return_statement(stmt),
            Node::BreakStatement(stmt) => self.visit_break_statement(stmt),
            Node::ContinueStatement(stmt) => self.visit_continue_statement(stmt),
            Node::ExpressionStatement(stmt) => self.visit_expression_statement(stmt),
            Node::ArrayLiteral(lit) => self.visit_array_literal(lit),
            Node::ObjectLiteral(lit) => self.visit_object_literal(lit),
            Node::Property(prop) => self.visit_property(prop),
            Node::Identifier(id) => self.visit_identifier(id),
            Node::Number(num) => self.visit_number(*num),
            Node::String(s) => self.visit_string(s),
            Node::Boolean(b) => self.visit_boolean(*b),
            Node::Null => self.visit_null(),
            Node::Undefined => self.visit_undefined(),
            Node::This => self.visit_this(),
            Node::ArrowFunctionExpression(expr) => self.visit_arrow_function_expression(expr),
            Node::FunctionExpression(expr) => self.visit_function_expression(expr),
            Node::ClassExpression(expr) => self.visit_class_expression(expr),
            Node::YieldExpression(expr) => self.visit_yield_expression(expr),
            Node::AwaitExpression(expr) => self.visit_await_expression(expr),
            Node::Super(super_expr) => self.visit_super(super_expr),
            Node::MetaProperty(prop) => self.visit_meta_property(prop),
            Node::SpreadElement(elem) => self.visit_spread_element(elem),
            Node::RestElement(elem) => self.visit_rest_element(elem),
            Node::TemplateLiteral(lit) => self.visit_template_literal(lit),
            Node::TaggedTemplateExpression(expr) => self.visit_tagged_template_expression(expr),
            Node::ImportDeclaration(decl) => self.visit_import_declaration(decl),
            Node::ExportDeclaration(decl) => self.visit_export_declaration(decl),
            Node::LabeledStatement(stmt) => self.visit_labeled_statement(stmt),
            Node::WithStatement(stmt) => self.visit_with_statement(stmt),
            Node::DebuggerStatement(stmt) => self.visit_debugger_statement(stmt),
            Node::BigInt(bigint) => self.visit_bigint(bigint),
            Node::RegExp(regexp) => self.visit_regexp(regexp),
        }
    }

    fn visit_program(&mut self, program: &Program) -> Self::Output {
        for statement in &program.body {
            self.visit_node(statement);
        }
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) -> Self::Output {
        for declarator in &decl.declarations {
            self.visit_node(&declarator.id);
            if let Some(init) = &declarator.init {
                self.visit_node(init);
            }
        }
    }

    fn visit_function_declaration(&mut self, decl: &FunctionDeclaration) -> Self::Output {
        if let Some(id) = &decl.id {
            self.visit_node(id);
        }
        for param in &decl.params {
            self.visit_node(param);
        }
        self.visit_node(&decl.body);
    }

    fn visit_class_declaration(&mut self, decl: &ClassDeclaration) -> Self::Output {
        if let Some(id) = &decl.id {
            self.visit_node(id);
        }
        if let Some(super_class) = &decl.super_class {
            self.visit_node(super_class);
        }
        self.visit_node(&decl.body);
    }

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) -> Self::Output {
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
    }

    fn visit_unary_expression(&mut self, expr: &UnaryExpression) -> Self::Output {
        self.visit_node(&expr.argument);
    }

    fn visit_call_expression(&mut self, expr: &CallExpression) -> Self::Output {
        self.visit_node(&expr.callee);
        for arg in &expr.arguments {
            self.visit_node(arg);
        }
    }

    fn visit_new_expression(&mut self, expr: &NewExpression) -> Self::Output {
        self.visit_node(&expr.callee);
        for arg in &expr.arguments {
            self.visit_node(arg);
        }
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression) -> Self::Output {
        self.visit_node(&expr.object);
        if expr.computed {
            self.visit_node(&expr.property);
        }
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression) -> Self::Output {
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
    }

    fn visit_conditional_expression(&mut self, expr: &ConditionalExpression) -> Self::Output {
        self.visit_node(&expr.test);
        self.visit_node(&expr.consequent);
        self.visit_node(&expr.alternate);
    }

    fn visit_logical_expression(&mut self, expr: &LogicalExpression) -> Self::Output {
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression) -> Self::Output {
        self.visit_node(&expr.argument);
    }

    fn visit_block_statement(&mut self, stmt: &BlockStatement) -> Self::Output {
        for statement in &stmt.body {
            self.visit_node(statement);
        }
    }

    fn visit_if_statement(&mut self, stmt: &IfStatement) -> Self::Output {
        self.visit_node(&stmt.test);
        self.visit_node(&stmt.consequent);
        if let Some(alternate) = &stmt.alternate {
            self.visit_node(alternate);
        }
    }

    fn visit_for_statement(&mut self, stmt: &ForStatement) -> Self::Output {
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

    fn visit_for_in_statement(&mut self, stmt: &ForInStatement) -> Self::Output {
        self.visit_node(&stmt.left);
        self.visit_node(&stmt.right);
        self.visit_node(&stmt.body);
    }

    fn visit_for_of_statement(&mut self, stmt: &ForOfStatement) -> Self::Output {
        self.visit_node(&stmt.left);
        self.visit_node(&stmt.right);
        self.visit_node(&stmt.body);
    }

    fn visit_while_statement(&mut self, stmt: &WhileStatement) -> Self::Output {
        self.visit_node(&stmt.test);
        self.visit_node(&stmt.body);
    }

    fn visit_do_while_statement(&mut self, stmt: &DoWhileStatement) -> Self::Output {
        self.visit_node(&stmt.body);
        self.visit_node(&stmt.test);
    }

    fn visit_switch_statement(&mut self, stmt: &SwitchStatement) -> Self::Output {
        self.visit_node(&stmt.discriminant);
        for case in &stmt.cases {
            if let Some(test) = &case.test {
                self.visit_node(test);
            }
            for consequent in &case.consequent {
                self.visit_node(consequent);
            }
        }
    }

    fn visit_try_statement(&mut self, stmt: &TryStatement) -> Self::Output {
        self.visit_node(&stmt.block);
        if let Some(handler) = &stmt.handler {
            self.visit_node(handler);
        }
        if let Some(finalizer) = &stmt.finalizer {
            self.visit_node(finalizer);
        }
    }

    fn visit_catch_clause(&mut self, clause: &CatchClause) -> Self::Output {
        self.visit_node(&clause.param);
        self.visit_node(&clause.body);
    }

    fn visit_throw_statement(&mut self, stmt: &ThrowStatement) -> Self::Output {
        self.visit_node(&stmt.argument);
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStatement) -> Self::Output {
        if let Some(argument) = &stmt.argument {
            self.visit_node(argument);
        }
    }

    fn visit_break_statement(&mut self, _stmt: &BreakStatement) -> Self::Output {}

    fn visit_continue_statement(&mut self, _stmt: &ContinueStatement) -> Self::Output {}

    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement) -> Self::Output {
        self.visit_node(&stmt.expression);
    }

    fn visit_array_literal(&mut self, lit: &ArrayLiteral) -> Self::Output {
        for elem in lit.elements.iter().flatten() {
            self.visit_node(elem);
        }
    }

    fn visit_object_literal(&mut self, lit: &ObjectLiteral) -> Self::Output {
        for property in &lit.properties {
            self.visit_node(property);
        }
    }

    fn visit_property(&mut self, prop: &Property) -> Self::Output {
        if prop.computed {
            self.visit_node(&prop.key);
        }
        self.visit_node(&prop.value);
    }

    fn visit_identifier(&mut self, _id: &str) -> Self::Output {}

    fn visit_number(&mut self, _num: f64) -> Self::Output {}

    fn visit_string(&mut self, _s: &str) -> Self::Output {}

    fn visit_boolean(&mut self, _b: bool) -> Self::Output {}

    fn visit_null(&mut self) -> Self::Output {}

    fn visit_undefined(&mut self) -> Self::Output {}

    fn visit_this(&mut self) -> Self::Output {}

    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression) -> Self::Output {
        for param in &expr.params {
            self.visit_node(param);
        }
        self.visit_node(&expr.body);
    }

    fn visit_function_expression(&mut self, expr: &FunctionExpression) -> Self::Output {
        if let Some(id) = &expr.id {
            self.visit_node(id);
        }
        for param in &expr.params {
            self.visit_node(param);
        }
        self.visit_node(&expr.body);
    }

    fn visit_class_expression(&mut self, expr: &ClassExpression) -> Self::Output {
        if let Some(id) = &expr.id {
            self.visit_node(id);
        }
        if let Some(super_class) = &expr.super_class {
            self.visit_node(super_class);
        }
        self.visit_node(&expr.body);
    }

    fn visit_yield_expression(&mut self, expr: &YieldExpression) -> Self::Output {
        if let Some(argument) = &expr.argument {
            self.visit_node(argument);
        }
    }

    fn visit_await_expression(&mut self, expr: &AwaitExpression) -> Self::Output {
        self.visit_node(&expr.argument);
    }

    fn visit_super(&mut self, _super_expr: &Super) -> Self::Output {}

    fn visit_meta_property(&mut self, _prop: &MetaProperty) -> Self::Output {}

    fn visit_spread_element(&mut self, elem: &SpreadElement) -> Self::Output {
        self.visit_node(&elem.argument);
    }

    fn visit_rest_element(&mut self, elem: &RestElement) -> Self::Output {
        self.visit_node(&elem.argument);
    }

    fn visit_template_literal(&mut self, lit: &TemplateLiteral) -> Self::Output {
        for expr in &lit.expressions {
            self.visit_node(expr);
        }
    }

    fn visit_tagged_template_expression(
        &mut self,
        expr: &TaggedTemplateExpression,
    ) -> Self::Output {
        self.visit_node(&expr.tag);
        self.visit_node(&expr.quasi);
    }

    fn visit_import_declaration(&mut self, decl: &ImportDeclaration) -> Self::Output {
        for specifier in &decl.specifiers {
            self.visit_node(specifier);
        }
    }

    fn visit_export_declaration(&mut self, decl: &ExportDeclaration) -> Self::Output {
        if let Some(declaration) = &decl.declaration {
            self.visit_node(declaration);
        }
        for specifier in &decl.specifiers {
            self.visit_node(specifier);
        }
        if let Some(source) = &decl.source {
            self.visit_node(source);
        }
    }

    fn visit_labeled_statement(&mut self, stmt: &LabeledStatement) -> Self::Output {
        self.visit_node(&stmt.label);
        self.visit_node(&stmt.body);
    }

    fn visit_with_statement(&mut self, stmt: &WithStatement) -> Self::Output {
        self.visit_node(&stmt.object);
        self.visit_node(&stmt.body);
    }

    fn visit_debugger_statement(&mut self, _stmt: &DebuggerStatement) -> Self::Output {}

    fn visit_bigint(&mut self, _bigint: &str) -> Self::Output {}

    fn visit_regexp(&mut self, _regexp: &RegExp) -> Self::Output {}
}

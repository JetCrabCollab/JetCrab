use crate::ast::statements::control_flow::{ForInStatement, ForOfStatement};
use crate::ast::*;

pub mod default_visitor;
pub use default_visitor::{DefaultVisitor, NodeCounter};

pub trait Visitor {
    type Output;

    fn visit_node(&mut self, node: &Node) -> Self::Output {
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

    fn visit_program(&mut self, program: &Program) -> Self::Output;
    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) -> Self::Output;
    fn visit_function_declaration(&mut self, decl: &FunctionDeclaration) -> Self::Output;
    fn visit_class_declaration(&mut self, decl: &ClassDeclaration) -> Self::Output;
    fn visit_binary_expression(&mut self, expr: &BinaryExpression) -> Self::Output;
    fn visit_unary_expression(&mut self, expr: &UnaryExpression) -> Self::Output;
    fn visit_call_expression(&mut self, expr: &CallExpression) -> Self::Output;
    fn visit_new_expression(&mut self, expr: &NewExpression) -> Self::Output;
    fn visit_member_expression(&mut self, expr: &MemberExpression) -> Self::Output;
    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression) -> Self::Output;
    fn visit_conditional_expression(&mut self, expr: &ConditionalExpression) -> Self::Output;
    fn visit_logical_expression(&mut self, expr: &LogicalExpression) -> Self::Output;
    fn visit_update_expression(&mut self, expr: &UpdateExpression) -> Self::Output;
    fn visit_block_statement(&mut self, stmt: &BlockStatement) -> Self::Output;
    fn visit_if_statement(&mut self, stmt: &IfStatement) -> Self::Output;
    fn visit_for_statement(&mut self, stmt: &ForStatement) -> Self::Output;
    fn visit_for_in_statement(&mut self, stmt: &ForInStatement) -> Self::Output;
    fn visit_for_of_statement(&mut self, stmt: &ForOfStatement) -> Self::Output;
    fn visit_while_statement(&mut self, stmt: &WhileStatement) -> Self::Output;
    fn visit_do_while_statement(&mut self, stmt: &DoWhileStatement) -> Self::Output;
    fn visit_switch_statement(&mut self, stmt: &SwitchStatement) -> Self::Output;
    fn visit_try_statement(&mut self, stmt: &TryStatement) -> Self::Output;
    fn visit_catch_clause(&mut self, clause: &CatchClause) -> Self::Output;
    fn visit_throw_statement(&mut self, stmt: &ThrowStatement) -> Self::Output;
    fn visit_return_statement(&mut self, stmt: &ReturnStatement) -> Self::Output;
    fn visit_break_statement(&mut self, stmt: &BreakStatement) -> Self::Output;
    fn visit_continue_statement(&mut self, stmt: &ContinueStatement) -> Self::Output;
    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement) -> Self::Output;
    fn visit_array_literal(&mut self, lit: &ArrayLiteral) -> Self::Output;
    fn visit_object_literal(&mut self, lit: &ObjectLiteral) -> Self::Output;
    fn visit_property(&mut self, prop: &Property) -> Self::Output;
    fn visit_identifier(&mut self, id: &str) -> Self::Output;
    fn visit_number(&mut self, num: f64) -> Self::Output;
    fn visit_string(&mut self, s: &str) -> Self::Output;
    fn visit_boolean(&mut self, b: bool) -> Self::Output;
    fn visit_null(&mut self) -> Self::Output;
    fn visit_undefined(&mut self) -> Self::Output;
    fn visit_this(&mut self) -> Self::Output;
    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression) -> Self::Output;
    fn visit_function_expression(&mut self, expr: &FunctionExpression) -> Self::Output;
    fn visit_class_expression(&mut self, expr: &ClassExpression) -> Self::Output;
    fn visit_yield_expression(&mut self, expr: &YieldExpression) -> Self::Output;
    fn visit_await_expression(&mut self, expr: &AwaitExpression) -> Self::Output;
    fn visit_super(&mut self, super_expr: &Super) -> Self::Output;
    fn visit_meta_property(&mut self, prop: &MetaProperty) -> Self::Output;
    fn visit_spread_element(&mut self, elem: &SpreadElement) -> Self::Output;
    fn visit_rest_element(&mut self, elem: &RestElement) -> Self::Output;
    fn visit_template_literal(&mut self, lit: &TemplateLiteral) -> Self::Output;
    fn visit_tagged_template_expression(&mut self, expr: &TaggedTemplateExpression)
        -> Self::Output;
    fn visit_import_declaration(&mut self, decl: &ImportDeclaration) -> Self::Output;
    fn visit_export_declaration(&mut self, decl: &ExportDeclaration) -> Self::Output;
    fn visit_labeled_statement(&mut self, stmt: &LabeledStatement) -> Self::Output;
    fn visit_with_statement(&mut self, stmt: &WithStatement) -> Self::Output;
    fn visit_debugger_statement(&mut self, stmt: &DebuggerStatement) -> Self::Output;
    fn visit_bigint(&mut self, bigint: &str) -> Self::Output;
    fn visit_regexp(&mut self, regexp: &RegExp) -> Self::Output;
}

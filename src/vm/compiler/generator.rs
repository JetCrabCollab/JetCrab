use crate::ast::node::Node;
use crate::vm::compiler::expressions::{
    ArithmeticCore, ArithmeticGenerator, AssignmentCore, AssignmentGenerator, ComparisonCore,
    ComparisonGenerator, LogicalCore, LogicalGenerator, UnaryCore, UnaryGenerator,
};
use crate::vm::compiler::literals::{
    ArrayCore, ArrayGenerator, FunctionLiteralCore, ObjectCore, ObjectGenerator,
};
use crate::vm::compiler::scope::{ConstantCore, ConstantManager, ScopeCore, ScopeManager};
use crate::vm::compiler::statements::{
    ClassCore, ClassGenerator, ControlFlowCore, ControlFlowGenerator, FunctionCore,
    FunctionGenerator, VariableCore, VariableGenerator,
};
use crate::vm::instructions::Instruction;
use crate::vm::types::{CodeAddress, ConstantIndex, LocalIndex};
use std::collections::HashMap;

pub struct BytecodeGenerator {
    constants: Vec<String>,
    constant_map: HashMap<String, ConstantIndex>,
    instructions: Vec<Instruction>,
    local_vars: HashMap<String, LocalIndex>,
    next_local: usize,
    loop_labels: Vec<CodeAddress>,
    statement_labels: HashMap<String, CodeAddress>,
    current_labels: Vec<String>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            constant_map: HashMap::new(),
            instructions: Vec::new(),
            local_vars: HashMap::new(),
            next_local: 0,
            loop_labels: Vec::new(),
            statement_labels: HashMap::new(),
            current_labels: Vec::new(),
        }
    }
}

impl Default for BytecodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl BytecodeGenerator {
    pub fn generate(&mut self, ast: &Node) -> Vec<Instruction> {
        self.visit_node(ast);
        self.instructions.clone()
    }

    pub fn get_constants(&self) -> &Vec<String> {
        <Self as ConstantManager>::get_constants(self)
    }

    fn visit_node(&mut self, node: &Node) {
        match node {
            Node::Program(program) => {
                for stmt in &program.body {
                    self.visit_node(stmt);
                }
            }
            Node::VariableDeclaration(_decl) => {
                <Self as VariableGenerator>::generate_variable_declaration(self, node);
            }
            Node::FunctionDeclaration(_decl) => {
                <Self as FunctionGenerator>::generate_function_declaration(self, node);
            }
            Node::ClassDeclaration(_decl) => {
                <Self as ClassGenerator>::generate_class_declaration(self, node);
            }
            Node::ImportDeclaration(_stmt) => {
                <Self as ControlFlowGenerator>::generate_import_declaration(self, node);
            }
            Node::ExportDeclaration(_stmt) => {
                <Self as ControlFlowGenerator>::generate_export_declaration(self, node);
            }
            Node::ClassExpression(_expr) => {
                <Self as ClassGenerator>::generate_class_expression(self, node);
            }
            Node::YieldExpression(expr) => {
                if let Some(arg) = &expr.argument {
                    self.visit_node(arg);
                }
                self.instructions.push(Instruction::Yield);
            }
            Node::AwaitExpression(expr) => {
                self.visit_node(&expr.argument);
                self.instructions.push(Instruction::Await);
            }
            Node::SwitchStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_switch_statement(self, node);
            }
            Node::TryStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_try_statement(self, node);
            }
            Node::CatchClause(_clause) => {
                <Self as ControlFlowGenerator>::generate_catch_clause(self, node);
            }
            Node::ThrowStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_throw_statement(self, node);
            }
            Node::ReturnStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_return_statement(self, node);
            }
            Node::BreakStatement(_) => {
                <Self as ControlFlowGenerator>::generate_break_statement(self, node);
            }
            Node::ContinueStatement(_) => {
                <Self as ControlFlowGenerator>::generate_continue_statement(self, node);
            }
            Node::LabeledStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_labeled_statement(self, node);
            }
            Node::WithStatement(stmt) => {
                self.visit_node(&stmt.object);
                self.visit_node(&stmt.body);
            }
            Node::DebuggerStatement(_) => {}
            Node::TemplateLiteral(lit) => {
                for expr in &lit.expressions {
                    self.visit_node(expr);
                }
            }
            Node::TaggedTemplateExpression(expr) => {
                self.visit_node(&expr.tag);
                self.visit_node(&expr.quasi);
            }
            Node::Super(_) => {
                self.instructions
                    .push(Instruction::LoadLocal(LocalIndex::new(0)));
            }
            Node::MetaProperty(prop) => {
                self.visit_node(&prop.meta);
                self.visit_node(&prop.property);

                self.instructions.push(Instruction::LoadThisFunction);
            }
            Node::SpreadElement(elem) => {
                self.visit_node(&elem.argument);
                self.instructions.push(Instruction::Spread);
            }
            Node::RegExp(re) => {
                let constant_id = <Self as ConstantManager>::add_constant(self, re.pattern.clone());
                self.instructions.push(Instruction::PushConst(constant_id));
            }
            Node::BigInt(val) => {
                let constant_id = <Self as ConstantManager>::add_constant(self, val.clone());
                self.instructions.push(Instruction::PushBigInt(constant_id));
            }
            Node::BinaryExpression(expr) => {
                // Use ComparisonGenerator for comparison operators
                match expr.operator.as_str() {
                    "==" | "!=" | "===" | "!==" | "<" | ">" | "<=" | ">=" => {
                        <Self as ComparisonGenerator>::generate_comparison_expression(self, node);
                    }
                    // Use ArithmeticGenerator for arithmetic operators
                    "+" | "-" | "*" | "/" | "%" | "**" => {
                        <Self as ArithmeticGenerator>::generate_binary_expression(self, node);
                    }
                    // Use LogicalGenerator for logical operators
                    "&&" | "||" | "??" => {
                        <Self as LogicalGenerator>::generate_logical_expression(self, node);
                    }
                    // Default to arithmetic for unknown operators
                    _ => {
                        <Self as ArithmeticGenerator>::generate_binary_expression(self, node);
                    }
                }
            }
            Node::UnaryExpression(_expr) => {
                <Self as UnaryGenerator>::generate_unary_expression(self, node);
            }
            Node::CallExpression(_expr) => {
                <Self as AssignmentGenerator>::generate_call_expression(self, node);
            }
            Node::NewExpression(_expr) => {
                <Self as AssignmentGenerator>::generate_new_expression(self, node);
            }
            Node::MemberExpression(_expr) => {
                <Self as AssignmentGenerator>::generate_member_expression(self, node);
            }
            Node::AssignmentExpression(_expr) => {
                <Self as AssignmentGenerator>::generate_assignment_expression(self, node);
            }
            Node::ConditionalExpression(_expr) => {
                <Self as AssignmentGenerator>::generate_conditional_expression(self, node);
            }
            Node::LogicalExpression(_expr) => {
                <Self as LogicalGenerator>::generate_logical_expression(self, node);
            }
            Node::UpdateExpression(_expr) => {
                <Self as UnaryGenerator>::generate_update_expression(self, node);
            }
            Node::ArrowFunctionExpression(expr) => {
                for param in &expr.params {
                    self.visit_node(param);
                }
                self.visit_node(&expr.body);
            }
            Node::FunctionExpression(_expr) => {
                <Self as FunctionGenerator>::generate_function_expression(self, node);
            }
            Node::BlockStatement(stmt) => {
                for node in &stmt.body {
                    self.visit_node(node);
                }
            }
            Node::IfStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_if_statement(self, node);
            }
            Node::ForStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_for_statement(self, node);
            }
            Node::ForInStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_for_in_statement(self, node);
            }
            Node::ForOfStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_for_of_statement(self, node);
            }
            Node::WhileStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_while_statement(self, node);
            }
            Node::DoWhileStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_do_while_statement(self, node);
            }
            Node::ExpressionStatement(stmt) => {
                self.visit_node(&stmt.expression);
            }
            Node::ArrayLiteral(_lit) => {
                <Self as ArrayGenerator>::generate_array_literal(self, node);
            }
            Node::ObjectLiteral(_lit) => {
                <Self as ObjectGenerator>::generate_object_literal(self, node);
            }
            Node::Property(prop) => {
                self.visit_node(&prop.key);
                self.visit_node(&prop.value);
            }
            Node::RestElement(elem) => {
                self.visit_node(&elem.argument);
            }
            Node::Identifier(name) => {
                if let Some(&local_idx) = <Self as ScopeManager>::get_local(self, name) {
                    self.instructions.push(Instruction::LoadLocal(local_idx));
                } else {
                    let constant_id = <Self as ConstantManager>::add_constant(self, name.clone());
                    self.instructions.push(Instruction::PushConst(constant_id));
                }
            }
            Node::Number(n) => {
                let constant_id = <Self as ConstantManager>::add_constant(self, n.to_string());
                self.instructions.push(Instruction::PushConst(constant_id));
            }
            Node::String(s) => {
                // Store string literals with quotes to distinguish from numbers
                let quoted_string = format!("\"{}\"", s);
                let constant_id = <Self as ConstantManager>::add_constant(self, quoted_string);
                self.instructions.push(Instruction::PushConst(constant_id));
            }
            Node::Boolean(b) => {
                if *b {
                    self.instructions.push(Instruction::PushTrue);
                } else {
                    self.instructions.push(Instruction::PushFalse);
                }
            }
            Node::Null => {
                self.instructions.push(Instruction::PushNull);
            }
            Node::Undefined => {
                self.instructions.push(Instruction::PushUndefined);
            }
            Node::This => {
                self.instructions.push(Instruction::LoadThis);
            }
        }
    }
}

impl ConstantCore for BytecodeGenerator {
    fn constants(&self) -> &Vec<String> {
        &self.constants
    }

    fn constant_map(&self) -> &HashMap<String, ConstantIndex> {
        &self.constant_map
    }

    fn constants_mut(&mut self) -> &mut Vec<String> {
        &mut self.constants
    }

    fn constant_map_mut(&mut self) -> &mut HashMap<String, ConstantIndex> {
        &mut self.constant_map
    }
}

impl ScopeCore for BytecodeGenerator {
    fn local_vars(&self) -> &HashMap<String, LocalIndex> {
        &self.local_vars
    }

    fn local_vars_mut(&mut self) -> &mut HashMap<String, LocalIndex> {
        &mut self.local_vars
    }

    fn next_local(&self) -> usize {
        self.next_local
    }

    fn set_next_local(&mut self, next: usize) {
        self.next_local = next;
    }
}

impl VariableCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

impl ClassCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

impl ControlFlowCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }

    fn push_loop_label(&mut self, break_address: CodeAddress) {
        self.loop_labels.push(break_address);
    }

    fn pop_loop_label(&mut self) {
        self.loop_labels.pop();
    }

    fn get_current_break_address(&self) -> Option<CodeAddress> {
        self.loop_labels.last().copied()
    }
}

impl crate::vm::compiler::statements::control_flow::LabelManager for BytecodeGenerator {
    fn add_label(&mut self, name: String, address: CodeAddress) {
        self.statement_labels.insert(name, address);
    }

    fn get_label_address(&self, name: &str) -> Option<CodeAddress> {
        self.statement_labels.get(name).copied()
    }

    fn get_label_start_address(&self, label_name: &str) -> Option<CodeAddress> {
        self.statement_labels
            .get(&format!("{}_start", label_name))
            .copied()
    }

    fn get_label_end_address(&self, label_name: &str) -> Option<CodeAddress> {
        self.statement_labels
            .get(&format!("{}_end", label_name))
            .copied()
    }

    fn push_current_label(&mut self, name: String) {
        self.current_labels.push(name);
    }

    fn pop_current_label(&mut self) {
        self.current_labels.pop();
    }

    fn get_current_labels(&self) -> &[String] {
        &self.current_labels
    }
}

impl ArithmeticCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

impl ComparisonCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

impl LogicalCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

impl UnaryCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

impl AssignmentCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

impl ObjectCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

impl ArrayCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

impl FunctionLiteralCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

impl FunctionCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        BytecodeGenerator::visit_node(self, node)
    }
}

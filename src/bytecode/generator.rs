use crate::ast::node::Node;
use crate::bytecode::expressions::{
    ArithmeticCore, ArithmeticGenerator, AssignmentCore, AssignmentGenerator, ComparisonCore,
    LogicalCore, LogicalGenerator, UnaryCore, UnaryGenerator,
};
use crate::bytecode::literals::{
    ArrayCore, ArrayGenerator, FunctionLiteralCore, ObjectCore, ObjectGenerator,
};
use crate::bytecode::scope::{ConstantCore, ConstantManager, ScopeCore, ScopeManager};
use crate::bytecode::statements::{
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
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            constant_map: HashMap::new(),
            instructions: Vec::new(),
            local_vars: HashMap::new(),
            next_local: 0,
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
            Node::ImportDeclaration(_) | Node::ExportDeclaration(_) => {}
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
            Node::SwitchStatement(stmt) => {
                self.visit_node(&stmt.discriminant);
                for case in &stmt.cases {
                    if let Some(test) = &case.test {
                        self.visit_node(test);
                    }
                    for cons in &case.consequent {
                        self.visit_node(cons);
                    }
                }
            }
            Node::TryStatement(stmt) => {
                self.visit_node(&stmt.block);
                if let Some(handler) = &stmt.handler {
                    self.visit_node(handler);
                }
                if let Some(finalizer) = &stmt.finalizer {
                    self.visit_node(finalizer);
                }
                self.instructions
                    .push(Instruction::Try(CodeAddress::new(0), CodeAddress::new(0)));
            }
            Node::CatchClause(clause) => {
                self.visit_node(&clause.param);
                self.visit_node(&clause.body);
                self.instructions.push(Instruction::Catch);
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
            Node::LabeledStatement(stmt) => {
                self.visit_node(&stmt.label);
                self.visit_node(&stmt.body);
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
            Node::BinaryExpression(_expr) => {
                <Self as ArithmeticGenerator>::generate_binary_expression(self, node);
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
            Node::FunctionExpression(expr) => {
                if let Some(id) = &expr.id {
                    self.visit_node(id);
                }
                for param in &expr.params {
                    self.visit_node(param);
                }
                self.visit_node(&expr.body);
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
                let constant_id = <Self as ConstantManager>::add_constant(self, s.clone());
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
        self.visit_node(node)
    }
}

impl FunctionCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl ClassCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl ControlFlowCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl ArithmeticCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl ComparisonCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl LogicalCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl UnaryCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl AssignmentCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl ObjectCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl ArrayCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl FunctionLiteralCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

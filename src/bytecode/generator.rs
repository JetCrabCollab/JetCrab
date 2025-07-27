use crate::ast::node::Node;
use crate::vm::instructions::Instruction;
use crate::vm::types::{ArraySize, CodeAddress, ConstantIndex, FunctionIndex, LocalIndex};
use std::collections::HashMap;

pub struct BytecodeGenerator {
    constants: Vec<String>,
    constant_map: HashMap<String, ConstantIndex>,
    instructions: Vec<Instruction>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            constant_map: HashMap::new(),
            instructions: Vec::new(),
        }
    }

    pub fn generate(&mut self, ast: &Node) -> Vec<Instruction> {
        self.visit_node(ast);
        self.instructions.clone()
    }

    pub fn get_constants(&self) -> &Vec<String> {
        &self.constants
    }

    fn visit_node(&mut self, node: &Node) {
        match node {
            Node::Program(program) => {
                for stmt in &program.body {
                    self.visit_node(stmt);
                }
            }
            Node::VariableDeclaration(decl) => {
                for var in &decl.declarations {
                    self.visit_node(&var.id);
                    if let Some(init) = &var.init {
                        self.visit_node(init);
                        self.instructions
                            .push(Instruction::StoreLocal(LocalIndex::new(0)));
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
            Node::ClassDeclaration(decl) => {
                if let Some(id) = &decl.id {
                    self.visit_node(id);
                }
                if let Some(super_class) = &decl.super_class {
                    self.visit_node(super_class);
                }
                self.visit_node(&decl.body);
                self.instructions.push(Instruction::NewClass);
            }
            Node::ImportDeclaration(_) | Node::ExportDeclaration(_) => {}
            Node::ClassExpression(expr) => {
                if let Some(id) = &expr.id {
                    self.visit_node(id);
                }
                if let Some(super_class) = &expr.super_class {
                    self.visit_node(super_class);
                }
                self.visit_node(&expr.body);
                self.instructions.push(Instruction::NewClass);
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
            Node::ThrowStatement(stmt) => {
                self.visit_node(&stmt.argument);
                self.instructions.push(Instruction::Throw);
            }
            Node::ReturnStatement(stmt) => {
                if let Some(arg) = &stmt.argument {
                    self.visit_node(arg);
                }
                self.instructions.push(Instruction::Return);
            }
            Node::BreakStatement(_) => {
                self.instructions
                    .push(Instruction::Jump(CodeAddress::new(0)));
            }
            Node::ContinueStatement(_) => {
                self.instructions
                    .push(Instruction::Jump(CodeAddress::new(0)));
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
                let constant_id = self.add_constant(re.pattern.clone());
                self.instructions.push(Instruction::PushConst(constant_id));
            }
            Node::BigInt(val) => {
                let constant_id = self.add_constant(val.clone());
                self.instructions.push(Instruction::PushBigInt(constant_id));
            }
            Node::BinaryExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
                match expr.operator.as_str() {
                    "+" => self.instructions.push(Instruction::Add),
                    "-" => self.instructions.push(Instruction::Sub),
                    "*" => self.instructions.push(Instruction::Mul),
                    "/" => self.instructions.push(Instruction::Div),
                    _ => {
                        self.instructions.push(Instruction::Add);
                    }
                }
            }
            Node::UnaryExpression(expr) => {
                self.visit_node(&expr.argument);
                match expr.operator.as_str() {
                    "!" => self.instructions.push(Instruction::Not),
                    "-" => {
                        self.instructions
                            .push(Instruction::PushConst(ConstantIndex::new(0)));
                        self.instructions.push(Instruction::Sub);
                    }
                    "+" => {}
                    "~" => {
                        self.instructions
                            .push(Instruction::PushConst(ConstantIndex::new(0)));
                        self.instructions.push(Instruction::Sub);
                        self.instructions.push(Instruction::Inc);
                    }
                    "typeof" => self.instructions.push(Instruction::TypeOf),
                    "void" => {
                        self.instructions.push(Instruction::Pop);
                        self.instructions.push(Instruction::PushUndefined);
                    }
                    "delete" => self.instructions.push(Instruction::Delete),
                    _ => {}
                }
            }
            Node::CallExpression(expr) => {
                for arg in &expr.arguments {
                    self.visit_node(arg);
                }
                self.visit_node(&expr.callee);
                self.instructions
                    .push(Instruction::Call(FunctionIndex::new(expr.arguments.len())));
            }
            Node::NewExpression(expr) => {
                for arg in &expr.arguments {
                    self.visit_node(arg);
                }
                self.visit_node(&expr.callee);
                self.instructions.push(Instruction::New);
            }
            Node::MemberExpression(expr) => {
                self.visit_node(&expr.object);
                self.visit_node(&expr.property);
                self.instructions.push(Instruction::GetProperty);
            }
            Node::AssignmentExpression(expr) => {
                self.visit_node(&expr.right);
                self.visit_node(&expr.left);
                self.instructions
                    .push(Instruction::StoreLocal(LocalIndex::new(0)));
            }
            Node::ConditionalExpression(expr) => {
                self.visit_node(&expr.test);

                let jump_to_alternate = self.instructions.len();
                self.instructions
                    .push(Instruction::JumpIfFalse(CodeAddress::new(0)));

                self.visit_node(&expr.consequent);

                let jump_to_end = self.instructions.len();
                self.instructions
                    .push(Instruction::Jump(CodeAddress::new(0)));

                let alternate_start = self.instructions.len();
                self.instructions[jump_to_alternate] =
                    Instruction::JumpIfFalse(CodeAddress::new(alternate_start));

                self.visit_node(&expr.alternate);

                let end_pos = self.instructions.len();
                self.instructions[jump_to_end] = Instruction::Jump(CodeAddress::new(end_pos));
            }
            Node::LogicalExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
                match expr.operator.as_str() {
                    "&&" => self.instructions.push(Instruction::And),
                    "||" => self.instructions.push(Instruction::Or),
                    "??" => self.instructions.push(Instruction::NullishCoalesce),
                    _ => {
                        self.instructions.push(Instruction::And);
                    }
                }
            }
            Node::UpdateExpression(expr) => {
                self.visit_node(&expr.argument);
                match expr.operator.as_str() {
                    "++" => {
                        if expr.prefix {
                            self.instructions.push(Instruction::Inc);
                        } else {
                            self.instructions.push(Instruction::Dup);
                            self.instructions.push(Instruction::Inc);
                        }
                    }
                    "--" => {
                        if expr.prefix {
                            self.instructions.push(Instruction::Dec);
                        } else {
                            self.instructions.push(Instruction::Dup);
                            self.instructions.push(Instruction::Dec);
                        }
                    }
                    _ => {
                        self.instructions.push(Instruction::Inc);
                    }
                }
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
            Node::IfStatement(stmt) => {
                self.visit_node(&stmt.test);
                self.visit_node(&stmt.consequent);
                if let Some(alt) = &stmt.alternate {
                    self.visit_node(alt);
                }
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
            Node::WhileStatement(stmt) => {
                self.visit_node(&stmt.test);
                self.visit_node(&stmt.body);
            }
            Node::DoWhileStatement(stmt) => {
                self.visit_node(&stmt.body);
                self.visit_node(&stmt.test);
            }
            Node::ExpressionStatement(stmt) => {
                self.visit_node(&stmt.expression);
            }
            Node::ArrayLiteral(lit) => {
                for elem in &lit.elements {
                    if let Some(e) = elem {
                        self.visit_node(e);
                    }
                }
                self.instructions
                    .push(Instruction::NewArray(ArraySize::new(lit.elements.len())));
            }
            Node::ObjectLiteral(lit) => {
                for prop in &lit.properties {
                    if let Node::Property(property) = prop {
                        self.visit_node(&property.key);
                        self.visit_node(&property.value);
                        self.instructions.push(Instruction::SetProperty);
                    }
                }
                self.instructions.push(Instruction::NewObject);
            }
            Node::Property(prop) => {
                self.visit_node(&prop.key);
                self.visit_node(&prop.value);
            }
            Node::RestElement(elem) => {
                self.visit_node(&elem.argument);
            }
            Node::Identifier(name) => {
                let constant_id = self.add_constant(name.clone());
                self.instructions.push(Instruction::PushConst(constant_id));
            }
            Node::Number(n) => {
                let constant_id = self.add_constant(n.to_string());
                self.instructions.push(Instruction::PushConst(constant_id));
            }
            Node::String(s) => {
                let constant_id = self.add_constant(s.clone());
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

    fn add_constant(&mut self, value: String) -> ConstantIndex {
        if let Some(&id) = self.constant_map.get(&value) {
            id
        } else {
            let id = ConstantIndex::new(self.constants.len());
            self.constants.push(value.clone());
            self.constant_map.insert(value, id);
            id
        }
    }
}

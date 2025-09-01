use crate::ast::*;

pub trait DeclarationVisitor {
    type Output;

    fn visit_program(&mut self, program: &Program) -> Self::Output;
    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) -> Self::Output;
    fn visit_function_declaration(&mut self, decl: &FunctionDeclaration) -> Self::Output;
    fn visit_class_declaration(&mut self, decl: &ClassDeclaration) -> Self::Output;
}

impl<T> DeclarationVisitor for T
where
    T: Visitor,
{
    type Output = T::Output;

    fn visit_program(&mut self, program: &Program) -> Self::Output {
        for statement in &program.body {
            self.visit_node(statement);
        }
        self.default_output()
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) -> Self::Output {
        for declarator in &decl.declarations {
            self.visit_node(&declarator.id);
            if let Some(init) = &declarator.init {
                self.visit_node(init);
            }
        }
        self.default_output()
    }

    fn visit_function_declaration(&mut self, decl: &FunctionDeclaration) -> Self::Output {
        if let Some(id) = &decl.id {
            self.visit_node(id);
        }
        for param in &decl.params {
            self.visit_node(param);
        }
        self.visit_node(&decl.body);
        self.default_output()
    }

    fn visit_class_declaration(&mut self, decl: &ClassDeclaration) -> Self::Output {
        if let Some(id) = &decl.id {
            self.visit_node(id);
        }
        if let Some(super_class) = &decl.super_class {
            self.visit_node(super_class);
        }
        self.visit_node(&decl.body);
        self.default_output()
    }
}

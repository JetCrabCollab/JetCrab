use super::ControlFlowCore;
use crate::ast::Node;

pub fn generate_import_declaration<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::ImportDeclaration(decl) = node {
        for specifier in &decl.specifiers {
            this.visit_node(specifier);
        }
        this.visit_node(&decl.source);
    }
}

pub fn generate_export_declaration<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::ExportDeclaration(decl) = node {
        if let Some(declaration) = &decl.declaration {
            this.visit_node(declaration);
        }
        for specifier in &decl.specifiers {
            this.visit_node(specifier);
        }
        if let Some(source) = &decl.source {
            this.visit_node(source);
        }
    }
}

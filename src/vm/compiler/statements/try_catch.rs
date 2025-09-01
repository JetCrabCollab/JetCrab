use crate::ast::Node;
use crate::vm::instructions::Instruction;

use super::ControlFlowCore;

pub fn generate_try_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::TryStatement(stmt) = node {
        this.visit_node(&stmt.block);

        if let Some(handler) = &stmt.handler {
            generate_catch_clause(this, handler);
        }

        if let Some(finalizer) = &stmt.finalizer {
            this.visit_node(finalizer);
        }
    }
}

pub fn generate_catch_clause<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::CatchClause(clause) = node {
        this.visit_node(&clause.param);
        this.visit_node(&clause.body);
    }
}

pub fn generate_throw_statement<T>(this: &mut T, node: &Node)
where
    T: ControlFlowCore,
{
    if let Node::ThrowStatement(stmt) = node {
        this.visit_node(&stmt.argument);
        this.instructions().push(Instruction::Throw);
    }
}

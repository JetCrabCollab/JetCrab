use crate::ast::*;

pub trait StatementVisitor {
    type Output;

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
    fn visit_labeled_statement(&mut self, stmt: &LabeledStatement) -> Self::Output;
    fn visit_with_statement(&mut self, stmt: &WithStatement) -> Self::Output;
    fn visit_debugger_statement(&mut self, stmt: &DebuggerStatement) -> Self::Output;
}

impl<T> StatementVisitor for T
where
    T: Visitor,
{
    type Output = T::Output;

    fn visit_block_statement(&mut self, stmt: &BlockStatement) -> Self::Output {
        for statement in &stmt.body {
            self.visit_node(statement);
        }
        self.default_output()
    }

    fn visit_if_statement(&mut self, stmt: &IfStatement) -> Self::Output {
        self.visit_node(&stmt.test);
        self.visit_node(&stmt.consequent);
        if let Some(alternate) = &stmt.alternate {
            self.visit_node(alternate);
        }
        self.default_output()
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
        self.default_output()
    }

    fn visit_for_in_statement(&mut self, stmt: &ForInStatement) -> Self::Output {
        self.visit_node(&stmt.left);
        self.visit_node(&stmt.right);
        self.visit_node(&stmt.body);
        self.default_output()
    }

    fn visit_for_of_statement(&mut self, stmt: &ForOfStatement) -> Self::Output {
        self.visit_node(&stmt.left);
        self.visit_node(&stmt.right);
        self.visit_node(&stmt.body);
        self.default_output()
    }

    fn visit_while_statement(&mut self, stmt: &WhileStatement) -> Self::Output {
        self.visit_node(&stmt.test);
        self.visit_node(&stmt.body);
        self.default_output()
    }

    fn visit_do_while_statement(&mut self, stmt: &DoWhileStatement) -> Self::Output {
        self.visit_node(&stmt.body);
        self.visit_node(&stmt.test);
        self.default_output()
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
        self.default_output()
    }

    fn visit_try_statement(&mut self, stmt: &TryStatement) -> Self::Output {
        self.visit_node(&stmt.block);
        if let Some(handler) = &stmt.handler {
            self.visit_node(handler);
        }
        if let Some(finalizer) = &stmt.finalizer {
            self.visit_node(finalizer);
        }
        self.default_output()
    }

    fn visit_catch_clause(&mut self, clause: &CatchClause) -> Self::Output {
        self.visit_node(&clause.param);
        self.visit_node(&clause.body);
        self.default_output()
    }

    fn visit_throw_statement(&mut self, stmt: &ThrowStatement) -> Self::Output {
        self.visit_node(&stmt.argument);
        self.default_output()
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStatement) -> Self::Output {
        if let Some(argument) = &stmt.argument {
            self.visit_node(argument);
        }
        self.default_output()
    }

    fn visit_break_statement(&mut self, _stmt: &BreakStatement) -> Self::Output {
        self.default_output()
    }

    fn visit_continue_statement(&mut self, _stmt: &ContinueStatement) -> Self::Output {
        self.default_output()
    }

    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement) -> Self::Output {
        self.visit_node(&stmt.expression);
        self.default_output()
    }

    fn visit_labeled_statement(&mut self, stmt: &LabeledStatement) -> Self::Output {
        self.visit_node(&stmt.label);
        self.visit_node(&stmt.body);
        self.default_output()
    }

    fn visit_with_statement(&mut self, stmt: &WithStatement) -> Self::Output {
        self.visit_node(&stmt.object);
        self.visit_node(&stmt.body);
        self.default_output()
    }

    fn visit_debugger_statement(&mut self, _stmt: &DebuggerStatement) -> Self::Output {
        self.default_output()
    }
}

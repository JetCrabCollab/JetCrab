use crate::ast::*;

pub trait ExpressionVisitor {
    type Output;

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) -> Self::Output;
    fn visit_unary_expression(&mut self, expr: &UnaryExpression) -> Self::Output;
    fn visit_call_expression(&mut self, expr: &CallExpression) -> Self::Output;
    fn visit_new_expression(&mut self, expr: &NewExpression) -> Self::Output;
    fn visit_member_expression(&mut self, expr: &MemberExpression) -> Self::Output;
    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression) -> Self::Output;
    fn visit_conditional_expression(&mut self, expr: &ConditionalExpression) -> Self::Output;
    fn visit_logical_expression(&mut self, expr: &LogicalExpression) -> Self::Output;
    fn visit_update_expression(&mut self, expr: &UpdateExpression) -> Self::Output;
    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression) -> Self::Output;
    fn visit_function_expression(&mut self, expr: &FunctionExpression) -> Self::Output;
    fn visit_class_expression(&mut self, expr: &ClassExpression) -> Self::Output;
    fn visit_yield_expression(&mut self, expr: &YieldExpression) -> Self::Output;
    fn visit_await_expression(&mut self, expr: &AwaitExpression) -> Self::Output;
    fn visit_super(&mut self, super_expr: &Super) -> Self::Output;
    fn visit_meta_property(&mut self, prop: &MetaProperty) -> Self::Output;
    fn visit_spread_element(&mut self, elem: &SpreadElement) -> Self::Output;
    fn visit_rest_element(&mut self, elem: &RestElement) -> Self::Output;
    fn visit_tagged_template_expression(&mut self, expr: &TaggedTemplateExpression)
        -> Self::Output;
}

impl<T> ExpressionVisitor for T
where
    T: Visitor,
{
    type Output = T::Output;

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) -> Self::Output {
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
        self.default_output()
    }

    fn visit_unary_expression(&mut self, expr: &UnaryExpression) -> Self::Output {
        self.visit_node(&expr.argument);
        self.default_output()
    }

    fn visit_call_expression(&mut self, expr: &CallExpression) -> Self::Output {
        self.visit_node(&expr.callee);
        for arg in &expr.arguments {
            self.visit_node(arg);
        }
        self.default_output()
    }

    fn visit_new_expression(&mut self, expr: &NewExpression) -> Self::Output {
        self.visit_node(&expr.callee);
        for arg in &expr.arguments {
            self.visit_node(arg);
        }
        self.default_output()
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression) -> Self::Output {
        self.visit_node(&expr.object);
        if expr.computed {
            self.visit_node(&expr.property);
        }
        self.default_output()
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression) -> Self::Output {
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
        self.default_output()
    }

    fn visit_conditional_expression(&mut self, expr: &ConditionalExpression) -> Self::Output {
        self.visit_node(&expr.test);
        self.visit_node(&expr.consequent);
        self.visit_node(&expr.alternate);
        self.default_output()
    }

    fn visit_logical_expression(&mut self, expr: &LogicalExpression) -> Self::Output {
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
        self.default_output()
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression) -> Self::Output {
        self.visit_node(&expr.argument);
        self.default_output()
    }

    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression) -> Self::Output {
        for param in &expr.params {
            self.visit_node(param);
        }
        self.visit_node(&expr.body);
        self.default_output()
    }

    fn visit_function_expression(&mut self, expr: &FunctionExpression) -> Self::Output {
        if let Some(id) = &expr.id {
            self.visit_node(id);
        }
        for param in &expr.params {
            self.visit_node(param);
        }
        self.visit_node(&expr.body);
        self.default_output()
    }

    fn visit_class_expression(&mut self, expr: &ClassExpression) -> Self::Output {
        if let Some(id) = &expr.id {
            self.visit_node(id);
        }
        if let Some(super_class) = &expr.super_class {
            self.visit_node(super_class);
        }
        self.visit_node(&expr.body);
        self.default_output()
    }

    fn visit_yield_expression(&mut self, expr: &YieldExpression) -> Self::Output {
        if let Some(argument) = &expr.argument {
            self.visit_node(argument);
        }
        self.default_output()
    }

    fn visit_await_expression(&mut self, expr: &AwaitExpression) -> Self::Output {
        self.visit_node(&expr.argument);
        self.default_output()
    }

    fn visit_super(&mut self, _super_expr: &Super) -> Self::Output {
        self.default_output()
    }

    fn visit_meta_property(&mut self, _prop: &MetaProperty) -> Self::Output {
        self.default_output()
    }

    fn visit_spread_element(&mut self, elem: &SpreadElement) -> Self::Output {
        self.visit_node(&elem.argument);
        self.default_output()
    }

    fn visit_rest_element(&mut self, elem: &RestElement) -> Self::Output {
        self.visit_node(&elem.argument);
        self.default_output()
    }

    fn visit_tagged_template_expression(
        &mut self,
        expr: &TaggedTemplateExpression,
    ) -> Self::Output {
        self.visit_node(&expr.tag);
        self.visit_node(&expr.quasi);
        self.default_output()
    }
}

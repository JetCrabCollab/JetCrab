use crate::ast::*;

pub trait LiteralVisitor {
    type Output;

    fn visit_array_literal(&mut self, lit: &ArrayLiteral) -> Self::Output;
    fn visit_object_literal(&mut self, lit: &ObjectLiteral) -> Self::Output;
    fn visit_template_literal(&mut self, lit: &TemplateLiteral) -> Self::Output;
    fn visit_property(&mut self, prop: &Property) -> Self::Output;
    fn visit_identifier(&mut self, id: &str) -> Self::Output;
    fn visit_number(&mut self, num: f64) -> Self::Output;
    fn visit_string(&mut self, s: &str) -> Self::Output;
    fn visit_boolean(&mut self, b: bool) -> Self::Output;
    fn visit_null(&mut self) -> Self::Output;
    fn visit_undefined(&mut self) -> Self::Output;
    fn visit_this(&mut self) -> Self::Output;
    fn visit_regexp(&mut self, regexp: &RegExp) -> Self::Output;
    fn visit_bigint(&mut self, bigint: &str) -> Self::Output;
}

impl<T> LiteralVisitor for T
where
    T: Visitor,
{
    type Output = T::Output;

    fn visit_array_literal(&mut self, lit: &ArrayLiteral) -> Self::Output {
        for elem in lit.elements.iter().flatten() {
            self.visit_node(elem);
        }
        self.default_output()
    }

    fn visit_object_literal(&mut self, lit: &ObjectLiteral) -> Self::Output {
        for property in &lit.properties {
            self.visit_node(property);
        }
        self.default_output()
    }

    fn visit_template_literal(&mut self, lit: &TemplateLiteral) -> Self::Output {
        for expr in &lit.expressions {
            self.visit_node(expr);
        }
        self.default_output()
    }

    fn visit_property(&mut self, prop: &Property) -> Self::Output {
        if prop.computed {
            self.visit_node(&prop.key);
        }
        self.visit_node(&prop.value);
        self.default_output()
    }

    fn visit_identifier(&mut self, _id: &str) -> Self::Output {
        self.default_output()
    }

    fn visit_number(&mut self, _num: f64) -> Self::Output {
        self.default_output()
    }

    fn visit_string(&mut self, _s: &str) -> Self::Output {
        self.default_output()
    }

    fn visit_boolean(&mut self, _b: bool) -> Self::Output {
        self.default_output()
    }

    fn visit_null(&mut self) -> Self::Output {
        self.default_output()
    }

    fn visit_undefined(&mut self) -> Self::Output {
        self.default_output()
    }

    fn visit_this(&mut self) -> Self::Output {
        self.default_output()
    }

    fn visit_regexp(&mut self, _regexp: &RegExp) -> Self::Output {
        self.default_output()
    }

    fn visit_bigint(&mut self, _bigint: &str) -> Self::Output {
        self.default_output()
    }
}

use crate::ast::Node;
use crate::vm::instructions::Instruction;

pub trait TemplateGenerator {
    fn generate_template_literal(&mut self, node: &Node);
}

pub trait TemplateCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
    fn add_constant(&mut self, value: String) -> crate::vm::types::ConstantIndex;
}

impl<T> TemplateGenerator for T
where
    T: TemplateCore,
{
    fn generate_template_literal(&mut self, node: &Node) {
        if let Node::TemplateLiteral(lit) = node {
            // Start with an empty string
            let empty_string_id = self.add_constant("".to_string());
            self.instructions()
                .push(Instruction::PushConst(empty_string_id));

            // Process each quasi and expression pair
            for (i, quasi) in lit.quasis.iter().enumerate() {
                // Add the text part
                if !quasi.value.is_empty() {
                    let text_id = self.add_constant(quasi.value.clone());
                    self.instructions().push(Instruction::PushConst(text_id));
                    self.instructions().push(Instruction::Add); // Concatenate strings
                }

                // Add the expression if it exists
                if i < lit.expressions.len() {
                    self.visit_node(&lit.expressions[i]);
                    // Convert expression to string and concatenate
                    self.instructions().push(Instruction::ToString);
                    self.instructions().push(Instruction::Add);
                }
            }
        }
    }
}

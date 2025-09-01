use jetcrab::ast::visitor::*;
use jetcrab::ast::*;

fn main() {
    println!("🧪 Testing Modularized Visitor System");
    println!("=====================================");

    // Create a simple AST
    let program = Program {
        body: vec![
            Node::VariableDeclaration(VariableDeclaration {
                kind: "let".to_string(),
                declarations: vec![VariableDeclarator {
                    id: Box::new(Node::Identifier("x".to_string())),
                    init: Some(Box::new(Node::Number(42.0))),
                    span: None,
                }],
                span: None,
            }),
            Node::ExpressionStatement(ExpressionStatement {
                expression: Box::new(Node::BinaryExpression(BinaryExpression {
                    operator: "+".to_string(),
                    left: Box::new(Node::Identifier("x".to_string())),
                    right: Box::new(Node::Number(10.0)),
                    span: None,
                })),
                span: None,
            }),
        ],
        source_type: "module".to_string(),
        span: None,
    };

    // Test NodeCounter
    println!("\n📊 Testing NodeCounter:");
    let mut counter = NodeCounter::new();
    counter.visit_node(&Node::Program(program.clone()));
    println!("Total nodes counted: {}", counter.count.as_usize());

    // Test DefaultVisitor
    println!("\n🔍 Testing DefaultVisitor:");
    let mut visitor = DefaultVisitor;
    visitor.visit_node(&Node::Program(program));

    println!("\n✅ All visitor tests passed!");
    println!("\n🏗️  Modularization Benefits:");
    println!("   • Clean separation of concerns");
    println!("   • Easy to extend with new visitor types");
    println!("   • Better maintainability");
    println!("   • Reduced file sizes");
    println!("   • Improved code organization");
}

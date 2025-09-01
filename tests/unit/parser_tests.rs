use jetcrab::parser::core::Parser;

#[test]
fn test_parser_new() {
    let _parser = Parser::new("42");
}

#[test]
fn test_parser_parse() {
    let mut parser = Parser::new("42");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_variable_declaration() {
    let mut parser = Parser::new("let x = 42;");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_function_declaration() {
    let mut parser = Parser::new("function test() {}");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_binary_expression() {
    let mut parser = Parser::new("1 + 2");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_unary_expression() {
    let mut parser = Parser::new("-42");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_call_expression() {
    let mut parser = Parser::new("func(42)");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_member_expression() {
    let mut parser = Parser::new("obj.prop");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_assignment_expression() {
    let mut parser = Parser::new("x = 42");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_logical_expression() {
    let mut parser = Parser::new("true && false");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_conditional_expression() {
    let mut parser = Parser::new("true ? 1 : 2");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_array_literal() {
    let mut parser = Parser::new("[42]");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_object_literal() {
    let mut parser = Parser::new("{key: 42}");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_arrow_function() {
    let mut parser = Parser::new("(x) => 42");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_class_declaration() {
    let mut parser = Parser::new("class Test {}");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_function_with_parameters() {
    let mut parser = Parser::new("function test(x, y) {}");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_destructuring() {
    let mut parser = Parser::new("let {x: y} = obj;");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

#[test]
fn test_parser_parse_spread_operator() {
    let mut parser = Parser::new("[...array]");
    let ast = parser.parse();
    assert!(ast.is_ok());
}

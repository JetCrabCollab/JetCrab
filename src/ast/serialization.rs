use crate::ast::node::Node;
use serde_json;

pub fn serialize_ast(node: &Node) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(node)
}

pub fn deserialize_ast(json: &str) -> Result<Node, serde_json::Error> {
    serde_json::from_str(json)
}

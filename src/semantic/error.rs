#[derive(Debug, Clone)]
pub struct SemanticError {
    pub message: String,
    pub position: Option<(u32, u32)>,
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Semantic error: {}", self.message)
    }
}

impl std::error::Error for SemanticError {}

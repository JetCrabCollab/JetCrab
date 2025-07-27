use crate::lexer::{Token, TokenKind};
use crate::parser::error::ParserError;
use crate::vm::types::ErrorCount;

#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    SkipUntil(Vec<String>),
    SkipUntilStatement,
    SkipUntilBlock,
    SkipUntilFunction,
    SkipUntilClass,
    SkipUntilModule,
    InsertToken(String),
    ReplaceToken(String),
    DeleteToken,
    NoRecovery,
}

#[derive(Debug, Clone)]
pub struct RecoveryContext {
    pub current_token: Option<Token>,
    pub previous_token: Option<Token>,
    pub recovery_tokens: Vec<String>,
    pub context: ParsingContext,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParsingContext {
    TopLevel,
    Statement,
    Block,
    Function,
    Class,
    Module,
    Expression,
    Declaration,
}

impl RecoveryContext {
    pub fn new(
        current_token: Option<Token>,
        previous_token: Option<Token>,
        context: ParsingContext,
    ) -> Self {
        Self {
            current_token,
            previous_token,
            recovery_tokens: Vec::new(),
            context,
        }
    }

    pub fn with_recovery_tokens(mut self, tokens: Vec<String>) -> Self {
        self.recovery_tokens = tokens;
        self
    }

    pub fn determine_strategy(&self) -> RecoveryStrategy {
        match &self.context {
            ParsingContext::TopLevel => {
                if let Some(token) = &self.current_token {
                    match token.kind {
                        TokenKind::Semicolon | TokenKind::RightBrace => {
                            RecoveryStrategy::SkipUntil(vec![";".to_string(), "}".to_string()])
                        }
                        _ => RecoveryStrategy::SkipUntilStatement,
                    }
                } else {
                    RecoveryStrategy::NoRecovery
                }
            }

            ParsingContext::Statement => {
                RecoveryStrategy::SkipUntil(vec![";".to_string(), "}".to_string(), ")".to_string()])
            }

            ParsingContext::Block => RecoveryStrategy::SkipUntil(vec!["}".to_string()]),

            ParsingContext::Function => {
                RecoveryStrategy::SkipUntil(vec!["}".to_string(), ";".to_string()])
            }

            ParsingContext::Class => RecoveryStrategy::SkipUntil(vec!["}".to_string()]),

            ParsingContext::Module => RecoveryStrategy::SkipUntil(vec![
                "}".to_string(),
                "import".to_string(),
                "export".to_string(),
            ]),

            ParsingContext::Expression => RecoveryStrategy::SkipUntil(vec![
                ";".to_string(),
                ",".to_string(),
                ")".to_string(),
                "]".to_string(),
                "}".to_string(),
            ]),

            ParsingContext::Declaration => {
                RecoveryStrategy::SkipUntil(vec![";".to_string(), "}".to_string()])
            }
        }
    }

    pub fn is_recovery_token(&self, token: &Token) -> bool {
        let token_str = format!("{:?}", token.kind);
        self.recovery_tokens.iter().any(|t| token_str.contains(t))
    }

    pub fn current_position(&self) -> Option<crate::ast::node::Position> {
        self.current_token
            .as_ref()
            .map(|t| crate::ast::node::Position {
                line: t.start().line,
                column: t.start().column,
            })
    }
}

#[derive(Debug)]
pub struct ErrorRecovery {
    max_errors: ErrorCount,
    error_count: ErrorCount,
    errors: Vec<ParserError>,
}

impl ErrorRecovery {
    pub fn new(max_errors: usize) -> Self {
        Self {
            max_errors: ErrorCount::new(max_errors),
            error_count: ErrorCount::new(0),
            errors: Vec::new(),
        }
    }

    pub fn can_recover(&self) -> bool {
        self.error_count.as_usize() < self.max_errors.as_usize()
    }

    pub fn add_error(&mut self, error: ParserError) {
        self.errors.push(error);
        self.error_count.increment();
    }

    pub fn errors(&self) -> &[ParserError] {
        &self.errors
    }

    pub fn clear_errors(&mut self) {
        self.errors.clear();
        self.error_count.reset();
    }

    pub fn error_count(&self) -> usize {
        self.error_count.as_usize()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Default for ErrorRecovery {
    fn default() -> Self {
        Self::new(100)
    }
}

use crate::lexer::TokenKind;
use crate::parser::Parser;

impl Parser {
    pub fn is_equality_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(
                &token.kind,
                TokenKind::Equal
                    | TokenKind::NotEqual
                    | TokenKind::StrictEqual
                    | TokenKind::StrictNotEqual
            )
        } else {
            false
        }
    }
}

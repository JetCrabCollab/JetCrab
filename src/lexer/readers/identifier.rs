use crate::lexer::{LexerError, TokenKind};

pub trait IdentifierReader {
    fn read_identifier_or_keyword(&mut self) -> Result<TokenKind, LexerError>;
}

impl<T> IdentifierReader for T
where
    T: LexerCore,
{
    fn read_identifier_or_keyword(&mut self) -> Result<TokenKind, LexerError> {
        let mut identifier = String::new();

        while self.pos() < self.source().len() {
            let c = self.source()[self.pos()];
            if c.is_alphanumeric() || c == '_' || c == '$' || c.is_alphabetic() || !c.is_ascii() {
                identifier.push(c);
                self.advance_pos();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "true" => Ok(TokenKind::Boolean(true)),
            "false" => Ok(TokenKind::Boolean(false)),
            "null" => Ok(TokenKind::Null),
            "undefined" => Ok(TokenKind::Undefined),
            "this" => Ok(TokenKind::Keyword("this".to_string())),
            "super" => Ok(TokenKind::Keyword("super".to_string())),
            "let" | "const" | "var" | "function" | "if" | "else" | "return" | "async" | "await"
            | "yield" | "import" | "export" | "new" | "class" | "extends" | "static" | "get"
            | "set" | "try" | "catch" | "finally" | "throw" | "break" | "continue" | "switch"
            | "case" | "default" | "for" | "while" | "do" | "in" | "of" | "with" | "delete"
            | "instanceof" | "typeof" | "void" | "debugger" | "enum" | "interface" | "package"
            | "private" | "protected" | "public" | "implements" | "abstract" | "boolean"
            | "byte" | "char" | "double" | "final" | "float" | "goto" | "int" | "long"
            | "native" | "short" | "synchronized" | "throws" | "transient" | "volatile" => {
                Ok(TokenKind::Keyword(identifier))
            }
            _ => Ok(TokenKind::Identifier(identifier)),
        }
    }
}

pub trait LexerCore {
    fn source(&self) -> &[char];
    fn pos(&self) -> usize;
    fn advance_pos(&mut self);
}

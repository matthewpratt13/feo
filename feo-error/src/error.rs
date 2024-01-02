use crate::lex_error::LexError;
use crate::parser_error::ParserError;
use crate::type_error::TypeError;

#[derive(Debug, Clone)]
pub enum CompilerError {
    Lex(LexError),
    Parser(ParserError),
    Type(TypeError),
}

impl CompilerError {
    pub fn line_col(&self) -> (usize, usize) {
        match self {
            Self::Lex(l) => l.position.line_col(),
            Self::Parser(p) => p.position.line_col(),
            Self::Type(t) => t.position.line_col(),
        }
    }
}

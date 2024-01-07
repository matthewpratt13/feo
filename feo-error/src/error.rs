use crate::lex_error::LexError;
use crate::parser_error::ParserError;
use crate::type_error::TypeError;

pub type FeoError = Box<dyn std::error::Error>;

#[derive(Debug, Clone)]
pub enum CompilerError {
    Lex(LexError),
    Parser(ParserError),
    Type(TypeError),
}

impl CompilerError {
    pub fn line_col(&self) -> (usize, usize) {
        match self {
            CompilerError::Lex(l) => l.position.line_col(),
            CompilerError::Parser(p) => p.position.line_col(),
            CompilerError::Type(t) => t.position.line_col(),
        }
    }

    pub fn error_kind(&self) -> FeoError {
        match self {
            CompilerError::Lex(l) => Box::new(l.error_kind),
            CompilerError::Parser(p) => Box::new(p.error_kind),
            CompilerError::Type(t) => Box::new(t.error_kind),
        }
    }
}

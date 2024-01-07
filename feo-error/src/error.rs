use std::error::Error;

use feo_types::span::Position;

use crate::lex_error::LexError;
use crate::parser_error::{ParserError, ParserErrorKind};
use crate::type_error::TypeError;

#[derive(Default, Debug, Clone)]
pub enum CompilerError {
    Lex(LexError),
    Parser(ParserError),
    Type(TypeError),

    #[default]
    UnexpectedError,
}

impl CompilerError {
    pub fn line_col(&self) -> (usize, usize) {
        match self {
            CompilerError::Lex(l) => l.position.line_col(),
            CompilerError::Parser(p) => p.position.line_col(),
            CompilerError::Type(t) => t.position.line_col(),
            CompilerError::UnexpectedError => Position::default().line_col(),
        }
    }

    pub fn error_kind(&self) -> Box<dyn Error> {
        match self {
            CompilerError::Lex(l) => Box::new(l.error_kind),
            CompilerError::Parser(p) => Box::new(p.error_kind),
            CompilerError::Type(t) => Box::new(t.error_kind),
            CompilerError::UnexpectedError => Box::new(ParserErrorKind::UnknownError),
        }
    }
}

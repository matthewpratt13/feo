use std::error::Error;
use std::sync::Arc;

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

#[derive(Default, Debug, Clone)]
pub struct Position {
    input: Arc<String>,
    pos: usize,
}

impl Position {
    pub fn new(input: &str, pos: usize) -> Position {
        input
            .get(pos..)
            .map(|_| Position {
                input: Arc::new(input.to_string()),
                pos,
            })
            .expect("Position out of bounds")
    }

    #[inline]
    pub fn line_col(&self) -> (usize, usize) {
        if self.pos > self.input.len() {
            panic!("Position out of bounds");
        }

        let slice = &self.input[..self.pos];
        let lines = slice.split('\n').collect::<Vec<_>>();
        let line_count = lines.len();
        let last_line_len = lines.last().unwrap_or(&"").chars().count() + 1;

        (line_count, last_line_len)
    }
}

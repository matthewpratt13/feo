use std::sync::Arc;

use thiserror::Error;

use crate::span::{Span, Spanned};

#[derive(Debug, Clone)]
pub enum DelimKind {
    Paren,
    Bracket,
    Brace,
}

impl TryFrom<char> for DelimKind {
    type Error = DelimiterError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' | ')' => Ok(DelimKind::Paren),
            '[' | ']' => Ok(DelimKind::Bracket),
            '{' | '}' => Ok(DelimKind::Brace),
            _ => Err(DelimiterError::UnrecognizedDelimiter),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DelimOrientation {
    Open,
    Close,
}

#[derive(Debug)]
pub struct Delimiter {
    pub delim_kind: DelimKind,
    pub delim_orientation: DelimOrientation,
    span: Span,
}

impl Delimiter {
    pub fn new(
        delim_kind: DelimKind,
        delim_orientation: DelimOrientation,
        input: &str,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            delim_kind,
            delim_orientation,
            span: Span::new(Arc::new(input.to_string()), start, end),
        }
    }
}

impl Spanned for Delimiter {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug, Error)]
pub enum DelimiterError {
    #[error("unrecognized delimiter")]
    UnrecognizedDelimiter,
}

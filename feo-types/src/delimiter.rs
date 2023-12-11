use thiserror::Error;

use crate::span::{Span, Spanned};

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

pub enum DelimOrientation {
    Open,
    Close,
}

pub struct Delimiter {
    delim_kind: DelimKind,
    delim_orientation: DelimOrientation,
    span: Span,
}

impl Delimiter {
    pub fn delim_kind(&self) -> &DelimKind {
        &self.delim_kind
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

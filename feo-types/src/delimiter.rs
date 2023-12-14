use crate::error::TypeError;
use crate::span::{Span, Spanned};

#[derive(Debug, Clone)]
pub enum DelimKind {
    Paren,
    Bracket,
    Brace,
}

impl TryFrom<char> for DelimKind {
    type Error = TypeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' | ')' => Ok(DelimKind::Paren),
            '[' | ']' => Ok(DelimKind::Bracket),
            '{' | '}' => Ok(DelimKind::Brace),
            _ => Err(TypeError::UnrecognizedDelimiter),
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
    pub delim: (DelimKind, DelimOrientation),
    span: Span,
}

impl Delimiter {
    pub fn new(delim_kind: DelimKind, delim_orientation: DelimOrientation, span: Span) -> Self {
        Self {
            delim: (delim_kind, delim_orientation),
            span,
        }
    }
}

impl Spanned for Delimiter {
    fn span(&self) -> &Span {
        &self.span
    }
}

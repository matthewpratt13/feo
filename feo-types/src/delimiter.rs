use core::str::FromStr;

use crate::error::TypeErrorKind;
use crate::span::{Span, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum DelimKind {
    Paren,
    Bracket,
    Brace,
}

impl FromStr for DelimKind {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" | ")" => Ok(DelimKind::Paren),
            "[" | "]" => Ok(DelimKind::Bracket),
            "{" | "}" => Ok(DelimKind::Brace),
            _ => Err(TypeErrorKind::UnrecognizedDelimiter),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DelimOrientation {
    Open,
    Close,
}

impl FromStr for DelimOrientation {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" | "[" | "{" => Ok(DelimOrientation::Open),
            ")" | "]" | "}" => Ok(DelimOrientation::Close),
            _ => Err(TypeErrorKind::UnrecognizedDelimiter),
        }
    }
}

#[derive(Debug, Clone)]
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

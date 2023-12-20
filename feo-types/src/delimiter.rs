use std::str::FromStr;

use crate::error::TypeError;
use crate::span::{Span, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum DelimKind {
    Paren,
    Bracket,
    Brace,
}

impl FromStr for DelimKind {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" | ")" => Ok(DelimKind::Paren),
            "[" | "]" => Ok(DelimKind::Bracket),
            "{" | "}" => Ok(DelimKind::Brace),
            _ => Err(TypeError::UnrecognizedDelimiter),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DelimOrientation {
    Open,
    Close,
}

impl FromStr for DelimOrientation {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" | "[" | "{" => Ok(DelimOrientation::Open),
            ")" | "]" | "}" => Ok(DelimOrientation::Close),
            _ => Err(TypeError::UnrecognizedDelimiter),
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

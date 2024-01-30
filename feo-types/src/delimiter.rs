use std::str::FromStr;

use crate::{
    span::{Span, Spanned},
    utils::TypeErrorKind,
};

#[derive(Debug, Clone, PartialEq)]
pub enum DelimKind {
    Parenthesis,
    Bracket,
    Brace,
}

impl FromStr for DelimKind {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" | ")" => Ok(DelimKind::Parenthesis),
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

    pub fn as_char(self) -> char {
        match (self.delim.0, self.delim.1) {
            (DelimKind::Parenthesis, DelimOrientation::Open) => '(',
            (DelimKind::Parenthesis, DelimOrientation::Close) => ')',
            (DelimKind::Bracket, DelimOrientation::Open) => '[',
            (DelimKind::Bracket, DelimOrientation::Close) => ']',
            (DelimKind::Brace, DelimOrientation::Open) => '{',
            (DelimKind::Brace, DelimOrientation::Close) => '}',
        }
    }
}

impl Spanned for Delimiter {
    fn span(&self) -> Span {
        self.clone().span
    }
}

pub fn is_delimiter(c: char) -> bool {
    ['(', ')', '[', ']', '{', '}'].contains(&c)
}

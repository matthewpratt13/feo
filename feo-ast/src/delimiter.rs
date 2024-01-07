use std::str::FromStr;

use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    type_error::{TypeError, TypeErrorKind},
};

use feo_types::span::{Position, Span, Spanned};

use crate::token::{Token, Tokenize};

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

    pub fn as_char(delim_kind: DelimKind, delim_orientation: DelimOrientation) -> char {
        match (delim_kind, delim_orientation) {
            (DelimKind::Parenthesis, DelimOrientation::Open) => '(',
            (DelimKind::Parenthesis, DelimOrientation::Close) => ')',
            (DelimKind::Bracket, DelimOrientation::Open) => '[',
            (DelimKind::Bracket, DelimOrientation::Close) => ']',
            (DelimKind::Brace, DelimOrientation::Open) => '{',
            (DelimKind::Brace, DelimOrientation::Close) => '}',
        }
    }
}

impl Tokenize for Delimiter {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = TypeError {
            error_kind: TypeErrorKind::UnrecognizedDelimiter,
            position: Position::new(src, start),
        };

        // convert `TypeErrorKind` to `CompilerError::Type(TypeError)`
        let delim_kind = DelimKind::from_str(content)
            .map_err(|_| handler.emit_err(CompilerError::Type(error.clone())))?;

        // convert `TypeErrorKind` to `CompilerError::Type(TypeError)`
        let delim_orientation = DelimOrientation::from_str(content)
            .map_err(|_| handler.emit_err(CompilerError::Type(error)))?;

        let delimiter = Delimiter::new(delim_kind, delim_orientation, span);

        let token = Token::Delim(delimiter);

        Ok(Some(token))
    }
}

impl Spanned for Delimiter {
    fn span(&self) -> &Span {
        &self.span
    }
}

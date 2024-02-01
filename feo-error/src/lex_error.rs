use std::error::Error;
use std::fmt;

use feo_types::span::Position;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum LexErrorKind {
    EmptyCharLiteral,
    InvalidCharLiteral,
    ExpectedCharLiteral,
    ExpectedClosingSingleQuote,
    InvalidEscapeSequence,
    ExpectedEscapeSequence,
    UnclosedBlockComment,
    UnclosedDelimiters,
    UnopenedDelimiters,
    MismatchedDelimiters,

    InvalidChar(char),

    #[default]
    UnknownError,
}

impl fmt::Display for LexErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexErrorKind::EmptyCharLiteral => write!(f, "detected empty char literal"),
            LexErrorKind::InvalidCharLiteral => write!(f, "invalid char literal"),
            LexErrorKind::ExpectedCharLiteral => write!(f, "expected char literal"),
            LexErrorKind::ExpectedClosingSingleQuote => write!(f, "expected closing single quote"),
            LexErrorKind::InvalidEscapeSequence => write!(f, "invalid escape sequence"),
            LexErrorKind::ExpectedEscapeSequence => write!(f, "expected escape sequence"),
            LexErrorKind::UnclosedBlockComment => write!(f, "unclosed block comment"),
            LexErrorKind::UnclosedDelimiters => write!(f, "unclosed delimiters"),
            LexErrorKind::UnopenedDelimiters => write!(f, "unopened delimiters"),
            LexErrorKind::MismatchedDelimiters => write!(f, "mismatched delimiters"),
            LexErrorKind::InvalidChar(c) => write!(f, "invalid char (`{}`)", c),
            LexErrorKind::UnknownError => write!(f, "unknown error"),
        }
    }
}

impl Error for LexErrorKind {}

#[derive(Default, Debug, Clone)]
pub struct LexError {
    pub error_kind: LexErrorKind,
    pub position: Position,
}

impl Error for LexError {}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}:{}",
            self.error_kind,
            self.position.line_col().0,
            self.position.line_col().1
        )
    }
}

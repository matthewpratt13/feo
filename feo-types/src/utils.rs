use std::error::Error;
use std::fmt;

use crate::{delimiter::Delimiter, punctuation::Punctuation};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum TypeErrorKind {
    UnrecognizedCommentOpener,
    UnrecognizedDelimiter,
    UnrecognizedKeyword,
    UnexpectedPunctuation,
    InvalidTypeAnnotation,

    #[default]
    UnknownError,
}

impl fmt::Display for TypeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeErrorKind::UnrecognizedCommentOpener => {
                write!(f, "unrecognized comment open sequence")
            }
            TypeErrorKind::UnrecognizedDelimiter => write!(f, "unrecognized delimiter"),
            TypeErrorKind::UnrecognizedKeyword => write!(f, "unrecognized keyword"),
            TypeErrorKind::UnexpectedPunctuation => write!(f, "unexpected punctuation"),
            TypeErrorKind::InvalidTypeAnnotation => write!(f, "invalid type annotation"),
            TypeErrorKind::UnknownError => write!(f, "unknown error"),
        }
    }
}

impl Error for TypeErrorKind {}

pub type Asterisk = Punctuation;
pub type Bang = Punctuation;
pub type Colon = Punctuation;
pub type Comma = Punctuation;
pub type DblColon = Punctuation;
pub type DblDot = Punctuation;
pub type DblPipe = Punctuation;
pub type Dot = Punctuation;
pub type DotDotEquals = Punctuation;
pub type Equals = Punctuation;
pub type FatArrow = Punctuation;
pub type HashBang = Punctuation;
pub type HashSign = Punctuation;
pub type Minus = Punctuation;
pub type Pipe = Punctuation;
pub type Plus = Punctuation;
pub type QuestionMark = Punctuation;
pub type Semicolon = Punctuation;
pub type ThinArrow = Punctuation;
pub type Underscore = Punctuation;
pub type Brace = Delimiter;
pub type Bracket = Delimiter;
pub type Parenthesis = Delimiter;

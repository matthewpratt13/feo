use std::{error::Error, fmt};

use crate::error::Position;

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

#[derive(Default, Debug, Clone)]
pub struct TypeError {
    pub error_kind: TypeErrorKind,
    pub position: Position,
}

impl fmt::Display for TypeError {
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

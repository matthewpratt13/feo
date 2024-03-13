use std::error::Error;
use std::fmt;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum TypeErrorKind {
    UnrecognizedCommentOpener,
    UnrecognizedDelimiter,
    UnrecognizedKeyword,
    UnexpectedPunctuation,
    MismatchedTypes,
    ValueNotFound,

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
            TypeErrorKind::UnknownError => write!(f, "unknown error"),

            TypeErrorKind::MismatchedTypes => write!(f, "original type does not new value's type"),

            TypeErrorKind::ValueNotFound => {
                write!(f, "value not found")
            }
        }
    }
}

impl Error for TypeErrorKind {}

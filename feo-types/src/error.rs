use std::error::Error;
use std::fmt;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum TypeErrorKind {
    UnrecognizedCommentOpener,
    UnrecognizedDelimiter,
    UnrecognizedKeyword,
    UnexpectedPunctuation,
    UnrecognizedBuiltInTypeAnnotation,
    MismatchedTypeAnnotation,

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
            TypeErrorKind::UnrecognizedBuiltInTypeAnnotation => {
                write!(f, "unrecognized built-in type annotation")
            }
            TypeErrorKind::MismatchedTypeAnnotation => {
                write!(f, "type annotation does not match value's type")
            }
        }
    }
}

impl Error for TypeErrorKind {}

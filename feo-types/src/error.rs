use std::error::Error;
use std::fmt;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum TypeErrorKind {
    UnrecognizedCommentOpener,
    UnrecognizedDelimiter,
    UnrecognizedKeyword,
    UnexpectedPunctuation,
    MismatchedTypeAnnotation,
    MismatchedIntTypeAnnotation,
    MismatchedUIntTypeAnnotation,
    MismatchedFloatTypeAnnotation,

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
            TypeErrorKind::MismatchedTypeAnnotation => {
                write!(f, "type annotation does not match value's type")
            }
            TypeErrorKind::MismatchedIntTypeAnnotation => {
                write!(f, "int type annotation does not match value's type")
            }
            TypeErrorKind::MismatchedUIntTypeAnnotation => {
                write!(f, "uint type annotation does not match value's type")
            }
            TypeErrorKind::MismatchedFloatTypeAnnotation => {
                write!(f, "float type annotation does not match value's type")
            }
        }
    }
}

impl Error for TypeErrorKind {}

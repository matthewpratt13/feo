use feo_types::span::Position;

use crate::error::FeoError;

#[derive(Default, Debug, Copy, Clone)]
pub enum TypeErrorKind {
    UnrecognizedCommentOpener,
    UnrecognizedDelimiter,
    UnrecognizedKeyword,
    UnrecognizedPunctuation,
    UnrecognizedBuiltInTypeAnnotation,

    #[default]
    UnknownError,
}

impl FeoError for TypeErrorKind {}

#[derive(Default, Debug, Clone)]
pub struct TypeError {
    pub error_kind: TypeErrorKind,
    pub position: Position,
}

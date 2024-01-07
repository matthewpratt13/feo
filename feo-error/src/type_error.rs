use feo_types::span::Position;

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

#[derive(Default, Debug, Clone)]
pub struct TypeError {
    pub error_kind: TypeErrorKind,
    pub position: Position,
}

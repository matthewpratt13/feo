use feo_types::span::Position;

#[derive(Debug, Clone)]
pub enum TypeErrorKind {
    UnrecognizedCommentOpener,
    UnrecognizedDelimiter,
    UnrecognizedKeyword,
    UnrecognizedPunctuation,
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub error_kind: TypeErrorKind,
    pub position: Position,
}

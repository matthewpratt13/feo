use feo_types::span::Position;

#[derive(Debug, Clone)]
pub enum TypeErrorKind {
    UnrecognizedDelimiter,
    UnrecognizedKeyword,
    UnrecognizedPunctuation,
}

#[derive(Debug, Clone)]
pub struct TypeError<'a> {
    pub error_kind: TypeErrorKind,
    pub pos: Option<Position<'a>>,
}

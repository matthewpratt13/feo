#[derive(Debug, Clone)]
pub enum TypeErrorKind {
    UnrecognizedDelimiter,
    UnrecognizedKeyword,
    UnrecognizedPunctuation,
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub error_kind: TypeErrorKind,
    pub pos: usize,
}
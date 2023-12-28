#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    ParseCharError,
    ParseBoolError,
    ParseIntError,
    ParseUIntError,
    ParseFloatError,
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub error_kind: ParserErrorKind,
    pub pos: usize,
}

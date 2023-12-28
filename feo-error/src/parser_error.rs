use feo_types::span::Position;

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    ParseCharError,
    ParseBoolError,
    ParseIntError,
    ParseUIntError,
    ParseFloatError,
}

#[derive(Debug, Clone)]
pub struct ParserError<'a> {
    pub error_kind: ParserErrorKind,
    pub pos: Option<Position<'a>>,
}

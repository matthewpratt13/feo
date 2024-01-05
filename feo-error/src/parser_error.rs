use feo_types::span::Position;

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    ParseCharError,
    ParseBoolError,
    ParseIntError,
    ParseUIntError,
    ParseU256Error,
    ParseFloatError,
    CharPositionNotFound,
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub error_kind: ParserErrorKind,
    pub position: Position,
}

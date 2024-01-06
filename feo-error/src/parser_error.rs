use feo_types::span::Position;

use crate::error::FeoError;

#[derive(Default, Debug, Copy, Clone)]
pub enum ParserErrorKind {
    ParseCharError,
    ParseBoolError,
    ParseIntError,
    ParseUIntError,
    ParseU256Error,
    ParseFloatError,
    CharPositionNotFound,

    #[default]
    UnknownError,
}

impl FeoError for ParserErrorKind {}

#[derive(Default, Debug, Clone)]
pub struct ParserError {
    pub error_kind: ParserErrorKind,
    pub position: Position,
}

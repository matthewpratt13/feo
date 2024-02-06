use std::error::Error;
use std::fmt;

use feo_types::span::Position;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum ParserErrorKind {
    ParseCharError,
    ParseBoolError,
    ParseIntError,
    ParseUIntError,
    ParseU256Error,
    ParseFloatError,
    CharPositionNotFound,
    MismatchedTokens,
    TokenNotFound,
    InvalidToken,
    UnexpectedToken,

    #[default]
    UnknownError,
}

impl fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrorKind::ParseCharError => write!(f, "unable to parse char"),
            ParserErrorKind::ParseBoolError => write!(f, "unable to parse bool"),
            ParserErrorKind::ParseIntError => write!(f, "unable to parse int"),
            ParserErrorKind::ParseUIntError => write!(f, "unable to parse uint"),
            ParserErrorKind::ParseU256Error => write!(f, "unable to parse u256"),
            ParserErrorKind::ParseFloatError => write!(f, "unable to parse float"),
            ParserErrorKind::CharPositionNotFound => write!(f, "cannot detect char position"),
            ParserErrorKind::MismatchedTokens => write!(f, "mismatched tokens"),
            ParserErrorKind::TokenNotFound => write!(f, "token not found"),
            ParserErrorKind::InvalidToken => write!(f, "invalid token"),
            ParserErrorKind::UnexpectedToken => write!(f, "unexpected token"),
            ParserErrorKind::UnknownError => write!(f, "unknown error"),
        }
    }
}

impl Error for ParserErrorKind {}

#[derive(Default, Debug, Clone)]
pub struct ParserError {
    pub error_kind: ParserErrorKind,
    pub position: Position,
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}:{}",
            self.error_kind,
            self.position.line_col().0,
            self.position.line_col().1
        )
    }
}

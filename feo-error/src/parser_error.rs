use std::error::Error;
use std::fmt;

use feo_types::keyword::KeywordKind;
use feo_types::punctuation::PuncKind;
use feo_types::span::Position;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum ParserErrorKind {
    ParseCharError,
    ParseBoolError,
    ParseIntError,
    ParseUIntError,
    ParseU256Error,
    ParseFloatError,
    ParseTypeAnnotationError,
    CharPositionNotFound,

    MissingDelimiter {
        delim: String,
    },

    InvalidKeyword {
        keyword_kind: KeywordKind,
    },

    InvalidPunctuation {
        punc_kind: PuncKind,
    },

    InvalidToken {
        token: String,
    },

    UnexpectedToken {
        expected: String,
        found: String,
    },

    TokenNotFound,

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
            ParserErrorKind::ParseTypeAnnotationError => {
                write!(f, "unable to parse type annotation")
            }
            ParserErrorKind::CharPositionNotFound => write!(f, "cannot detect character position"),
            ParserErrorKind::MissingDelimiter { delim } => {
                write!(f, "missing delimiter: `{}`", delim)
            }
            ParserErrorKind::InvalidKeyword { keyword_kind } => {
                write!(
                    f,
                    "invalid `KeywordKind` in this context: {}",
                    keyword_kind.as_str()
                )
            }
            ParserErrorKind::InvalidPunctuation { punc_kind } => {
                write!(
                    f,
                    "invalid `PuncKind` in this context: {}",
                    punc_kind.as_str()
                )
            }
            ParserErrorKind::InvalidToken { token } => {
                write!(f, "invalid token in this context: {}", token)
            }
            ParserErrorKind::UnexpectedToken { expected, found } => write!(
                f,
                "unexpected token. expected {}, found {}",
                expected, found,
            ),
            ParserErrorKind::TokenNotFound => write!(f, "token not found"),
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

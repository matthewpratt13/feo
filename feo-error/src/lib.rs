pub mod error;
pub mod handler;
pub mod lex_error;
pub mod parser_error;
pub mod type_error;
pub mod warning;

impl std::error::Error for lex_error::LexErrorKind {}

impl std::fmt::Display for lex_error::LexErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for parser_error::ParserErrorKind {}

impl std::fmt::Display for parser_error::ParserErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for type_error::TypeErrorKind {}

impl std::fmt::Display for type_error::TypeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

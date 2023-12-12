use thiserror::Error;

use feo_types::Span;

#[derive(Debug, Error)]
pub enum ParserErrorKind {
    #[error("token not found")]
    TokenNotFound,
}

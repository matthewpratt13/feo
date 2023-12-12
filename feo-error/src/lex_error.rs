use thiserror::Error;

use feo_types::{Span, SpanError};

#[derive(Debug, Error)]
pub enum LexErrorKind {
    #[error("source file empty")]
    SourceFileEmpty,
    #[error("invalid char")]
    InvalidChar,
    #[error("unclosed delimiters")]
    UnclosedDelimiters
}

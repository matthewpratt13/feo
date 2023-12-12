use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexErrorKind {
    #[error("token not found")]
    TokenNotFound,
    #[error("source file empty")]
    SourceFileEmpty,
    #[error("invalid char")]
    InvalidChar,
    #[error("unclosed delimiters")]
    UnclosedDelimiters,
    #[error("unopened block comment")]
    UnopenedBlockComment,
    #[error("span error")]
    SpanError(#[from] feo_types::SpanError),
}

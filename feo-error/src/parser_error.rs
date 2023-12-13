use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserErrorKind {
    #[error("unable to parse bool")]
    ParseBoolError,
    #[error("unable to parse char")]
    ParseCharError,
    #[error("unable to parse float")]
    ParseFloatError,
    #[error("unable to parse int")]
    ParseIntError,
    #[error("span")]
    SpanError(#[from] feo_types::SpanError),
}

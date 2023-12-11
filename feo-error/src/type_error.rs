use thiserror::Error;

use feo_types::Span;

#[derive(Debug, Error)]
pub enum TypeErrorKind {
    #[error("unrecognized delimiter")]
    DelimiterError(#[from] feo_types::DelimiterError),
    #[error("span error")]
    SpanError(#[from] feo_types::SpanError),
}

pub struct TypeError {
    error_kind: TypeErrorKind,
    span: Span,
}

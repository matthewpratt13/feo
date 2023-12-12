use thiserror::Error;

use feo_types::Span;

#[derive(Debug, Error)]
pub enum TypeErrorKind {
    // TODO: remove these conversions - this crate should not know about feo_types
    #[error("unrecognized delimiter")]
    DelimiterError(#[from] feo_types::DelimiterError),
    #[error("span error")]
    SpanError(#[from] feo_types::SpanError),
}

pub struct TypeError {
    error_kind: TypeErrorKind,
    span: Span,
}

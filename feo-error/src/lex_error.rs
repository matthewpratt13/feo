use feo_types::Span;

pub enum LexErrorKind {}

pub struct LexError {
    error_kind: LexErrorKind,
    span: Span,
}

use crate::span::{Span, Spanned};

pub struct Identifier {
    name: String,
    span: Span,
}

impl Spanned for Identifier {
    fn span(&self) -> &Span {
        &self.span
    }
}

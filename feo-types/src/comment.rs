use crate::span::{Span, Spanned};

#[derive(Debug)]
pub struct Comment {
    pub content: String,
    span: Span,
}

impl Spanned for Comment {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub struct DocComment {
    pub content: String,
    span: Span,
}

impl Spanned for DocComment {
    fn span(&self) -> &Span {
        &self.span
    }
}

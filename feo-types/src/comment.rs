use crate::span::{Span, Spanned};

#[derive(Debug)]
pub struct Comment {
    pub content: String,
    span: Span,
}

impl Comment {
    pub fn new(content: String, span: Span) -> Self {
        Self { content, span }
    }
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

impl DocComment {
    pub fn new(content: String, span: Span) -> Self {
        Self { content, span }
    }
}

impl Spanned for DocComment {
    fn span(&self) -> &Span {
        &self.span
    }
}

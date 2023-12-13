use std::sync::Arc;

use crate::span::{Span, Spanned};

#[derive(Debug)]
pub enum CommentKind {
    LineComment,
    BlockComment,
}

#[derive(Debug)]
pub struct GenericComment {
    span: Span,
}

impl Spanned for GenericComment {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub struct DocComment {
    comment_kind: CommentKind,
    contents: Arc<String>,
    span: Span,
}

impl Spanned for DocComment {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub struct DocComment {
    comment_kind: CommentKind,
    span: Span,
}

impl Spanned for DocComment {
    fn span(&self) -> &Span {
        &self.span
    }
}

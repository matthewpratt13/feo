use crate::span::{Span, Spanned};

#[derive(Debug)]
pub enum CommentKind {
    Newline,
    Trailing,
    Inline,
    Multiline,
}

#[derive(Debug)]
pub struct Comment {
    comment_kind: CommentKind,
    span: Span,
}

impl Spanned for Comment {
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

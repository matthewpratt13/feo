use crate::span::{Span, Spanned};

pub enum CommentKind {
    Newline,
    Trailing,
    Inline,
    Multiline,
}

pub struct Comment {
    comment_kind: CommentKind,
    span: Span,
}

impl Spanned for Comment {
    fn span(&self) -> &Span {
        &self.span
    }
}

pub struct DocComment {
    comment_kind: CommentKind,
    span: Span,
}

impl Spanned for DocComment {
    fn span(&self) -> &Span {
        &self.span
    }
}

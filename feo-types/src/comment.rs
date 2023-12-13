use crate::span::{Span, Spanned};

#[derive(Debug)]
pub enum CommentKind {
    Newline, // `LineComment`
    Trailing, // `LineComment`
    Inline, // `DocComment`
    Multiline, // `DocComment`
}

#[derive(Debug)]
pub struct LineComment {
    comment_kind: CommentKind,
    span: Span,
}

impl Spanned for LineComment {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub struct BlockComment {
    comment_kind: CommentKind,
    span: Span,
}

impl Spanned for BlockComment {
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

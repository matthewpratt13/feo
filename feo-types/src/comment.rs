use crate::span::{Span, Spanned};

#[derive(Debug)]
pub enum CommentKind {
    Line,
    Block,
    OuterDocComment,
    ModuleDocComment,
}

#[derive(Debug)]
pub struct Comment {
    comment_kind: CommentKind,
    start: usize,
    end: usize,
}

impl Comment {
    pub fn new(comment_kind: CommentKind, start: usize, end: usize) -> Self {
        Self {
            comment_kind,
            start,
            end,
        }
    }
}

#[derive(Debug)]
pub struct DocComment {
    comment_kind: CommentKind,
    span: Span,
}

impl DocComment {
    pub fn new(comment_kind: CommentKind, span: Span) -> Self {
        Self { comment_kind, span }
    }
}
impl Spanned for DocComment {
    fn span(&self) -> &Span {
        &self.span
    }
}

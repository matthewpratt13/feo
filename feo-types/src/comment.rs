use crate::span::{Span, Spanned};

#[derive(Debug, Clone)]
pub enum CommentKind {
    LineComment,  // slash-slash
    BlockComment, // open: slash-asterisk | close: asterisk-slash
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub comment_kind: CommentKind,
    pub data: String,
    span: Span,
}

impl Comment {
    pub fn new(comment_kind: CommentKind, data: String, span: Span) -> Self {
        Self {
            comment_kind,
            data,
            span,
        }
    }
}

impl Spanned for Comment {
    fn span(&self) -> Span {
        self.clone().span
    }
}

use crate::span::{Span, Spanned};

#[derive(Debug, Clone)]
pub enum DocCommentKind {
    InnerDocComment, // slash-slash-bang
    OuterDocComment, // slash-slash-slash
}

#[derive(Debug, Clone)]
pub struct DocComment {
    pub doc_comment_kind: DocCommentKind,
    pub content: String,
    span: Span,
}

impl DocComment {
    pub fn new(doc_comment_kind: DocCommentKind, content: String, span: Span) -> Self {
        Self {
            doc_comment_kind,
            content,
            span,
        }
    }
}

impl Spanned for DocComment {
    fn span(&self) -> Span {
        self.clone().span
    }
}

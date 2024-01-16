use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    type_error::{TypeError, TypeErrorKind},
};
use feo_types::span::{Position, Span, Spanned};

use crate::token::{Token, Tokenize};

#[derive(Debug, Clone)]
pub enum DocCommentKind {
    InnerDocComment,
    OuterDocComment,
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

impl Tokenize for DocComment {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let mut outer_doc_comment = String::from("//");
        outer_doc_comment.push('!');

        let doc_comment = match content {
            _ if content.starts_with("///") => DocComment::new(
                DocCommentKind::InnerDocComment,
                content
                    .strip_prefix("///")
                    .expect("Unable to process inner doc comment")
                    .trim()
                    .to_string(),
                span,
            ),

            _ if content.starts_with(&outer_doc_comment) => DocComment::new(
                DocCommentKind::OuterDocComment,
                content
                    .strip_prefix(&outer_doc_comment)
                    .expect("Unable to process outer doc comment")
                    .trim()
                    .to_string(),
                span,
            ),

            _ => {
                let error = TypeError {
                    error_kind: TypeErrorKind::UnrecognizedCommentOpener,
                    position: Position::new(src, start),
                };

                return Err(handler.emit_err(CompilerError::Type(error)));
            }
        };

        let token = Token::DocComment(doc_comment);

        Ok(Some(token))
    }
}

impl Spanned for DocComment {
    fn span(&self) -> Span {
        self.clone().span
    }
}

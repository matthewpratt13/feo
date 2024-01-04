use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    type_error::{TypeError, TypeErrorKind},
};
use feo_types::span::{Position, Span, Spanned};

use crate::token::{Token, Tokenize};

#[derive(Debug, Clone)]
pub enum CommentKind {
    LineComment,
    BlockComment,
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

impl Tokenize for Comment {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let comment = match content {
            _ if content.starts_with("//") => {
                Comment::new(CommentKind::LineComment, content.to_string(), span)
            }

            _ if content.starts_with("/*") => {
                Comment::new(CommentKind::BlockComment, content.to_string(), span)
            }

            _ => {
                let err = TypeError {
                    error_kind: TypeErrorKind::UnrecognizedCommentOpener,
                    position: Position::new(src, start),
                };

                return Err(handler.emit_err(CompilerError::Type(err)));
            }
        };

        let token = Token::Comment(comment);

        Ok(Some(token))
    }
}

impl Spanned for Comment {
    fn span(&self) -> &Span {
        &self.span
    }
}

use feo_error::handler::{ErrorEmitted, Handler};
use feo_types::span::{Span, Spanned};

use crate::token::{Token, Tokenize};

#[derive(Debug, Clone)]
pub struct DocComment {
    pub content: String,
    span: Span,
}

impl DocComment {
    pub fn new(content: String, span: Span) -> Self {
        Self { content, span }
    }
}

impl Tokenize for DocComment {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let doc_comment = DocComment::new(content.to_string(), span);

        let token = Token::DocComment(doc_comment);

        Ok(Some(token))
    }
}

impl Spanned for DocComment {
    fn span(&self) -> &Span {
        &self.span
    }
}

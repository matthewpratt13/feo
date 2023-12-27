use std::sync::Arc;

use feo_error::error::ErrorEmitted;

use feo_types::span::Span;
use feo_types::DocComment;

use crate::token::{Token, Tokenize};

impl Tokenize for DocComment {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let doc_comment = DocComment::new(content.to_string(), span);

        let token = Token::DocComment(doc_comment);

        Ok(Some(token))
    }
}

use std::sync::Arc;

use feo_error::error::ErrorEmitted;

use feo_types::span::Span;
use feo_types::Identifier;

use crate::token::{Token, Tokenize};

impl Tokenize for Identifier {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let iden = Identifier::new(content.to_string(), span);

        let token = Token::Iden(iden);

        Ok(Some(token))
    }
}

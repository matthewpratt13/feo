use feo_error::error::ErrorEmitted;
use feo_types::span::{Span, Spanned};

use crate::token::{Token, Tokenize};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    span: Span,
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}

impl Tokenize for Identifier {
    fn tokenize(
        src: &str,
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

impl Spanned for Identifier {
    fn span(&self) -> &Span {
        &self.span
    }
}

use feo_error::handler::{ErrorEmitted, Handler};
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
        _handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let identifier = Identifier::new(content.to_string(), span);

        let token = Token::Iden(identifier);

        Ok(Some(token))
    }
}

impl Spanned for Identifier {
    fn span(&self) -> &Span {
        &self.span
    }
}

pub fn is_keyword(iden: &str) -> bool {
    [
        "abstract",
        "as",
        "break",
        "const",
        "continue",
        "contract",
        "crate",
        "deref",
        "else",
        "enum",
        "extern",
        "for",
        "func",
        "if",
        "impl",
        "import",
        "in",
        "interface",
        "let",
        "library",
        "loop",
        "match",
        "mod",
        "mut",
        "None",
        "program",
        "pub",
        "ref",
        "return",
        "script",
        "self",
        "Some",
        "static",
        "struct",
        "super",
        "trait",
        "type",
        "unsafe",
        "while",
    ]
    .contains(&iden)
}

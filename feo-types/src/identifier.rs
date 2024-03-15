use crate::span::{Span, Spanned};

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

pub fn is_keyword(iden: &str) -> bool {
    [
        "abi", "abstract", "as", "break", "const", "continue", "contract", "crate", "else", "enum",
        "export", "extern", "for", "func", "if", "impl", "import", "in", "let", "library", "loop",
        "match", "mod", "mut", "payable", "pub", "ref", "return", "script", "self", "Self",
        "static", "storage", "struct", "super", "test", "topic", "trait", "type", "unsafe",
        "where", "while",
    ]
    .contains(&iden)
}

impl Spanned for Identifier {
    fn span(&self) -> Span {
        self.clone().span
    }
}

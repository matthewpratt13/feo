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
        "abi", "abstract", "as", "break", "const", "continue", "contract", "crate", "deref", "dyn",
        "else", "enum", "export", "extern", "for", "func", "if", "impl", "import", "in", "let",
        "library", "loop", "match", "mod", "mut", "None", "program", "pub", "ref", "return",
        "script", "self", "Self", "static", "storage", "struct", "super", "trait", "type",
        "unsafe", "where", "while",
    ]
    .contains(&iden)
}

impl Spanned for Identifier {
    fn span(&self) -> Span {
        self.clone().span
    }
}

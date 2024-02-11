use crate::{
    span::{Span, Spanned},
    TypeAnnotation,
};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    span: Span,
    type_ann_opt: Option<TypeAnnotation>,
}

impl Identifier {
    pub fn new(name: String, span: Span, type_ann_opt: Option<TypeAnnotation>) -> Self {
        Self {
            name,
            span,
            type_ann_opt,
        }
    }

    pub fn type_annotation(&self) -> Option<TypeAnnotation> {
        self.type_ann_opt.clone()
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

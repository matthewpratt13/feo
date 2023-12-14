use crate::span::{Span, Spanned};

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    span: Span,
}

impl Spanned for Identifier {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub struct PathExpression {
    pub path: Vec<String>,
    span: Span,
}

impl Spanned for PathExpression {
    fn span(&self) -> &Span {
        &self.span
    }
}

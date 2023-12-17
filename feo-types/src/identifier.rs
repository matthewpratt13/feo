use crate::span::{Span, Spanned};

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    span: Span,
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
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

impl PathExpression {
    pub fn new(path: Vec<String>, span: Span) -> Self {
        Self { path, span }
    }
}

impl Spanned for PathExpression {
    fn span(&self) -> &Span {
        &self.span
    }
}

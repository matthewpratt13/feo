use std::fmt;

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

impl Spanned for Identifier {
    fn span(&self) -> &Span {
        &self.span
    }
}

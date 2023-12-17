use std::fmt;

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
    pub fn new(path: String, span: Span) -> Self {
        let split = path.as_str().split("::").into_iter().map(|s| s.to_string()).collect();

        Self { path: split, span }
    }
}

impl Spanned for PathExpression {
    fn span(&self) -> &Span {
        &self.span
    }
}

impl fmt::Display for PathExpression {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_set().entries(self.path.iter()).finish()
    }
}

use std::sync::Arc;

use crate::span::{Span, Spanned};

#[derive(Debug)]
pub struct Comment {
    span: Span,
}

impl Comment {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            span: Span::new(Arc::new(String::new()), start, end),
        }
    }
}

impl Spanned for Comment {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub struct DocComment {
    span: Span,
}

impl DocComment {
    pub fn new(input: &str, start: usize, end: usize) -> Self {
        Self {
            span: Span::new(Arc::new(input.to_string()), start, end),
        }
    }
}

impl Spanned for DocComment {
    fn span(&self) -> &Span {
        &self.span
    }
}

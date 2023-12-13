use std::sync::Arc;

pub trait Spanned {
    fn span(&self) -> &Span;
}

#[derive(Debug, Clone)]
pub struct Span {
    src: Arc<String>,
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(src: Arc<String>, start: usize, end: usize) -> Self {
        Self { src, start, end }
    }


}

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

    pub fn source(&self) -> Arc<String> {
        Arc::clone(&self.src)
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

use thiserror::Error;

use std::sync::Arc;

pub trait Spanned {
    fn span(&self) -> &Span;
}

pub struct Span {
    src: Arc<&'static str>,
    start: usize,
    end: usize,
}

impl Span {
    pub fn build(src: &str, start: usize, end: usize) -> Result<Self, SpanError> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum SpanError {
    #[error("index is out of range")]
    IndexOutOfRange,
    #[error("end index is before start")]
    EndIndexBeforeStart,
    #[error("source file is empty")]
    SourceFileEmpty,
}

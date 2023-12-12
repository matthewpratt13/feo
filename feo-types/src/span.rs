use std::sync::Arc;

use thiserror::Error;

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
    pub fn build(src: &str, start: usize, end: usize) -> Result<Self, SpanError> {
        if start >= src.len() || end >= src.len() {
            return Err(SpanError::IndexOutOfRange)?;
        }

        if start > end {
            return Err(SpanError::EndIndexBeforeStart)?;
        }

        Ok(Self {
            src: Arc::new(src.to_string()),
            start,
            end,
        })
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

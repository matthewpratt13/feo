use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeErrorKind {
    #[error("delimiter error")]
    DelimiterError(#[from] feo_types::DelimiterError),
}

use feo_types::{
    span::{Span, Spanned},
    Keyword,
};

#[derive(Debug, Clone)]
pub struct SelfType(pub Keyword);

impl Spanned for SelfType {
    fn span(&self) -> Span {
        self.0.span()
    }
}

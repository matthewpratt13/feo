use feo_types::{
    span::{Span, Spanned},
    type_utils::Underscore,
};

#[derive(Debug, Clone)]
pub struct WildcardPatt(pub Underscore);

impl Spanned for WildcardPatt {
    fn span(&self) -> Span {
        self.0.span()
    }
}

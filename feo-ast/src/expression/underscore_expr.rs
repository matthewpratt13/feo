use feo_types::{
    span::{Span, Spanned},
    type_utils::Underscore,
};

#[derive(Debug, Clone)]
pub struct UnderscoreExpr(pub Underscore);

impl Spanned for UnderscoreExpr {
    fn span(&self) -> Span {
        self.0.span()
    }
}

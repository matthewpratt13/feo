use feo_types::{
    span::{Span, Spanned},
    utils::Underscore,
};

#[derive(Debug, Clone)]
pub struct UnderscoreExpr {
    pub underscore: Underscore,
}

impl Spanned for UnderscoreExpr {
    fn span(&self) -> Span {
        self.underscore.span()
    }
}

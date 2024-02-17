use feo_types::{
    span::{Span, Spanned},
    utils::Underscore,
};

#[derive(Clone)]
pub struct UnderscoreExpr {
    underscore: Underscore,
}

impl Spanned for UnderscoreExpr {
    fn span(&self) -> Span {
        self.underscore.span()
    }
}

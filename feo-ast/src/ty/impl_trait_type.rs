use crate::{
    keyword::Keyword,
    path::PathType,
    span::{Span, Spanned},
};

use super::Type;

// (one bound)
pub struct ImplTraitType {
    kw_impl: Keyword,
    trait_bound: TraitBound,
}

pub type TraitBound = PathType;

impl Type for ImplTraitType {}

impl Spanned for ImplTraitType {
    fn span(&self) -> Span {
        let start_pos = self.kw_impl.span().start();
        let end_pos = self.trait_bound.span().end();
        let source = self.kw_impl.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

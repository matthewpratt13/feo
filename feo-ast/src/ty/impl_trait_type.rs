use feo_types::{
    span::{Span, Spanned},
    utils::KwImpl,
};

use crate::path::PathType;

// (one bound)
#[derive(Clone)]
pub struct ImplTraitType {
    kw_impl: KwImpl,
    trait_bound: TraitBound,
}

pub type TraitBound = PathType;

impl Spanned for ImplTraitType {
    fn span(&self) -> Span {
        let start_pos = self.kw_impl.span().start();
        let end_pos = self.trait_bound.span().end();
        let source = self.kw_impl.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

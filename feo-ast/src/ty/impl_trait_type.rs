use feo_types::{
    span::{Span, Spanned},
    utils::KwImpl,
};

use crate::path::PathType;

// (one bound)
#[derive(Debug, Clone)]
pub struct ImplTraitType {
    kw_impl: KwImpl,
    trait_bound: TraitBound,
}

pub type TraitBound = PathType;

impl Spanned for ImplTraitType {
    fn span(&self) -> Span {
        let s1 = self.kw_impl.span();
        let s2 = self.trait_bound.span();

        Span::join(s1, s2)
    }
}

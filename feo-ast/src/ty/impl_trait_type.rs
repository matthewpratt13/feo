use crate::{
    item::{Plus, QuestionMark},
    keyword::KeywordKind,
    path::SimplePath,
};

pub struct ImplTraitType {
    kw_impl: KeywordKind,
    trait_bounds: TraitBounds,
}

pub struct TraitBounds {
    first_trait_bound: TraitBound,
    subsequent_trait_bounds: Vec<(Plus, TraitBound)>,
}

pub struct TraitBound {
    question_mark: QuestionMark,
    path: SimplePath,
}

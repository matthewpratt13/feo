use crate::{keyword::KeywordKind, path::SimplePath};

use self::impl_trait_type::{ImplTraitType, TraitBounds};

mod impl_trait_type;

pub enum Type {
    ImplTrait(ImplTraitType),
    TraitObject(TraitObjectType),
    TypePath(SimplePath),
    Tuple,
    Reference,
    Array,
    Slice,
    Inferred,
    QualifiedPathIn,
}

pub struct TraitObjectType {
    kw_dyn: KeywordKind,
    trait_bounds: TraitBounds,
}

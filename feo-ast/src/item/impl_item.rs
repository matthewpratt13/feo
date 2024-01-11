use crate::{keyword::KeywordKind, ty::Type};

use super::associated_item::AssociatedItem;
use super::Brace;

pub enum ImplItem {
    Inherent(InherentImpl),
    Trait(TraitImpl),
}

pub struct InherentImpl {
    kw_impl: KeywordKind,
    object_type: Type,
    open_brace: Brace,
    associated_items: Vec<AssociatedItem>,
    close_brace: Brace,
}

pub struct TraitImpl {
    kw_unsafe_opt: Option<KeywordKind>,
    kw_trait: KeywordKind,
    trait_type: Type,
    kw_for: KeywordKind,
    object_type: Type,
    open_brace: Brace,
    associated_items: Vec<AssociatedItem>,
    close_brace: Brace,
}

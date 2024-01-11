use crate::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    ty::Type,
};

use super::associated_item::AssociatedItem;

pub enum ImplItem {
    Inherent(InherentImpl),
    Trait(TraitImpl),
}

pub struct InherentImpl {
    kw_impl: KeywordKind,
    object_type: Type,
    open_brace: (DelimKind, DelimOrientation),
    associated_items: Vec<AssociatedItem>,
    close_brace: (DelimKind, DelimOrientation),
}

pub struct TraitImpl {
    kw_unsafe_opt: Option<KeywordKind>,
    kw_trait: KeywordKind,
    trait_type: Type,
    kw_for: KeywordKind,
    object_type: Type,
    open_brace: (DelimKind, DelimOrientation),
    associated_items: Vec<AssociatedItem>,
    close_brace: (DelimKind, DelimOrientation),
}

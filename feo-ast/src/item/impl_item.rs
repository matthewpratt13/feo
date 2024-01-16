use crate::{keyword::KeywordKind, path::SimplePath, ty::Type, type_utils::Brace};

use super::{AssociatedItem, VisibilityKind};

pub struct InherentImpl<T> {
    visibility_opt: Option<VisibilityKind>,
    kw_impl: KeywordKind,
    object_type: Box<dyn Type>,
    open_brace: Brace,
    associated_items: Vec<Box<dyn AssociatedItem<T>>>,
    close_brace: Brace,
}

pub struct TraitImpl<T> {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: Option<KeywordKind>,
    kw_impl: KeywordKind,
    trait_path: SimplePath,
    kw_for: KeywordKind,
    object_type: Box<dyn Type>,
    open_brace: Brace,
    associated_items: Vec<Box<dyn AssociatedItem<T>>>,
    close_brace: Brace,
}

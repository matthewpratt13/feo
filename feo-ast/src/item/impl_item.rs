use crate::{
    expression::Attribute, keyword::KeywordKind, path::SimplePath, ty::Type, type_utils::Brace,
};

use super::{AssociatedItem, ConstantItem, FunctionItem, TypeAliasItem, VisibilityKind, Item};

// pub enum ImplItemKind {
//     Inherent(InherentImpl),
//     Trait(TraitImpl),
// }

// pub enum AssociatedItemKind {
//     Constant(ConstantItem),
//     Function(FunctionItem),
//     TypeAlias(TypeAliasItem),
// }

pub struct InherentImpl<T> {
    kw_impl: KeywordKind,
    object_type: Box<dyn Type>,
    open_brace: Brace,
    associated_items: Vec<Box<dyn AssociatedItem<T>>>,
    close_brace: Brace,
}

pub struct TraitImpl<T> {
    kw_unsafe_opt: Option<KeywordKind>,
    kw_impl: KeywordKind,
    trait_path: SimplePath,
    kw_for: KeywordKind,
    object_type: Box<dyn Type>,
    open_brace: Brace,
    associated_items: Vec<Box<dyn AssociatedItem<T>>>,
    close_brace: Brace,
}

// pub struct AssociatedItem {
//     attributes: Vec<Attribute>,
//     item: (Option<VisibilityKind>, AssociatedItemKind),
// }

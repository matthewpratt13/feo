use crate::{
    expression::Attribute, keyword::KeywordKind, path::SimplePath, ty::Type, type_utils::Brace,
};

use super::{ConstantItem, FunctionItem, TypeAliasItem, VisibilityKind};

pub enum ImplItemKind {
    Inherent(InherentImpl),
    Trait(TraitImpl),
}

pub enum AssociatedItemKind {
    Constant(ConstantItem),
    Function(FunctionItem),
    TypeAlias(TypeAliasItem),
}

pub struct InherentImpl {
    kw_impl: KeywordKind,
    object_type: Box<Type>,
    open_brace: Brace,
    associated_items: Vec<AssociatedItem>,
    close_brace: Brace,
}

pub struct TraitImpl {
    kw_unsafe_opt: Option<KeywordKind>,
    kw_impl: KeywordKind,
    trait_path: SimplePath,
    kw_for: KeywordKind,
    object_type: Box<Type>,
    open_brace: Brace,
    associated_items: Vec<AssociatedItem>,
    close_brace: Brace,
}

pub struct AssociatedItem {
    attributes: Vec<Attribute>,
    item: (Option<VisibilityKind>, AssociatedItemKind),
}

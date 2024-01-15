use crate::{
    expression::Attribute, keyword::KeywordKind, path::SimplePath, ty::Type, type_utils::Brace,
};

use super::{ConstantItem, FunctionItem, TypeAliasItem, VisibilityKind};

pub enum ImplItemKind<T> {
    Inherent(InherentImpl<T>),
    Trait(TraitImpl<T>),
}

pub enum AssociatedItemKind<T> {
    Constant(ConstantItem),
    Function(FunctionItem<T>),
    TypeAlias(TypeAliasItem),
}

pub struct InherentImpl<T> {
    kw_impl: KeywordKind,
    object_type: Box<Type>,
    open_brace: Brace,
    associated_items: Vec<AssociatedItem<T>>,
    close_brace: Brace,
}

pub struct TraitImpl<T> {
    kw_unsafe_opt: Option<KeywordKind>,
    kw_impl: KeywordKind,
    trait_path: SimplePath,
    kw_for: KeywordKind,
    object_type: Box<Type>,
    open_brace: Brace,
    associated_items: Vec<AssociatedItem<T>>,
    close_brace: Brace,
}

pub struct AssociatedItem<T> {
    attributes: Vec<Attribute>,
    item: (Option<VisibilityKind>, AssociatedItemKind<T>),
}

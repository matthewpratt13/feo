use crate::{
    expression::Attribute, keyword::KeywordKind, path::SimplePath, ty::Type, type_utils::Brace,
};

use super::{AssociatedItem, ImplItem, Item, VisibilityKind, WhereClause};

pub struct InherentImpl<T> {
    visibility_opt: Option<VisibilityKind>,
    kw_impl: KeywordKind,
    object_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    attributes: Vec<Attribute>,
    associated_items: Vec<Box<dyn AssociatedItem<T>>>,
    close_brace: Brace,
}

impl<T> Item for InherentImpl<T> {}

impl<T, I> ImplItem<I> for InherentImpl<T> where I: Item {}

pub struct TraitImpl<T> {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: Option<KeywordKind>,
    kw_impl: KeywordKind,
    trait_path: SimplePath,
    kw_for: KeywordKind,
    object_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    attributes: Vec<Attribute>,
    associated_items: Vec<Box<dyn AssociatedItem<T>>>,
    close_brace: Brace,
}

impl<T> Item for TraitImpl<T> {}

impl<T, I> ImplItem<I> for TraitImpl<T> where I: Item {}

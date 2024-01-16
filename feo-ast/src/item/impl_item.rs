use crate::{
    expression::Attribute, keyword::KeywordKind, path::SimplePath, ty::Type, type_utils::Brace,
};

use super::{AssociatedItem, VisibilityKind, WhereClause};

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

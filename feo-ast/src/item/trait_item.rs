use crate::{
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    type_utils::{Brace, Colon},
};

use super::{AssociatedItem, TypeParamBounds, VisibilityKind, WhereClause};

pub struct TraitItem<T> {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: Option<KeywordKind>,
    kw_impl: KeywordKind,
    name: Identifier,
    type_param_bounds_opt: Option<(Colon, Option<TypeParamBounds>)>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    attributes: Vec<Attribute>,
    associated_items: Vec<Box<dyn AssociatedItem<T>>>,
    close_brace: Brace,
}

use crate::{
    expression::OuterAttr,
    identifier::Identifier,
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Colon, Equals, Semicolon},
};

use super::{AssociatedItem, Item, TypeParamBounds, VisibilityKind, WhereClause};

pub struct TypeAliasItem {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_type: KeywordKind,
    name: Identifier,
    type_param_bounds_opt: Option<(Colon, TypeParamBounds)>,
    where_clause_opt: Option<WhereClause>,
    value_opt: Option<(Equals, Box<dyn Type>, Option<WhereClause>)>,
    semicolon: Semicolon,
}

impl Item for TypeAliasItem {}

impl<A> AssociatedItem<A> for TypeAliasItem where A: Item {}

use crate::{
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Colon, Equals, Semicolon},
};

use super::{TypeParamBounds, VisibilityKind, WhereClause};

pub struct TypeAliasItem {
    attributes: Vec<Attribute>,
    visibility_opt: Option<VisibilityKind>,
    kw_type: KeywordKind,
    name: Identifier,
    type_param_bounds_opt: Option<(Colon, TypeParamBounds)>,
    where_clause_opt: Option<WhereClause>,
    value_opt: Option<(Equals, Box<dyn Type>, Option<WhereClause>)>,
    semicolon: Semicolon,
}

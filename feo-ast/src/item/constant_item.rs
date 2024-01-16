use crate::{
    expression::{Attribute, Expression},
    identifier::Identifier,
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Colon, Equals, Semicolon},
};

use super::VisibilityKind;

pub struct ConstantItem {
    attributes: Vec<Attribute>,
    visibility_opt: Option<VisibilityKind>,
    kw_const: KeywordKind,
    name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Equals, Box<dyn Expression>)>,
    semicolon: Semicolon,
}

pub struct StaticItem {
    visibility_opt: Option<VisibilityKind>,
    kw_static: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Equals, Box<dyn Expression>)>,
    semicolon: Semicolon,
}

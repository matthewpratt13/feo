use crate::{
    expression::{Attribute, Expression},
    identifier::Identifier,
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Colon, Equals, Semicolon},
};

use super::{AssociatedItem, Item, VisibilityKind};

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

impl Item for ConstantItem {}

impl<A> AssociatedItem<A> for ConstantItem where A: Item {}

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

impl Item for StaticItem {}

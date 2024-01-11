use crate::{expression::Expression, identifier::Identifier, keyword::KeywordKind, ty::Type};

use super::{Colon, Semicolon};

pub struct ConstantItem {
    kw_const: KeywordKind,
    name: Identifier,
    colon: Colon,
    item_type: Type,
    assignment_opt: Option<(Colon, Box<Expression>)>,
    semicolon: Semicolon,
}

pub struct StaticItem {
    kw_static: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    name: Identifier,
    colon: Colon,
    item_type: Type,
    assignment_opt: Option<(Colon, Box<Expression>)>,
    semicolon: Semicolon,
}

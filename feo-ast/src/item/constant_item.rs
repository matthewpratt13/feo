use crate::{
    expression::Expression, identifier::Identifier, keyword::KeywordKind, punctuation::PuncKind,
    ty::Type,
};

use super::Colon;

pub struct ConstantItem {
    kw_const: KeywordKind,
    name: Identifier,
    colon: Colon,
    item_type: Type,
    assignment_opt: Option<(Colon, Box<Expression>)>,
    semicolon: PuncKind,
}

pub struct StaticItem {
    kw_static: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    name: Identifier,
    colon: Colon,
    item_type: Type,
    assignment_opt: Option<(Colon, Box<Expression>)>,
    semicolon: PuncKind,
}

use crate::{
    expression::Expression, identifier::Identifier, keyword::KeywordKind, punctuation::PuncKind,
    ty::Type,
};

pub struct ConstantItem {
    kw_const: KeywordKind,
    name: Identifier,
    colon: PuncKind,
    item_type: Type,
    assignment_opt: Option<(PuncKind, Box<Expression>)>,
    semicolon: PuncKind,
}

pub struct StaticItem {
    kw_static: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    name: Identifier,
    colon: PuncKind,
    item_type: Type,
    assignment_opt: Option<(PuncKind, Box<Expression>)>,
    semicolon: PuncKind,
}

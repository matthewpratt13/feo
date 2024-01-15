use crate::{
    expression::ExpressionKind,
    identifier::Identifier,
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Colon, Semicolon},
};

pub struct ConstantItem {
    kw_const: KeywordKind,
    name: Identifier,
    colon: Colon,
    item_type: Box<Type>,
    assignment_opt: Option<(Colon, Box<ExpressionKind>)>,
    semicolon: Semicolon,
}

pub struct StaticItem {
    kw_static: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    name: Identifier,
    colon: Colon,
    item_type: Box<Type>,
    assignment_opt: Option<(Colon, Box<ExpressionKind>)>,
    semicolon: Semicolon,
}

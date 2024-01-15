use crate::{
    expression::Expression,
    identifier::Identifier,
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Colon, Semicolon},
};

pub struct ConstantItem {
    kw_const: KeywordKind,
    name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Colon, Box<dyn Expression>)>,
    semicolon: Semicolon,
}

pub struct StaticItem {
    kw_static: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Colon, Box<dyn Expression>)>,
    semicolon: Semicolon,
}

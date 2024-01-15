use crate::{
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    type_utils::{Brace, Semicolon},
};

use super::Item;

pub struct ModWithoutBody {
    kw_mod: KeywordKind,
    name: Identifier,
    semicolon: Semicolon,
}

pub struct ModWithBody {
    kw_mod: KeywordKind,
    name: Identifier,
    open_brace: Brace,
    attributes: Vec<Attribute>,
    items: Vec<Box<dyn Item>>,
    close_brace: Brace,
}

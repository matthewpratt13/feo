use crate::{
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    type_utils::{Brace, Semicolon},
};

use super::ItemKind;

pub enum ModuleItemKind {
    WithoutBody(ModWithoutBody),
    WithBody(ModWithBody),
}

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
    items: Vec<ItemKind>,
    close_brace: Brace,
}

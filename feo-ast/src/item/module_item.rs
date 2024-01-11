use crate::{
    delimiter::{DelimKind, DelimOrientation},
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
};

use super::{Item, Semicolon};

pub enum ModuleItem {
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
    open_brace: (DelimKind, DelimOrientation),
    attributes: Vec<Attribute>,
    items: Vec<Item>,
    close_brace: (DelimKind, DelimOrientation),
}

use crate::{expression::Attribute, identifier::Identifier, keyword::KeywordKind};

use super::{Brace, Item, Semicolon};

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
    open_brace: Brace,
    attributes: Vec<Attribute>,
    items: Vec<Item>,
    close_brace: Brace,
}

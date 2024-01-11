use crate::{
    delimiter::{DelimKind, DelimOrientation},
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    punctuation::PuncKind,
};

use super::Item;

pub enum ModuleItem {
    WithoutBody(ModWithoutBody),
    WithBody(ModWithBody),
}

pub struct ModWithoutBody {
    kw_mod: KeywordKind,
    name: Identifier,
    semicolon: PuncKind,
}

pub struct ModWithBody {
    kw_mod: KeywordKind,
    name: Identifier,
    open_brace: (DelimKind, DelimOrientation),
    attributes: Vec<Attribute>,
    items: Vec<Item>,
    close_brace: (DelimKind, DelimOrientation),
}

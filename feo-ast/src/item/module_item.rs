use crate::{
    delimiter::{DelimKind, DelimOrientation},
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    punctuation::PuncKind,
};

use super::Item;

pub enum ModuleItem {
    WithoutBody(ModuleWithoutBody),
    WithBody(ModuleWithBody),
}

pub struct ModuleWithoutBody {
    kw_mod: KeywordKind,
    name: Identifier,
    semicolon: PuncKind,
}

pub struct ModuleWithBody {
    kw_mod: KeywordKind,
    name: Identifier,
    open_brace: (DelimKind, DelimOrientation),
    attributes: Vec<Attribute>,
    items: Vec<Item>,
    close_brace: (DelimKind, DelimOrientation),
}

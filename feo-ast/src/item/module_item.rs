use crate::{
    expression::InnerAttr,
    identifier::Identifier,
    keyword::KeywordKind,
    type_utils::{Brace, Semicolon},
};

use super::{Item, ModItem, VisibilityKind};

pub struct ModWithBody {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: KeywordKind,
    kw_mod: KeywordKind,
    name: Identifier,
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    items: Vec<Box<dyn Item>>,
    close_brace: Brace,
}

impl Item for ModWithBody {}

impl<M> ModItem<M> for ModWithBody where M: Item {}

pub struct ModWithoutBody {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: KeywordKind,
    kw_mod: KeywordKind,
    name: Identifier,
    semicolon: Semicolon,
}

impl Item for ModWithoutBody {}

impl<M> ModItem<M> for ModWithoutBody where M: Item {}

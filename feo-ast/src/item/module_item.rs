use crate::{
    expression::Attribute,
    identifier::Identifier,
    keyword::KeywordKind,
    type_utils::{Brace, Semicolon},
};

use super::{Item, VisibilityKind};

pub struct ModWithBody {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: KeywordKind,
    kw_mod: KeywordKind,
    name: Identifier,
    open_brace: Brace,
    attributes: Vec<Attribute>,
    items: Vec<Box<dyn Item>>,
    close_brace: Brace,
}

pub struct ModWithoutBody {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: KeywordKind,
    kw_mod: KeywordKind,
    name: Identifier,
    semicolon: Semicolon,
}

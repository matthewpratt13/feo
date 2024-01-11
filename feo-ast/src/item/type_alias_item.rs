use crate::{identifier::Identifier, keyword::KeywordKind, ty::Type};

use super::{Equals, Semicolon};

pub struct TypeAliasItem {
    kw_type: KeywordKind,
    name: Identifier,
    value_opt: Option<(Equals, Type)>,
    semicolon: Semicolon,
}

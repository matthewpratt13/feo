use crate::{identifier::Identifier, keyword::KeywordKind, punctuation::PuncKind, ty::Type};

pub struct TypeAliasItem {
    kw_type: KeywordKind,
    name: Identifier,
    value_opt: (PuncKind, Type),
    semicolon: PuncKind,
}

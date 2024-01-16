use crate::{identifier::Identifier, keyword::KeywordKind, type_utils::Semicolon};

use super::{import_decl_item::AsClause, VisibilityKind};

pub enum CrateRefKind {
    Identifier(Identifier),
    KwSelf(KeywordKind),
}

pub struct ExternCrateItem {
    visibility_opt: Option<VisibilityKind>,
    kw_extern: KeywordKind,
    kw_crate: KeywordKind,
    crate_ref: CrateRefKind,
    as_clause_opt: Option<AsClause>,
    semicolon: Semicolon,
}

use crate::{identifier::Identifier, keyword::KeywordKind, punctuation::PuncKind};

use super::import_decl_item::AsClause;

pub enum CrateRef {
    Identifier(Identifier),
    KwSelf(KeywordKind),
}

pub struct ExternCrateItem {
    kw_extern: KeywordKind,
    kw_crate: KeywordKind,
    crate_ref: CrateRef,
    as_clause_opt: Option<AsClause>,
    semicolon: PuncKind,
}

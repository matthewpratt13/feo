use crate::{identifier::Identifier, keyword::KeywordKind, punctuation::PuncKind};

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

pub struct AsClause {}

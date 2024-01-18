use feo_types::span::{Span, Spanned};

use crate::{
    identifier::Identifier,
    keyword::Keyword,
    program::{ContractItem, LibraryItem},
    statement::Statement,
    type_utils::Semicolon,
};

use super::{Item, VisibilityKind};

pub enum CrateRefKind {
    Identifier(Identifier),
    KwSelf(Keyword),
}

pub struct ExternCrateDecl {
    visibility_opt: Option<VisibilityKind>,
    kw_extern: Keyword,
    kw_crate: Keyword,
    crate_ref: CrateRefKind,
    as_clause_opt: Option<AsClause>,
    semicolon: Semicolon,
}

impl ContractItem for ExternCrateDecl {}

impl Item for ExternCrateDecl {}

impl LibraryItem for ExternCrateDecl {}

impl Statement for ExternCrateDecl {}

impl Spanned for ExternCrateDecl {
    fn span(&self) -> Span {
        let start_pos = if let Some(v) = &self.visibility_opt {
            v.span().start()
        } else {
            self.kw_extern.span().start()
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_extern.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct AsClause {
    kw_as: Keyword,
    new_name: Identifier,
}

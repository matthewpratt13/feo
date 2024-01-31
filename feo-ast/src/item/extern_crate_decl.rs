use feo_types::{
    span::{Span, Spanned},
    utils::{KwAs, KwCrate, KwExtern, KwSelfType, Semicolon},
    Identifier,
};

use crate::statement::Statement;

use super::Item;

pub enum CrateRefKind {
    Iden(Identifier),
    KwSelfType(KwSelfType),
}

pub struct ExternCrateDecl {
    kw_extern_crate: (KwExtern, KwCrate),
    crate_name: CrateRefKind,
    as_clause_opt: Option<AsClause>,
    semicolon: Semicolon,
}

impl Item for ExternCrateDecl {}

impl Statement for ExternCrateDecl {}

impl Spanned for ExternCrateDecl {
    fn span(&self) -> Span {
        let start_pos = self.kw_extern_crate.0.span().start();
        let end_pos = self.semicolon.span().end();
        let source = self.kw_extern_crate.0.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct AsClause {
    kw_as: KwAs,
    new_name: Identifier,
}

impl Spanned for AsClause {
    fn span(&self) -> Span {
        let start_pos = self.kw_as.span().start();
        let end_pos = self.new_name.span().end();
        let source = self.kw_as.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

use feo_types::{
    span::{Span, Spanned},
    type_utils::{KwAs, KwCrate, KwExtern, KwSelfType, Semicolon},
    Identifier,
};

use crate::attribute::OuterAttr;

use super::VisibilityKind;

#[derive(Debug, Clone)]
pub enum CrateRefKind {
    Iden(Identifier),
    KwSelfType(KwSelfType),
}

#[derive(Debug, Clone)]
pub struct ExternCrateDecl {
    attributes_opt: Option<Vec<OuterAttr>>,
    visibility_opt: Option<VisibilityKind>,
    kw_extern_crate: (KwExtern, KwCrate),
    crate_name: CrateRefKind,
    as_clause_opt: Option<AsClause>,
    semicolon: Semicolon,
}

impl Spanned for ExternCrateDecl {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_extern_crate.0.span(),
                },
            },
            None => self.kw_extern_crate.0.span(),
        };

        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct AsClause {
    kw_as: KwAs,
    new_name: Identifier,
}

impl Spanned for AsClause {
    fn span(&self) -> Span {
        let s1 = self.kw_as.span();
        let s2 = self.new_name.span();

        Span::join(s1, s2)
    }
}

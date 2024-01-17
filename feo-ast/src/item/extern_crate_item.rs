use feo_types::span::{Span, Spanned};

use crate::{
    identifier::Identifier, keyword::Keyword, program::LibraryItem, type_utils::Semicolon,
};

use super::{AsClause, Item, VisibilityKind};

pub enum CrateRefKind {
    Identifier(Identifier),
    KwSelf(Keyword),
}

pub struct ExternCrateItem {
    visibility_opt: Option<VisibilityKind>,
    kw_extern: Keyword,
    kw_crate: Keyword,
    crate_ref: CrateRefKind,
    as_clause_opt: Option<AsClause>,
    semicolon: Semicolon,
}

impl Item for ExternCrateItem {}

impl<L> LibraryItem<L> for ExternCrateItem where L: Item {}

impl Spanned for ExternCrateItem {
    fn span(&self) -> Span {
        let start_pos = if let Some(vk) = &self.visibility_opt {
            vk.span().start()
        } else {
            self.semicolon.span().start()
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_extern.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

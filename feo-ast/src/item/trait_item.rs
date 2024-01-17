use feo_types::span::{Span, Spanned};

use crate::{
    expression::InnerAttr,
    identifier::Identifier,
    keyword::Keyword,
    program::LibraryItem,
    statement::Statement,
    type_utils::{Brace, Colon},
};

use super::{AssociatedItem, Item, TypeParamBounds, VisibilityKind, WhereClause};

pub struct TraitItem {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: Option<Keyword>,
    kw_impl: Keyword,
    name: Identifier,
    type_param_bounds_opt: Option<(Colon, Option<TypeParamBounds>)>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    associated_items: Vec<Box<dyn AssociatedItem>>,
    close_brace: Brace,
}

impl Item for TraitItem {}

impl LibraryItem for TraitItem {}

impl Statement for TraitItem {}

impl Spanned for TraitItem {
    fn span(&self) -> Span {
        let start_pos = match &self.visibility_opt {
            Some(v) => v.span().start(),
            None => match &self.kw_unsafe_opt {
                Some(k) => k.span().start(),
                None => self.kw_impl.span().start(),
            },
        };

        let end_pos = self.name.span().end();
        let source = self.name.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

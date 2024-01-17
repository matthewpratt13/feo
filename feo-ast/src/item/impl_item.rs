use feo_types::span::{Span, Spanned};

use crate::{
    expression::InnerAttr, keyword::Keyword, path::SimplePath, statement::Statement, ty::Type,
    type_utils::Brace,
};

use super::{AssociatedItem, ImplItem, Item, VisibilityKind, WhereClause};

pub struct InherentImpl {
    visibility_opt: Option<VisibilityKind>,
    kw_impl: Keyword,
    object_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    associated_items: Vec<Box<dyn AssociatedItem>>,
    close_brace: Brace,
}

impl Item for InherentImpl {}

impl ImplItem for InherentImpl {}

impl Statement for InherentImpl {}

impl Spanned for InherentImpl {
    fn span(&self) -> Span {
        let start_pos = if let Some(v) = &self.visibility_opt {
            v.span().start()
        } else {
            self.kw_impl.span().start()
        };

        let end_pos = self.close_brace.span().end();
        let source = self.kw_impl.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TraitImpl {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: Option<Keyword>,
    kw_impl: Keyword,
    trait_path: SimplePath,
    kw_for: Keyword,
    object_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    associated_items: Vec<Box<dyn AssociatedItem>>,
    close_brace: Brace,
}

impl Item for TraitImpl {}

impl ImplItem for TraitImpl {}

impl Statement for TraitImpl {}

impl Spanned for TraitImpl {
    fn span(&self) -> Span {
        let start_pos = match &self.visibility_opt {
            Some(v) => v.span().start(),
            None => match &self.kw_unsafe_opt {
                Some(k) => k.span().start(),
                None => self.kw_impl.span().start(),
            },
        };

        let end_pos = self.close_brace.span().end();
        let source = self.trait_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

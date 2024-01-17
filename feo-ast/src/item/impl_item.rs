use feo_types::span::{Span, Spanned};

use crate::{
    expression::InnerAttr, keyword::Keyword, path::SimplePath, ty::Type, type_utils::Brace,
};

use super::{AssociatedItem, ImplItem, Item, VisibilityKind, WhereClause};

pub struct InherentImpl<T> {
    visibility_opt: Option<VisibilityKind>,
    kw_impl: Keyword,
    object_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    associated_items: Vec<Box<dyn AssociatedItem<T>>>,
    close_brace: Brace,
}

impl<T> Item for InherentImpl<T> {}

impl<T, I> ImplItem<I> for InherentImpl<T> where I: Item {}

impl<T> Spanned for InherentImpl<T> {
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

pub struct TraitImpl<T> {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: Option<Keyword>,
    kw_impl: Keyword,
    trait_path: SimplePath,
    kw_for: Keyword,
    object_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    associated_items: Vec<Box<dyn AssociatedItem<T>>>,
    close_brace: Brace,
}

impl<T> Item for TraitImpl<T> {}

impl<T, I> ImplItem<I> for TraitImpl<T> where I: Item {}

impl<T> Spanned for TraitImpl<T> {
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

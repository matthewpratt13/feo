use crate::{
    expression::{InnerAttr, OuterAttr},
    keyword::Keyword,
    path::SimplePath,
    span::{Span, Spanned},
    statement::Statement,
    ty::Type,
    type_utils::Brace,
};

use super::{AssociatedItem, Item, WhereClause};

pub trait ImplItem
where
    Self: Sized + Item,
{
}

pub struct InherentImpl {
    outer_attributes: Vec<OuterAttr>,
    kw_impl: Keyword,
    nominal_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<Box<dyn AssociatedItem>>, // excludes type alias
    close_brace: Brace,
}

impl ImplItem for InherentImpl {}

impl Item for InherentImpl {}

impl Statement for InherentImpl {}

impl Spanned for InherentImpl {
    fn span(&self) -> Span {
        let start_pos = if let Some(a) = self.outer_attributes.first() {
            a.span().start()
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
    outer_attributes: Vec<OuterAttr>,
    kw_impl: Keyword,
    implemented_trait_path: SimplePath,
    kw_for: Keyword,
    implementing_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<Box<dyn AssociatedItem>>, // cannot contain `TypeAliasDef`
    close_brace: Brace,
}

impl ImplItem for TraitImpl {}

impl Item for TraitImpl {}

impl Statement for TraitImpl {}

impl Spanned for TraitImpl {
    fn span(&self) -> Span {
        let start_pos = if let Some(a) = self.outer_attributes.first() {
            a.span().start()
        } else {
            self.kw_impl.span().start()
        };

        let end_pos = self.close_brace.span().end();
        let source = self.kw_impl.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

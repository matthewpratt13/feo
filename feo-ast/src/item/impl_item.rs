use feo_types::span::{Span, Spanned};

use crate::{
    expression::{InnerAttr, OuterAttr},
    keyword::Keyword,
    path::SimplePath,
    program::ContractItem,
    statement::Statement,
    ty::Type,
    type_utils::Brace,
};

use super::{AssociatedItem, ImplItem, Item, WhereClause};

pub struct InherentImpl {
    outer_attributes: Vec<OuterAttr>,
    kw_impl: Keyword,
    nominal_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<Box<dyn AssociatedItem>>,
    close_brace: Brace,
}

impl ContractItem for InherentImpl {}

impl Item for InherentImpl {}

impl ImplItem for InherentImpl {}

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
    trait_path: SimplePath,
    kw_for: Keyword,
    nominal_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<Box<dyn AssociatedItem>>,
    close_brace: Brace,
}

impl ContractItem for TraitImpl {}

impl Item for TraitImpl {}

impl ImplItem for TraitImpl {}

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

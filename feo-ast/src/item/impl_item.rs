use crate::{
    expression::{InnerAttr, OuterAttr},
    keyword::Keyword,
    path::PathType,
    span::{Span, Spanned},
    statement::Statement,
    ty::Type,
    type_utils::Brace,
};

use super::{ConstantItem, FunctionDef, Item, TypeAliasDef, WhereClause};

pub trait ImplItem
where
    Self: Item + Sized,
{
}

pub enum InherentImplItem<T> {
    Constant(ConstantItem),
    FuncDef(FunctionDef<T>),
}

pub enum TraitImplItem<T> {
    Constant(ConstantItem),
    FuncDef(FunctionDef<T>),
    TypeAlias(TypeAliasDef),
}

pub struct InherentImpl<T> {
    outer_attributes: Vec<OuterAttr>,
    kw_impl: Keyword,
    nominal_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<InherentImplItem<T>>,
    close_brace: Brace,
}

impl<T> ImplItem for InherentImpl<T> {}

impl<T> Item for InherentImpl<T> {}

impl<T> Statement for InherentImpl<T> {}

impl<T> Spanned for InherentImpl<T> {
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

pub struct TraitImpl<T> {
    outer_attributes: Vec<OuterAttr>,
    kw_impl: Keyword,
    implemented_trait_path: PathType,
    kw_for: Keyword,
    implementing_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<TraitImplItem<T>>,
    close_brace: Brace,
}

impl<T> ImplItem for TraitImpl<T> {}

impl<T> Item for TraitImpl<T> {}

impl<T> Statement for TraitImpl<T> {}

impl<T> Spanned for TraitImpl<T> {
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

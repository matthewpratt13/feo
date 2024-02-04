use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, KwFor, KwImpl},
};

use crate::{
    expression::{InnerAttr, OuterAttr},
    path::PathType,
    ty::Type,
};

use super::{ConstantItem, FunctionDef, Item, TypeAliasDef, WhereClause};

pub enum InherentImplItem<T, U> {
    Constant(ConstantItem),
    FuncDef(FunctionDef<T, U>),
}

pub enum TraitImplItem<T, U> {
    Constant(ConstantItem),
    FuncDef(FunctionDef<T, U>),
    TypeAlias(TypeAliasDef),
}

pub struct InherentImpl<T, U> {
    outer_attributes: Vec<OuterAttr>,
    kw_impl: KwImpl,
    nominal_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<InherentImplItem<T, U>>,
    close_brace: Brace,
}

impl<T, U> Item for InherentImpl<T, U> {}

impl<T, U> Spanned for InherentImpl<T, U> {
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

pub struct TraitImpl<T, U> {
    outer_attributes: Vec<OuterAttr>,
    kw_impl: KwImpl,
    implemented_trait_path: PathType,
    kw_for: KwFor,
    implementing_type: Box<dyn Type>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<TraitImplItem<T, U>>,
    close_brace: Brace,
}

impl<T, U> Item for TraitImpl<T, U> {}

impl<T, U> Spanned for TraitImpl<T, U> {
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

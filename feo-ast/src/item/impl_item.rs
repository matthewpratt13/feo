use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, KwFor, KwImpl},
};

use crate::{
    expression::{InnerAttr, OuterAttr},
    path::PathType,
    ty::Type,
};

use super::{ConstantItem, FunctionDef, TypeAliasDef, WhereClause};

#[derive(Clone)]
pub enum InherentImplItem {
    Constant(ConstantItem),
    FuncDef(FunctionDef),
}

#[derive(Clone)]
pub enum TraitImplItem {
    Constant(ConstantItem),
    FuncDef(FunctionDef),
    TypeAlias(TypeAliasDef),
}

#[derive(Clone)]
pub struct InherentImpl {
    outer_attributes: Vec<OuterAttr>,
    kw_impl: KwImpl,
    nominal_type: Type,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<InherentImplItem>,
    close_brace: Brace,
}

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

#[derive(Clone)]
pub struct TraitImpl {
    outer_attributes: Vec<OuterAttr>,
    kw_impl: KwImpl,
    implemented_trait_path: PathType,
    kw_for: KwFor,
    implementing_type: Type,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<TraitImplItem>,
    close_brace: Brace,
}

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

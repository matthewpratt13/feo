use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, KwFor, KwImpl},
};

use crate::{
    attribute::{InnerAttr, OuterAttr},
    path::PathType,
    ty::Type,
};

use super::{ConstantVarDef, FunctionWithBlock, TypeAliasDef, WhereClause};

#[derive(Debug, Clone)]
pub enum InherentImplItem {
    ConstantVarDef(ConstantVarDef),
    FuncWithBlock(FunctionWithBlock),
}

#[derive(Debug, Clone)]
pub enum TraitImplItem {
    ConstantVarDef(ConstantVarDef),
    FuncWithBlock(FunctionWithBlock),
    TypeAliasDef(TypeAliasDef),
}

#[derive(Debug, Clone)]
pub struct InherentImplBlock {
    outer_attributes: Vec<OuterAttr>,
    kw_impl: KwImpl,
    nominal_type: Type,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<InherentImplItem>,
    close_brace: Brace,
}

impl Spanned for InherentImplBlock {
    fn span(&self) -> Span {
        let s1 = if let Some(a) = self.outer_attributes.first() {
            a.span()
        } else {
            self.kw_impl.span()
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct TraitImplBlock {
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

impl Spanned for TraitImplBlock {
    fn span(&self) -> Span {
        let s1 = if let Some(a) = self.outer_attributes.first() {
            a.span()
        } else {
            self.kw_impl.span()
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

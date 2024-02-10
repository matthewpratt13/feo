use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, KwFor, KwImpl},
};

use crate::{
    expression::{InnerAttr, OuterAttr},
    path::PathType,
    ty::Type,
};

use super::{ConstantVarDef, FunctionWithBlock, TypeAliasDef, WhereClause};

#[derive(Clone)]
pub enum InherentImplItem {
    ConstantVarDef(ConstantVarDef),
    FuncWithBlock(FunctionWithBlock),
}

#[derive(Clone)]
pub enum TraitImplItem {
    ConstantVarDef(ConstantVarDef),
    FuncWithBlock(FunctionWithBlock),
    TypeAliasDef(TypeAliasDef),
}

#[derive(Clone)]
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

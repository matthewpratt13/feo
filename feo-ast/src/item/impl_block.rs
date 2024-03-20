use feo_types::{
    span::{Span, Spanned},
    type_utils::{Brace, KwFor, KwImpl},
};

use crate::{
    attribute::{InnerAttr, OuterAttr},
    path::PathType,
    ty::Type,
};

use super::{ConstantVarDef, FunctionWithBlock, TypeAliasDef};

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
    pub outer_attributes_opt: Option<Vec<OuterAttr>>,
    pub kw_impl: KwImpl,
    pub nominal_type: Type,
    pub open_brace: Brace,
    pub inner_attributes_opt: Option<Vec<InnerAttr>>,
    pub associated_items_opt: Option<Vec<InherentImplItem>>,
    pub close_brace: Brace,
}

impl Spanned for InherentImplBlock {
    fn span(&self) -> Span {
        let s1 = match &self.outer_attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => self.kw_impl.span(),
            },
            None => self.kw_impl.span(),
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct TraitImplBlock {
    pub outer_attributes_opt: Option<Vec<OuterAttr>>,
    pub kw_impl: KwImpl,
    pub implemented_trait_path: PathType,
    pub kw_for: KwFor,
    pub implementing_type: Type,
    pub open_brace: Brace,
    pub inner_attributes_opt: Option<Vec<InnerAttr>>,
    pub associated_items_opt: Option<Vec<TraitImplItem>>,
    pub close_brace: Brace,
}

impl Spanned for TraitImplBlock {
    fn span(&self) -> Span {
        let s1 = match &self.outer_attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => self.kw_impl.span(),
            },
            None => self.kw_impl.span(),
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

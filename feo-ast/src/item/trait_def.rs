use feo_types::{
    span::{Span, Spanned},
    type_utils::{Brace, KwTrait},
    Identifier,
};

use crate::{
    attribute::{InnerAttr, OuterAttr},
    expression::TermCollection,
    ty::TraitBound,
};

use super::{ConstantVarDef, FunctionSig, FunctionWithBlock, TypeAliasDef, VisibilityKind};

#[derive(Debug, Clone)]
pub enum TraitDefItem {
    Constant(ConstantVarDef),
    FuncDef(FunctionWithBlock),
    FuncSig(FunctionSig),
    TypeAlias(TypeAliasDef),
}

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub outer_attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_trait: KwTrait,
    pub trait_name: Identifier,
    pub type_param_bounds_opt: Option<TermCollection<TraitBound>>,
    pub open_brace: Brace,
    pub inner_attributes_opt: Option<Vec<InnerAttr>>,
    pub associated_items_opt: Option<Vec<TraitDefItem>>,
    pub close_brace: Brace,
}

impl Spanned for TraitDef {
    fn span(&self) -> Span {
        let s1 = match &self.outer_attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_trait.span(),
                },
            },
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_trait.span(),
            },
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, KwTrait},
    Identifier,
};

use crate::attribute::{InnerAttr, OuterAttr};

use super::{
    ConstantVarDef, FunctionSig, FunctionWithBlock, TypeAliasDef, TypeParamBounds, VisibilityKind,
    WhereClause,
};

#[derive(Debug, Clone)]
pub enum TraitDefItem {
    Constant(ConstantVarDef),
    FuncDef(FunctionWithBlock),
    FuncSig(FunctionSig),
    TypeAlias(TypeAliasDef),
}

#[derive(Debug, Clone)]
pub struct TraitDef {
    outer_attributes: Option<Vec<OuterAttr>>,
    visibility_opt: Option<VisibilityKind>,
    kw_trait: KwTrait,
    trait_name: Identifier,
    type_param_bounds_opt: Option<Option<TypeParamBounds>>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<TraitDefItem>,
    close_brace: Brace,
}

impl Spanned for TraitDef {
    fn span(&self) -> Span {
        let s1 = match &self.outer_attributes {
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

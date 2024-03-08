use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Equals, KwConst, KwMut, KwStatic, Semicolon},
    Identifier,
};

use crate::{attribute::OuterAttr, expression::Expression, ty::Type};

use super::VisibilityKind;

#[derive(Debug, Clone)]
pub struct ConstantVarDef {
    attributes: Option<Vec<OuterAttr>>,
    visibility_opt: Option<VisibilityKind>,
    kw_const: KwConst,
    item_name: Identifier,
    colon: Colon,
    item_type: Box<Type>,
    assignment_opt: Option<(Equals, Box<Expression>)>, // `None` is only allowed in a `TraitDef`
    semicolon: Semicolon,
}

impl Spanned for ConstantVarDef {
    fn span(&self) -> Span {
        let s1 = match &self.attributes {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_const.span(),
                },
            },
            None => self.kw_const.span(),
        };

        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct StaticVarDef {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_static: KwStatic,
    kw_mut_opt: Option<KwMut>,
    item_name: Identifier,
    colon: Colon,
    item_type: Type,
    assignment_opt: Option<(Equals, Expression)>,
    semicolon: Semicolon,
}

impl Spanned for StaticVarDef {
    fn span(&self) -> Span {
        let s1 = match self.attributes.first() {
            Some(a) => a.span(),
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_static.span(),
            },
        };

        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}

unsafe impl Sync for StaticVarDef {}

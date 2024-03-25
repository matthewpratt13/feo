use feo_types::{
    span::{Span, Spanned},
    type_utils::{KwConst, KwMut, KwStatic, Semicolon},
    Identifier,
};

use crate::{attribute::OuterAttr, expression::Expression, ty::Type};

use super::VisibilityKind;

#[derive(Debug, Clone)]
pub struct ConstVarDef {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_const: KwConst,
    pub item_name: Identifier,
    pub item_type: Box<Type>,
    pub assignment_opt: Option<Box<Expression>>, // `None` is only allowed in a `TraitDef`
    pub semicolon: Semicolon,
}

impl Spanned for ConstVarDef {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
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
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_static: KwStatic,
    pub kw_mut_opt: Option<KwMut>,
    pub item_name: Identifier,
    pub item_type: Type,
    pub assignment_opt: Option<Box<Expression>>,
    pub semicolon: Semicolon,
}

impl Spanned for StaticVarDef {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_static.span(),
                },
            },
            None => self.kw_static.span(),
        };

        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}

unsafe impl Sync for StaticVarDef {}

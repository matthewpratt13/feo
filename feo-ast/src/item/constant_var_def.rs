use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Equals, KwConst, KwMut, KwStatic, Semicolon},
    Identifier,
};

use crate::{attribute::OuterAttr, expression::Expression, ty::Type};

use super::VisibilityKind;

#[derive(Clone)]
pub struct ConstantVarDef {
    attributes: Vec<OuterAttr>,
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
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_const.span().start(),
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_const.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
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
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_static.span().start(),
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_static.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

unsafe impl Sync for StaticVarDef {}

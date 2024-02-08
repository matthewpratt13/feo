use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Equals, KwConst, KwMut, KwStatic, Semicolon},
    Identifier,
};

use crate::{
    expression::{Expression, OuterAttr},
    pattern::Pattern,
    ty::Type,
};

use super::{Item, VisibilityKind};

#[derive(Clone)]
pub struct ConstantItem {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_const: KwConst,
    item_name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Equals, Expression)>, // `None` is only allowed in a `TraitDef`
    semicolon: Semicolon,
}

impl Item for ConstantItem {}

impl Pattern for ConstantItem {}

impl Spanned for ConstantItem {
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
pub struct StaticItem {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_static: KwStatic,
    kw_mut_opt: Option<KwMut>,
    item_name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Equals, Expression)>,
    semicolon: Semicolon,
}

impl Item for StaticItem {}

impl Spanned for StaticItem {
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

unsafe impl Sync for StaticItem {}

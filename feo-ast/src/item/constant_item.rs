use crate::{
    expression::{Constant, Expression, OuterAttr},
    identifier::Identifier,
    keyword::Keyword,
    pattern::Pattern,
    span::{Span, Spanned},
    statement::Statement,
    ty::Type,
    type_utils::{Colon, Equals, Semicolon},
};

use super::{Item, VisibilityKind};

pub struct ConstantItem {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_const: Keyword,
    item_name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Equals, Box<dyn Expression>)>, // `None` is only allowed in a `TraitDef`
    semicolon: Semicolon,
}

impl Item for ConstantItem {}

impl Statement for ConstantItem {}

impl Pattern for ConstantItem {}

impl Constant for ConstantItem {}

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

pub struct StaticItem {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_static: Keyword,
    kw_mut_opt: Option<Keyword>,
    item_name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Equals, Box<dyn Expression>)>,
    semicolon: Semicolon,
}

impl Item for StaticItem {}

impl Statement for StaticItem {}

impl Constant for StaticItem {}

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

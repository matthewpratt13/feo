use feo_types::span::{Span, Spanned};

use crate::{
    expression::{Expression, OuterAttr},
    identifier::Identifier,
    keyword::Keyword,
    ty::Type,
    type_utils::{Colon, Equals, Semicolon},
};

use super::{AssociatedItem, Item, VisibilityKind};

pub struct ConstantItem {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_const: Keyword,
    name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Equals, Box<dyn Expression>)>,
    semicolon: Semicolon,
}

impl Item for ConstantItem {}

impl<A> AssociatedItem<A> for ConstantItem where A: Item {}

impl Spanned for ConstantItem {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.last() {
            Some(a) => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => a.span().start(),
            },
            None => self.kw_const.span().start(),
        };

        let end_pos = self.semicolon.span().end();
        let source = self.name.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct StaticItem {
    visibility_opt: Option<VisibilityKind>,
    kw_static: Keyword,
    kw_mut_opt: Option<Keyword>,
    name: Identifier,
    colon: Colon,
    item_type: Box<dyn Type>,
    assignment_opt: Option<(Equals, Box<dyn Expression>)>,
    semicolon: Semicolon,
}

impl Item for StaticItem {}

impl Spanned for StaticItem {
    fn span(&self) -> Span {
        let start_pos = if let Some(v) = &self.visibility_opt {
            v.span().start()
        } else {
            self.kw_static.span().start()
        };

        let end_pos = self.semicolon.span().end();
        let source = self.name.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

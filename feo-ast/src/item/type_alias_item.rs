use feo_types::span::{Span, Spanned};

use crate::{
    expression::OuterAttr,
    identifier::Identifier,
    keyword::Keyword,
    ty::Type,
    type_utils::{Colon, Equals, Semicolon},
};

use super::{AssociatedItem, Item, TypeParamBounds, VisibilityKind, WhereClause};

pub struct TypeAliasItem {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_type: Keyword,
    name: Identifier,
    type_param_bounds_opt: Option<(Colon, TypeParamBounds)>,
    where_clause_opt: Option<WhereClause>,
    value_opt: Option<(Equals, Box<dyn Type>, Option<WhereClause>)>,
    semicolon: Semicolon,
}

impl Item for TypeAliasItem {}

impl<A> AssociatedItem<A> for TypeAliasItem where A: Item {}

impl Spanned for TypeAliasItem {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_type.span().start(),
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.name.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

use feo_types::span::{Span, Spanned};

use crate::{
    expression::{InnerAttr, OuterAttr},
    identifier::Identifier,
    keyword::Keyword,
    program::{ContractItem, LibraryItem},
    statement::Statement,
    type_utils::{Brace, Colon},
};

use super::{AssociatedItem, Item, TypeParamBounds, VisibilityKind, WhereClause};

pub struct TraitDef {
    outer_attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_trait: Keyword,
    identifier: Identifier,
    type_param_bounds_opt: Option<(Colon, Option<TypeParamBounds>)>,
    where_clause_opt: Option<WhereClause>,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    associated_items: Vec<Box<dyn AssociatedItem>>,
    close_brace: Brace,
}

impl ContractItem for TraitDef {}

impl Item for TraitDef {}

impl LibraryItem for TraitDef {}

impl Statement for TraitDef {}

impl Spanned for TraitDef {
    fn span(&self) -> Span {
        let start_pos = match self.outer_attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_trait.span().start(),
            },
        };

        let end_pos = self.close_brace.span().end();
        let source = self.kw_trait.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

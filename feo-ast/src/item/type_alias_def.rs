use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Equals, Semicolon},
    Identifier, Keyword,
};

use crate::{expression::OuterAttr, statement::Statement, ty::Type};

use super::{Item, TypeParamBounds, VisibilityKind};

pub struct TypeAliasDef {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_type: Keyword,
    type_name: Identifier,
    type_param_bounds_opt: Option<(Colon, TypeParamBounds)>,
    assignment_opt: Option<(Equals, Box<dyn Type>)>,
    semicolon: Semicolon,
}

impl Item for TypeAliasDef {}

impl Statement for TypeAliasDef {}

impl Spanned for TypeAliasDef {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_type.span().start(),
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_type.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

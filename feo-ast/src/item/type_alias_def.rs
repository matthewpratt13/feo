use feo_types::{
    span::{Span, Spanned},
    utils::{KwType, Semicolon},
    Identifier,
};

use crate::{attribute::OuterAttr, ty::Type};

use super::{TypeParamBounds, VisibilityKind};

#[derive(Debug, Clone)]
pub struct TypeAliasDef {
    attributes_opt: Option<Vec<OuterAttr>>,
    visibility_opt: Option<VisibilityKind>,
    kw_type: KwType,
    type_name: Identifier,
    type_param_bounds_opt: Option<TypeParamBounds>,
    assignment_opt: Option<Type>,
    semicolon: Semicolon,
}

impl Spanned for TypeAliasDef {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_type.span(),
                },
            },
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_type.span(),
            },
        };

        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}

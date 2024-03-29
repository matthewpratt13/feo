use feo_types::{
    span::{Span, Spanned},
    type_utils::{KwType, Semicolon},
    Identifier,
};

use crate::{attribute::OuterAttr, ty::Type};

use super::VisibilityKind;

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_type: KwType,
    pub type_name: Identifier,
    pub type_opt: Option<Type>,
    pub semicolon: Semicolon,
}

impl Spanned for TypeDef {
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

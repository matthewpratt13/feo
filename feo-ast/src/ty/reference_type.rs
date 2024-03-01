use feo_types::{
    span::{Span, Spanned},
    utils::{Ampersand, KwMut},
};

use super::Type;

#[derive(Debug, Clone)]
pub struct ReferenceType {
    ampersand: Ampersand,
    kw_mut: KwMut,
    ty: Box<Type>,
}

impl Spanned for ReferenceType {
    fn span(&self) -> Span {
        let s1 = self.ampersand.span();
        let s2 = self.ty.span();

        Span::join(s1, s2)
    }
}

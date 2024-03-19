use feo_types::{
    span::{Span, Spanned},
    type_utils::{Ampersand, KwMut},
};

use super::Type;

#[derive(Debug, Clone)]
pub struct ReferenceType(pub Ampersand, pub Option<KwMut>, pub Box<Type>);

impl Spanned for ReferenceType {
    fn span(&self) -> Span {
        let s1 = self.0.span();
        let s2 = self.2.span();

        Span::join(s1, s2)
    }
}

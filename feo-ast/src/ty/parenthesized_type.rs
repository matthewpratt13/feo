use feo_types::{
    span::{Span, Spanned},
    utils::Parenthesis,
};

use super::Type;

#[derive(Debug, Clone)]
pub struct ParenthesizedType(pub Parenthesis, pub Box<Type>, pub Parenthesis);

impl Spanned for ParenthesizedType {
    fn span(&self) -> Span {
        let s1 = self.0.span();
        let s2 = self.2.span();

        Span::join(s1, s2)
    }
}

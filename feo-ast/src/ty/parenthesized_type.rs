use feo_types::{
    span::{Span, Spanned},
    utils::Parenthesis,
};

use super::Type;

#[derive(Debug, Clone)]
pub struct ParenthesizedType {
    open_parenthesis: Parenthesis,
    ty: Box<Type>,
    close_parenthesis: Parenthesis,
}

impl Spanned for ParenthesizedType {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

use feo_types::{
    span::{Span, Spanned},
    utils::{Comma, Parenthesis},
};

use super::Type;

#[derive(Debug, Clone)]
pub struct TupleType {
    open_parenthesis: Parenthesis,
    elements: Vec<(Type, Comma)>,
    trailing_element: Box<Type>,
    close_parenthesis: Parenthesis,
}

impl Spanned for TupleType {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

pub struct UnitType {
    open_parenthesis: Parenthesis,
    close_parenthesis: Parenthesis,
}

impl Spanned for UnitType {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

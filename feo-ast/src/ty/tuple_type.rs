use feo_types::{
    span::{Span, Spanned},
    utils::{Comma, Parenthesis},
};

use super::Type;

#[derive(Debug, Clone)]
pub struct TupleType {
    pub open_parenthesis: Parenthesis,
    pub elements: Option<Vec<(Type, Comma)>>,
    pub trailing_element: Box<Type>,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for TupleType {
    fn span(&self) -> Span {
        let s1 = self.open_parenthesis.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct UnitType(pub Parenthesis, pub Parenthesis);

impl Spanned for UnitType {
    fn span(&self) -> Span {
        let s1 = self.0.span();
        let s2 = self.1.span();

        Span::join(s1, s2)
    }
}

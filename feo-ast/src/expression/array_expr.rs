use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    utils::{Bracket, Comma, Semicolon},
};

use super::IterableExpr;

#[derive(Clone)]
pub enum ArrayElements {
    CommaSeparated(ArrayElementsCommaSeparated),
    RepeatedValue(ArrayElementsRepeatedValue),
}

#[derive(Clone)]
pub struct ArrayExpr {
    open_bracket: Bracket,
    elements_opt: Option<ArrayElements>,
    close_bracket: Bracket,
}

impl Spanned for ArrayExpr {
    fn span(&self) -> Span {
        let s1 = self.open_bracket.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct ArrayElementsCommaSeparated {
    first_element: Box<IterableExpr>,
    subsequent_elements: Vec<(Comma, IterableExpr)>,
    trailing_comma_opt: Option<Comma>,
}

#[derive(Clone)]
pub struct ArrayElementsRepeatedValue {
    repeat_operand: Box<IterableExpr>,
    semicolon: Semicolon,
    num_repeats: Primitive<u64>,
}

#[derive(Clone)]
pub struct IndexExpr {
    indexed_operand: ArrayExpr,
    open_bracket: Bracket,
    index: Primitive<u64>,
    close_bracket: Bracket,
}

impl Spanned for IndexExpr {
    fn span(&self) -> Span {
        let s1 = self.indexed_operand.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}

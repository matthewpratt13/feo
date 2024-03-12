use feo_types::{
    span::{Span, Spanned},
    utils::{Bracket, Semicolon},
    U64Primitive,
};

use super::{Assignable, Iterable};

#[derive(Debug, Clone)]
pub enum ArrayElementsKind {
    CommaSeparated(ArrayElementsCommaSeparated),
    RepeatedValue(ArrayElementsRepeatedValue),
}

#[derive(Debug, Clone)]
pub struct ArrayExpr {
    pub open_bracket: Bracket,
    pub elements_opt: Option<ArrayElementsKind>,
    pub close_bracket: Bracket,
}

impl Spanned for ArrayExpr {
    fn span(&self) -> Span {
        let s1 = self.open_bracket.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct ArrayElementsCommaSeparated {
    pub first_element: Box<Iterable>,
    pub subsequent_elements_opt: Option<Vec<Iterable>>,
}

#[derive(Debug, Clone)]
pub struct ArrayElementsRepeatedValue {
    pub repeat_operand: Box<Iterable>,
    pub semicolon: Semicolon,
    pub num_repeats: U64Primitive,
}

#[derive(Debug, Clone)]
pub struct IndexExpr {
    pub indexed_operand: Assignable,
    pub open_bracket: Bracket,
    pub index: U64Primitive,
    pub close_bracket: Bracket,
}

impl Spanned for IndexExpr {
    fn span(&self) -> Span {
        let s1 = self.indexed_operand.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}

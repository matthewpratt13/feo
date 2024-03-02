use feo_types::{
    literal::UIntType,
    span::{Span, Spanned},
    utils::{Bracket, Comma, Semicolon},
    Literal,
};

use super::Iterable;

#[derive(Debug, Clone)]
enum ArrayElementsKind {
    CommaSeparated(ArrayElementsCommaSeparated),
    RepeatedValue(ArrayElementsRepeatedValue),
}

#[derive(Debug, Clone)]
pub struct ArrayExpr {
    open_bracket: Bracket,
    elements_opt: Option<ArrayElementsKind>,
    close_bracket: Bracket,
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
    first_element: Box<Iterable>,
    subsequent_elements: Vec<(Comma, Iterable)>,
    trailing_comma_opt: Option<Comma>,
}

#[derive(Debug, Clone)]
pub struct ArrayElementsRepeatedValue {
    repeat_operand: Box<Iterable>,
    semicolon: Semicolon,
    num_repeats: Literal<UIntType>,
}

#[derive(Debug, Clone)]
pub struct IndexExpr {
    indexed_operand: ArrayExpr,
    open_bracket: Bracket,
    index: Literal<UIntType>,
    close_bracket: Bracket,
}

impl Spanned for IndexExpr {
    fn span(&self) -> Span {
        let s1 = self.indexed_operand.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}

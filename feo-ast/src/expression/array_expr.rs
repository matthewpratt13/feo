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

        // let start_pos = self.open_bracket.span().start();
        // let end_pos = self.close_bracket.span().end();
        // let source = self.open_bracket.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
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
    num_repeats: Primitive,
}

#[derive(Clone)]
pub struct IndexExpr {
    indexed_operand: ArrayExpr,
    open_bracket: Bracket,
    index: Primitive,
    close_bracket: Bracket,
}

impl Spanned for IndexExpr {
    fn span(&self) -> Span {
        let s1 = self.indexed_operand.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)

        // let start_pos = self.indexed_operand.span().start();
        // let end_pos = self.close_bracket.span().end();
        // let source = self.indexed_operand.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

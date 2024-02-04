use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    utils::{Bracket, Comma, Semicolon},
};

use super::IterableExpr;

pub struct ArrayExpr<T, U> {
    open_bracket: Bracket,
    elements_opt: Option<ArrayElements<T, U>>,
    close_bracket: Bracket,
}

impl<T, U> Spanned for ArrayExpr<T, U> {
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

pub struct ArrayElements<T, U> {
    first_element: IterableExpr<T, U>,
    subsequent_elements: Vec<(Comma, IterableExpr<T, U>)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct ArrayWithSingleRepeatedValue<T, U> {
    repeat_operand: IterableExpr<T, U>,
    semicolon: Semicolon,
    num_repeats: Primitive<u64>,
}

pub struct IndexExpr<T, U> {
    indexed_operand: ArrayExpr<T, U>,
    open_bracket: Bracket,
    index: Primitive<u64>,
    close_bracket: Bracket,
}

impl<T, U> Spanned for IndexExpr<T, U> {
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

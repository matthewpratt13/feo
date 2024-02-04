use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    utils::{Bracket, Comma, Semicolon},
};

use super::{Assignable, BooleanOperand, Constant, ExprWithoutBlock, IterableExpr};

pub struct ArrayExpr<I: IterableExpr> {
    open_bracket: Bracket,
    elements_opt: Option<ArrayElements<I>>,
    close_bracket: Bracket,
}

impl<I> ExprWithoutBlock for ArrayExpr<I> where I: IterableExpr {}

impl<I> BooleanOperand for ArrayExpr<I> where I: IterableExpr {}

impl<I> IterableExpr for ArrayExpr<I> where I: IterableExpr {}

impl<I> Assignable for ArrayExpr<I> where I: IterableExpr {}

impl<I> Constant for ArrayExpr<I> where I: IterableExpr {}

impl<I> Spanned for ArrayExpr<I>
where
    I: IterableExpr,
{
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

pub struct ArrayElements<I: IterableExpr> {
    first_element: I,
    subsequent_elements: Vec<(Comma, I)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct ArrayWithSingleRepeatedValue<I: IterableExpr> {
    repeat_operand: I,
    semicolon: Semicolon,
    num_repeats: Primitive<u64>,
}

pub struct IndexExpr<I: IterableExpr> {
    indexed_operand: ArrayExpr<I>,
    open_bracket: Bracket,
    index: Primitive<u64>,
    close_bracket: Bracket,
}

impl<I> ExprWithoutBlock for IndexExpr<I> where I: IterableExpr {}

impl<I> BooleanOperand for IndexExpr<I> where I: IterableExpr {}

impl<I> IterableExpr for IndexExpr<I> where I: IterableExpr {}

impl<I> Constant for IndexExpr<I> where I: IterableExpr {}

impl<I> Spanned for IndexExpr<I>
where
    I: IterableExpr,
{
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

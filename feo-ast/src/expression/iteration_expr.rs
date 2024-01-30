use feo_types::{
    span::{Span, Spanned},
    Keyword,
};

use crate::pattern::Pattern;

use super::{BlockExpr, BooleanOperand, Constant, ExprWithBlock, Expression, IterableExpr};

pub trait IterationExpr<E>
where
    Self: Sized + ExprWithBlock<E> + IterableExpr + BooleanOperand,
{
}

#[allow(dead_code)]
pub type BreakExpr = Keyword;

#[allow(dead_code)]
pub type ContinueExpr = Keyword;

pub struct InfiniteLoopExpr<T> {
    kw_loop: Keyword,
    block: BlockExpr<T>,
}

impl<T, E> IterationExpr<E> for InfiniteLoopExpr<T> where T: 'static {}

impl<T> Expression for InfiniteLoopExpr<T> {}

impl<T, E> ExprWithBlock<E> for InfiniteLoopExpr<T> {}

impl<T> BooleanOperand for InfiniteLoopExpr<T> where T: 'static {}

impl<T> Constant for InfiniteLoopExpr<T> where T: 'static {}

impl<T> IterableExpr for InfiniteLoopExpr<T> where T: 'static {}

impl<T> Spanned for InfiniteLoopExpr<T> {
    fn span(&self) -> Span {
        let s1 = self.kw_loop.span();
        let s2 = self.block.span();

        Span::join(s1, s2)

        // let start_pos = self.kw_loop.span().start();
        // let end_pos = self.block.span().end();
        // let source = self.kw_loop.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct PredicateLoopExpr<T> {
    kw_while: Keyword,
    conditional_operand: Box<dyn BooleanOperand>,
    block: BlockExpr<T>,
}

impl<T, E> IterationExpr<E> for PredicateLoopExpr<T> where T: 'static {}

impl<T> Expression for PredicateLoopExpr<T> {}

impl<T, E> ExprWithBlock<E> for PredicateLoopExpr<T> {}

impl<T> BooleanOperand for PredicateLoopExpr<T> where T: 'static {}

impl<T> Constant for PredicateLoopExpr<T> where T: 'static {}

impl<T> IterableExpr for PredicateLoopExpr<T> where T: 'static {}

impl<T> Spanned for PredicateLoopExpr<T> {
    fn span(&self) -> Span {
        let s1 = self.kw_while.span();
        let s2 = self.block.span();

        Span::join(s1, s2)

        // let start_pos = self.kw_while.span().start();
        // let end_pos = self.block.span().end();
        // let source = self.kw_while.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct IterLoopExpr<T> {
    kw_for: Keyword,
    pattern: Box<dyn Pattern>,
    kw_in: Keyword,
    iterator: Box<dyn IterableExpr>,
    block: BlockExpr<T>,
}

impl<T, E> IterationExpr<E> for IterLoopExpr<T> where T: 'static {}

impl<T> Expression for IterLoopExpr<T> {}

impl<T, E> ExprWithBlock<E> for IterLoopExpr<T> {}

impl<T> BooleanOperand for IterLoopExpr<T> where T: 'static {}

impl<T> IterableExpr for IterLoopExpr<T> where T: 'static {}

impl<T> Pattern for IterLoopExpr<T> {}

impl<T> Spanned for IterLoopExpr<T> {
    fn span(&self) -> Span {
        let s1 = self.kw_for.span();
        let s2 = self.block.span();

        Span::join(s1, s2)

        // let start_pos = self.kw_for.span().start();
        // let end_pos = self.block.span().end();
        // let source = self.kw_for.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

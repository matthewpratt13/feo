use crate::{
    keyword::Keyword,
    pattern::Pattern,
    span::{Span, Spanned},
};

use super::{BlockExpr, Constant, ExprWithBlock, Expression};

pub trait IterationExpr<E>
where
    Self: Sized + ExprWithBlock<E>,
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

impl<T, E> IterationExpr<E> for InfiniteLoopExpr<T> {}

impl<T> Expression for InfiniteLoopExpr<T> {}

impl<T, E> ExprWithBlock<E> for InfiniteLoopExpr<T> {}

impl<T> Constant for InfiniteLoopExpr<T> where T: 'static {}

impl<T> Spanned for InfiniteLoopExpr<T> {
    fn span(&self) -> Span {
        let start_pos = self.kw_loop.span().start();
        let end_pos = self.block.span().end();
        let source = self.kw_loop.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct PredicateLoopExpr<T> {
    kw_while: Keyword,
    conditional_operand: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T>,
}

impl<T, E> IterationExpr<E> for PredicateLoopExpr<T> {}

impl<T> Expression for PredicateLoopExpr<T> {}

impl<T, E> ExprWithBlock<E> for PredicateLoopExpr<T> {}

impl<T> Constant for PredicateLoopExpr<T> where T: 'static {}

impl<T> Spanned for PredicateLoopExpr<T> {
    fn span(&self) -> Span {
        let start_pos = self.kw_while.span().start();
        let end_pos = self.block.span().end();
        let source = self.kw_while.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct IterLoopExpr<T> {
    kw_for: Keyword,
    pattern: Box<dyn Pattern>,
    kw_in: Keyword,
    iterator: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T>,
}

impl<T, E> IterationExpr<E> for IterLoopExpr<T> {}

impl<T> Expression for IterLoopExpr<T> {}

impl<T, E> ExprWithBlock<E> for IterLoopExpr<T> {}

impl<T> Pattern for IterLoopExpr<T> {}

impl<T> Spanned for IterLoopExpr<T> {
    fn span(&self) -> Span {
        let start_pos = self.kw_for.span().start();
        let end_pos = self.block.span().end();
        let source = self.kw_for.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

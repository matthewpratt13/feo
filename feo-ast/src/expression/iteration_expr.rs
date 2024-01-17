use feo_types::span::{Span, Spanned};

use crate::{keyword::Keyword, pattern::Pattern};

use super::{BlockExpr, ExprWithBlock, Expression, IterationExpr};

pub struct InfiniteLoopExpr<T, U> {
    kw_loop: Keyword,
    block: BlockExpr<T, U>,
}

impl<T, U> Expression for InfiniteLoopExpr<T, U> {}

impl<T, U, E> ExprWithBlock<E> for InfiniteLoopExpr<T, U> {}

impl<T, U, E> IterationExpr<E> for InfiniteLoopExpr<T, U> {}

impl<T, U> Spanned for InfiniteLoopExpr<T, U> {
    fn span(&self) -> Span {
        let start_pos = self.kw_loop.span().start();
        let end_pos = self.block.span().end();
        let source = self.block.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct PredicateLoopExpr<T, U> {
    kw_while: Keyword,
    predicate: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T, U>,
}

impl<T, U> Expression for PredicateLoopExpr<T, U> {}

impl<T, U, E> ExprWithBlock<E> for PredicateLoopExpr<T, U> {}

impl<T, U, E> IterationExpr<E> for PredicateLoopExpr<T, U> {}

impl<T, U> Spanned for PredicateLoopExpr<T, U> {
    fn span(&self) -> Span {
        let start_pos = self.kw_while.span().start();
        let end_pos = self.block.span().end();
        let source = self.block.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct IterLoopExpr<T, U> {
    kw_for: Keyword,
    pattern: Box<dyn Pattern>,
    kw_in: Keyword,
    expression: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T, U>,
}

impl<T, U> Expression for IterLoopExpr<T, U> {}

impl<T, U, E> ExprWithBlock<E> for IterLoopExpr<T, U> {}

impl<T, U, E> IterationExpr<E> for IterLoopExpr<T, U> {}

impl<T, U> Spanned for IterLoopExpr<T, U> {
    fn span(&self) -> Span {
        let start_pos = self.kw_for.span().start();
        let end_pos = self.block.span().end();
        let source = self.block.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

use crate::{keyword::KeywordKind, pattern::Pattern};

use super::{BlockExpr, ExprWithBlock, Expression, IterationExpr};

pub struct InfiniteLoopExpr<T, U> {
    kw_loop: KeywordKind,
    block: BlockExpr<T, U>,
}

impl<T, U> Expression for InfiniteLoopExpr<T, U> {}

impl<T, U, E> ExprWithBlock<E> for InfiniteLoopExpr<T, U> where E: Expression {}

impl<T, U, I> IterationExpr<I> for InfiniteLoopExpr<T, U> where I: Expression {}

pub struct PredicateLoopExpr<T, U> {
    kw_while: KeywordKind,
    predicate: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T, U>,
}

impl<T, U> Expression for PredicateLoopExpr<T, U> {}

impl<T, U, E> ExprWithBlock<E> for PredicateLoopExpr<T, U> where E: Expression {}

impl<T, U, I> IterationExpr<I> for PredicateLoopExpr<T, U> where I: Expression {}

pub struct IterLoopExpr<T, U> {
    kw_for: KeywordKind,
    pattern: Box<dyn Pattern>,
    kw_in: KeywordKind,
    expression: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T, U>,
}

impl<T, U> Expression for IterLoopExpr<T, U> {}

impl<T, U, I> IterationExpr<I> for IterLoopExpr<T, U> where I: Expression {}

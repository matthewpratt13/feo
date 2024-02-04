use feo_types::{
    span::{Span, Spanned},
    utils::{KwFor, KwIn, KwLoop, KwWhile},
    Keyword,
};

use crate::{pattern::Pattern, statement::Statement};

use super::{BlockExpr, BooleanOperand, Constant, ExprWithBlock, ExprWithoutBlock, IterableExpr};

pub trait IterationExpr
where
    Self: ExprWithBlock + BooleanOperand + IterableExpr,
{
}

pub enum IterationExprKind<
    B: BooleanOperand,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
> {
    InfiniteLoop(InfiniteLoopExpr<E, S>),
    PredicateLoop(PredicateLoopExpr<B, E, S>),
    IterLoop(IterLoopExpr<E, I, S>),
}

impl<B, E, I, S> Spanned for IterationExprKind<B, E, I, S>
where
    B: BooleanOperand,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
{
    fn span(&self) -> Span {
        match self {
            IterationExprKind::InfiniteLoop(inf) => inf.span(),
            IterationExprKind::PredicateLoop(pl) => pl.span(),
            IterationExprKind::IterLoop(itl) => itl.span(),
        }
    }
}

#[allow(dead_code)]
pub type BreakExpr = Keyword;

#[allow(dead_code)]
pub type ContinueExpr = Keyword;

pub struct InfiniteLoopExpr<E: ExprWithoutBlock, S: Statement> {
    kw_loop: KwLoop,
    block: BlockExpr<E, S>,
}

impl<E, S> IterationExpr for InfiniteLoopExpr<E, S>
where
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<E, S> ExprWithBlock for InfiniteLoopExpr<E, S>
where
    E: ExprWithoutBlock,
    S: Statement,
{
}

impl<E, S> BooleanOperand for InfiniteLoopExpr<E, S>
where
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<E, S> IterableExpr for InfiniteLoopExpr<E, S>
where
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<E, S> Constant for InfiniteLoopExpr<E, S>
where
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<E, S> Spanned for InfiniteLoopExpr<E, S>
where
    E: ExprWithoutBlock,
    S: Statement,
{
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

pub struct PredicateLoopExpr<B: BooleanOperand, E: ExprWithoutBlock, S: Statement> {
    kw_while: KwWhile,
    conditional_operand: B,
    block: BlockExpr<E, S>,
}

impl<B, E, S> IterationExpr for PredicateLoopExpr<B, E, S>
where
    B: BooleanOperand,
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<B, E, S> ExprWithBlock for PredicateLoopExpr<B, E, S>
where
    B: BooleanOperand,
    E: ExprWithoutBlock,
    S: Statement,
{
}

impl<B, E, S> BooleanOperand for PredicateLoopExpr<B, E, S>
where
    B: BooleanOperand,
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<B, E, S> IterableExpr for PredicateLoopExpr<B, E, S>
where
    B: BooleanOperand,
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<B, E, S> Constant for PredicateLoopExpr<B, E, S>
where
    B: BooleanOperand,
    E: ExprWithoutBlock + 'static,
    S: Statement + 'static,
{
}

impl<B, E, S> Spanned for PredicateLoopExpr<B, E, S>
where
    B: BooleanOperand,
    E: ExprWithoutBlock,
    S: Statement,
{
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

pub struct IterLoopExpr<E: ExprWithoutBlock, I: IterableExpr, S: Statement> {
    kw_for: KwFor,
    pattern: Box<dyn Pattern>,
    kw_in: KwIn,
    iterator: I,
    block: BlockExpr<E, S>,
}

impl<E, I, S> IterationExpr for IterLoopExpr<E, I, S>
where
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
{
}

impl<E, I, S> ExprWithBlock for IterLoopExpr<E, I, S>
where
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
{
}

impl<E, I, S> BooleanOperand for IterLoopExpr<E, I, S>
where
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
{
}

impl<E, I, S> IterableExpr for IterLoopExpr<E, I, S>
where
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
{
}

impl<E, I, S> Pattern for IterLoopExpr<E, I, S>
where
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
{
}

impl<E, I, S> Spanned for IterLoopExpr<E, I, S>
where
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
{
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

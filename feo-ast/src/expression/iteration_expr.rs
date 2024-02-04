use feo_types::{
    span::{Span, Spanned},
    utils::{KwFor, KwIn, KwLoop, KwWhile},
    Keyword,
};

use crate::pattern::Pattern;

use super::{BlockExpr, BooleanOperand, IterableExpr};

pub enum IterationExprKind<T, U> {
    InfiniteLoop(InfiniteLoopExpr<T, U>),
    PredicateLoop(PredicateLoopExpr<T, U>),
    IterLoop(IterLoopExpr<T, U>),
}

impl<T, U> Spanned for IterationExprKind<T, U> {
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

pub struct InfiniteLoopExpr<T, U> {
    kw_loop: KwLoop,
    block: BlockExpr<T, U>,
}

impl<T, U> Spanned for InfiniteLoopExpr<T, U> {
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

pub struct PredicateLoopExpr<T, U> {
    kw_while: KwWhile,
    conditional_operand: BooleanOperand<T, U>,
    block: BlockExpr<T, U>,
}

impl<T, U> Spanned for PredicateLoopExpr<T, U> {
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

pub struct IterLoopExpr<T, U> {
    kw_for: KwFor,
    pattern: Box<dyn Pattern>,
    kw_in: KwIn,
    iterator: IterableExpr<T, U>,
    block: BlockExpr<T, U>,
}

impl<T, U> Pattern for IterLoopExpr<T, U> {}

impl<T, U> Spanned for IterLoopExpr<T, U> {
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

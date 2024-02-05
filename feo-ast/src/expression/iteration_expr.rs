use feo_types::{
    span::{Span, Spanned},
    utils::{KwFor, KwIn, KwLoop, KwWhile},
    Keyword,
};

use crate::pattern::Pattern;

use super::{BlockExpr, BooleanOperand, IterableExpr};

pub enum IterationExprKind {
    InfiniteLoop(InfiniteLoopExpr),
    PredicateLoop(PredicateLoopExpr),
    IterLoop(IterLoopExpr),
}

impl Spanned for IterationExprKind {
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

pub struct InfiniteLoopExpr {
    kw_loop: KwLoop,
    block: BlockExpr,
}

impl Spanned for InfiniteLoopExpr {
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

pub struct PredicateLoopExpr {
    kw_while: KwWhile,
    conditional_operand: Box<BooleanOperand>,
    block: Box<BlockExpr>,
}

impl Spanned for PredicateLoopExpr {
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

pub struct IterLoopExpr {
    kw_for: KwFor,
    pattern: Box<dyn Pattern>,
    kw_in: KwIn,
    iterator: Box<IterableExpr>,
    block: BlockExpr,
}

impl Pattern for IterLoopExpr {}

impl Spanned for IterLoopExpr {
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

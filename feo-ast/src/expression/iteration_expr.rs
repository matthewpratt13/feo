use feo_types::{
    span::{Span, Spanned},
    utils::{KwBreak, KwContinue, KwFor, KwIn, KwLoop, KwWhile},
};

use crate::pattern::Pattern;

use super::{BlockExpr, BooleanOperand, Iterable};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct BreakExpr(pub KwBreak);

impl Spanned for BreakExpr {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[derive(Debug, Clone)]
pub struct ContinueExpr(pub KwContinue);

impl Spanned for ContinueExpr {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[derive(Debug, Clone)]
pub struct InfiniteLoopExpr {
    kw_loop: KwLoop,
    block: BlockExpr,
}

impl Spanned for InfiniteLoopExpr {
    fn span(&self) -> Span {
        let s1 = self.kw_loop.span();
        let s2 = self.block.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct PredicateLoopExpr {
    pub kw_while: KwWhile,
    pub conditional_operand: Box<BooleanOperand>,
    pub block: Box<BlockExpr>,
}

impl Spanned for PredicateLoopExpr {
    fn span(&self) -> Span {
        let s1 = self.kw_while.span();
        let s2 = self.block.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct IterLoopExpr {
    kw_for: KwFor,
    pattern: Box<Pattern>,
    kw_in: KwIn,
    iterator: Box<Iterable>,
    block: BlockExpr,
}

impl Spanned for IterLoopExpr {
    fn span(&self) -> Span {
        let s1 = self.kw_for.span();
        let s2 = self.block.span();

        Span::join(s1, s2)
    }
}

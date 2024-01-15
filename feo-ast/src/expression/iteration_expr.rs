use crate::{keyword::KeywordKind, pattern::PatternKind};

use super::{BlockExpr, ExpressionKind};

pub enum IterationExprKind {
    InfiniteLoop(InfiniteLoopExpr),
    IterLoop(IterLoopExpr),
    PredicateLoop(PredicateLoopExpr),
}

pub struct InfiniteLoopExpr {
    kw_loop: KeywordKind,
    block: BlockExpr,
}

pub struct PredicateLoopExpr {
    kw_while: KeywordKind,
    predicate: Box<ExpressionKind>, // cannot be a struct expression
    block: BlockExpr,
}

pub struct IterLoopExpr {
    kw_for: KeywordKind,
    pattern: Box<PatternKind>,
    kw_in: KeywordKind,
    expression: Box<ExpressionKind>, // cannot be a struct expression
    block: BlockExpr,
}

use crate::{keyword::KeywordKind, pattern::Pattern};

use super::block_expr::BlockExpr;
use super::Expression;

pub enum IterationExpr {
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
    predicate: Box<Expression>, // cannot be a struct expression
    block: BlockExpr,
}

pub struct IterLoopExpr {
    kw_for: KeywordKind,
    pattern: Box<Pattern>,
    kw_in: KeywordKind,
    expression: Box<Expression>, // cannot be a struct expression
    block: BlockExpr,
}

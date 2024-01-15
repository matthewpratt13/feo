use crate::{keyword::KeywordKind, pattern::PatternKind};

use super::{BlockExpr, Expression};

// pub enum IterationExprKind {
//     InfiniteLoop(InfiniteLoopExpr),
//     IterLoop(IterLoopExpr),
//     PredicateLoop(PredicateLoopExpr),
// }

pub struct InfiniteLoopExpr<T> {
    kw_loop: KeywordKind,
    block: BlockExpr<T>,
}

pub struct PredicateLoopExpr<T> {
    kw_while: KeywordKind,
    predicate: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T>,
}

pub struct IterLoopExpr<T> {
    kw_for: KeywordKind,
    pattern: Box<PatternKind>,
    kw_in: KeywordKind,
    expression: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T>,
}

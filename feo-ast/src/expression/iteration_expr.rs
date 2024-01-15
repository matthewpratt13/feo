use crate::{keyword::KeywordKind, pattern::PatternKind};

use super::{BlockExpr, Expression};

// pub enum IterationExprKind {
//     InfiniteLoop(InfiniteLoopExpr),
//     IterLoop(IterLoopExpr),
//     PredicateLoop(PredicateLoopExpr),
// }

pub struct InfiniteLoopExpr<T, U> {
    kw_loop: KeywordKind,
    block: BlockExpr<T, U>,
}

pub struct PredicateLoopExpr<T, U> {
    kw_while: KeywordKind,
    predicate: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T, U>,
}

pub struct IterLoopExpr<T, U> {
    kw_for: KeywordKind,
    pattern: Box<PatternKind>,
    kw_in: KeywordKind,
    expression: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T, U>,
}

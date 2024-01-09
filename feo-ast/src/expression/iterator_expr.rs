use crate::keyword::KeywordKind;

pub enum IteratorExpr {
    Loop(LoopExpr),
    InfiniteLoop(InfiniteLoopExpr),
    IterLoop(IterLoopExpr),
    PredicateLoop(PredicateLoopExpr),
    Break(KeywordKind),
    Continue(KeywordKind),
}

pub struct LoopExpr {}

pub struct InfiniteLoopExpr {}

pub struct IterLoopExpr {}

pub struct PredicateLoopExpr {}

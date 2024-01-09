use crate::keyword::KeywordKind;

pub enum IteratorExpr {
    Loop(LoopExpr),
    IterLoop(IterLoopExpr),
    PredicateLoop(PredicateLoopExpr),
    Break(KeywordKind),
    Continue(KeywordKind),
}

pub struct LoopExpr {}

pub struct InfiniteLoopExpr {}

pub struct PredicateLoopExpr {}

pub struct IterLoopExpr {}

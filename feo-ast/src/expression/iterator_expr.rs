use crate::keyword::KeywordKind;

pub enum IteratorExpr {
    Loop,
    IteratorLoop,
    PredicateLoop,
    Break(KeywordKind),
    Continue(KeywordKind),
}

pub struct LoopExpr {}

pub struct InfiniteLoopExpr {}

pub struct PredicateLoopExpr {}

pub struct IteratorLoopExpr {}

pub struct BreakExpr {}

pub struct ContinueExpr {}

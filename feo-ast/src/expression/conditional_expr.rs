use crate::{keyword::KeywordKind, punctuation::PuncKind};

pub enum ConditionExpr {
    Loop,
    IteratorLoop,
    PredicateLoop,
    Break(KeywordKind),
    Continue(KeywordKind),
    IfExpr,
    MatchExpr,
    ReturnExpr,
    UnderscoreExpr(PuncKind),
}

pub struct LoopExpr {}

pub struct InfiniteLoopExpr {}

pub struct PredicateLoopExpr {}

pub struct IteratorLoopExpr {}

pub struct BreakExpr {}

pub struct ContinueExpr {}

pub struct IfExpr {}

pub struct MatchExpr {}

pub struct MatchArms {}

pub struct MatchArmGuard {}


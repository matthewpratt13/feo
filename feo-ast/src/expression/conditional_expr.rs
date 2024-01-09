use crate::punctuation::PuncKind;

pub enum ConditionalExpr {
    IfExpr,
    MatchExpr,
    ReturnExpr,
    UnderscoreExpr(PuncKind),
}

pub struct IfExpr {}

pub struct MatchExpr {}

pub struct MatchArms {}

pub struct MatchArmGuard {}

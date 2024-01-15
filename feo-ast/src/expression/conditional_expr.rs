use crate::{
    keyword::KeywordKind,
    pattern::PatternKind,
    type_utils::{Brace, Comma, FatArrow},
};

use super::{BlockExpr, ExpressionKind};

pub enum ConditionalExprKind {
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
}

pub struct IfExpr {
    kw_if: KeywordKind,
    condition: Box<ExpressionKind>, // cannot be a struct expression
    block: BlockExpr,
    else_if_block_opt: Option<(KeywordKind, Box<IfExpr>)>,
    else_block_opt: Option<(KeywordKind, BlockExpr)>,
}

pub struct MatchExpr {
    kw_match: KeywordKind,
    scrutinee: Box<ExpressionKind>, // cannot be a struct expression
    open_brace: Brace,
    match_arms_opt: Option<MatchArms>,
    close_brace: Brace,
}

pub struct MatchArms {
    arms: Vec<(MatchArm, FatArrow, ExpressionKind, Option<Comma>)>,
    final_arm: (MatchArm, FatArrow, Box<ExpressionKind>, Option<Comma>),
}

pub struct MatchArm {
    pattern: Box<PatternKind>,
    match_arm_guard_opt: Option<MatchArmGuard>,
}

pub struct MatchArmGuard {
    kw_if: KeywordKind,
    condition: Box<ExpressionKind>,
}

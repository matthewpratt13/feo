use crate::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    pattern::Pattern,
    punctuation::PuncKind,
};

use super::block_expr::BlockExpr;
use super::Expression;

pub enum ConditionalExpr {
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
}

pub type UnderscoreExpr = PuncKind;

pub struct IfExpr {
    kw_if: KeywordKind,
    condition: Box<Expression>, // cannot be a struct expression
    block_expr: BlockExpr,
    else_if_expr_opt: Option<(KeywordKind, Box<IfExpr>)>,
    else_block_opt: Option<(KeywordKind, BlockExpr)>,
}

pub struct MatchExpr {
    kw_match: KeywordKind,
    scrutinee: Box<Expression>, // cannot be a struct expression
    open_brace: (DelimKind, DelimOrientation),
    match_arms_opt: Option<MatchArms>,
    close_brace: (DelimKind, DelimOrientation),
}

pub struct MatchArms {
    arm: Vec<(MatchArm, PuncKind, Expression, Option<PuncKind>)>,
    final_arm: (MatchArm, PuncKind, Box<Expression>, Option<PuncKind>),
}

pub struct MatchArm {
    pattern: Pattern,
    match_arm_guard_opt: Option<MatchArmGuard>,
}

pub struct MatchArmGuard {
    kw_if: KeywordKind,
    condition: Box<Expression>,
}

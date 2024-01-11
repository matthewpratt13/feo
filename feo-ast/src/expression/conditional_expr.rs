use crate::{
    delimiter::{DelimKind, DelimOrientation},
    item::{Comma, FatArrow},
    keyword::KeywordKind,
    pattern::Pattern,
};

use super::block_expr::BlockExpr;
use super::Expression;

pub enum ConditionalExpr {
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
}

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
    arms: Vec<(MatchArm, FatArrow, Expression, Option<Comma>)>,
    final_arm: (MatchArm, FatArrow, Box<Expression>, Option<Comma>),
}

pub struct MatchArm {
    pattern: Pattern,
    match_arm_guard_opt: Option<MatchArmGuard>,
}

pub struct MatchArmGuard {
    kw_if: KeywordKind,
    condition: Box<Expression>,
}

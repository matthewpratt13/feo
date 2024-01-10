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

pub struct IfExpr {
    kw_if: KeywordKind,
    condition: Box<Expression>, // cannot be a struct expression
    block_expr: BlockExpr,
    else_if_expr_opt: Option<(KeywordKind, Box<IfExpr>)>,
    else_block_opt: Option<(KeywordKind, BlockExpr)>,
}

pub struct MatchExpr {
    kw_match: KeywordKind,
    condition: Box<Expression>, // cannot be a struct expression
    open_brace: (DelimKind, DelimOrientation),
    match_arms_opt: Option<MatchArms>,
    close_brace: (DelimKind, DelimOrientation),
}

pub struct MatchArms {
    pattern: Pattern,
    match_arm_guard_opt: Option<MatchArmGuard>,
}

pub struct MatchArmGuard {
    kw_if: KeywordKind,
    condition: Box<Expression>,
}

pub type UnderscoreExpr = PuncKind;

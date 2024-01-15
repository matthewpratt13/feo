use crate::{
    keyword::KeywordKind,
    pattern::Pattern,
    type_utils::{Brace, Comma, FatArrow},
};

use super::{BlockExpr, Expression};

pub struct IfExpr<T, U> {
    kw_if: KeywordKind,
    condition: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T, U>,
    else_if_block_opt: Option<(KeywordKind, Box<IfExpr<T, U>>)>,
    else_block_opt: Option<(KeywordKind, BlockExpr<T, U>)>,
}

pub struct MatchExpr {
    kw_match: KeywordKind,
    scrutinee: Box<dyn Expression>, // cannot be a struct expression
    open_brace: Brace,
    match_arms_opt: Option<MatchArms>,
    close_brace: Brace,
}

pub struct MatchArms {
    arms: Vec<(MatchArm, FatArrow, Box<dyn Expression>, Option<Comma>)>,
    final_arm: (MatchArm, FatArrow, Box<dyn Expression>, Option<Comma>),
}

pub struct MatchArm {
    pattern: Box<dyn Pattern>,
    match_arm_guard_opt: Option<MatchArmGuard>,
}

pub struct MatchArmGuard {
    kw_if: KeywordKind,
    condition: Box<dyn Expression>,
}

use crate::{
    keyword::KeywordKind,
    pattern::PatternKind,
    type_utils::{Brace, Comma, FatArrow},
};

use super::{BlockExpr, Expression};

// pub enum ConditionalExprKind<T> {
//     IfExpr(IfExpr<T>),
//     MatchExpr(MatchExpr),
// }

pub struct IfExpr<T> {
    kw_if: KeywordKind,
    condition: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T>,
    else_if_block_opt: Option<(KeywordKind, Box<IfExpr<T>>)>,
    else_block_opt: Option<(KeywordKind, BlockExpr<T>)>,
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
    pattern: Box<PatternKind>,
    match_arm_guard_opt: Option<MatchArmGuard>,
}

pub struct MatchArmGuard {
    kw_if: KeywordKind,
    condition: Box<dyn Expression>,
}

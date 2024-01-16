use crate::{
    keyword::KeywordKind,
    pattern::Pattern,
    type_utils::{Brace, Comma, FatArrow},
};

use super::{Attribute, BlockExpr, ConditionalExpr, ExprWithBlock, Expression};

pub struct IfExpr<T, U> {
    kw_if: KeywordKind,
    condition: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T, U>,
    else_if_block_opt: Option<(KeywordKind, Box<IfExpr<T, U>>)>,
    else_block_opt: Option<(KeywordKind, BlockExpr<T, U>)>,
}

impl<T, U> Expression for IfExpr<T, U> {}

impl<T, U, C> ConditionalExpr<C> for IfExpr<T, U> where C: Expression {}

impl<T, U, E> ExprWithBlock<E> for IfExpr<T, U> where E: Expression {}

pub struct MatchExpr {
    kw_match: KeywordKind,
    scrutinee: Box<dyn Expression>, // cannot be a struct expression
    open_brace: Brace,
    attributes: Vec<Attribute>,
    match_arms_opt: Option<MatchArms>,
    close_brace: Brace,
}

impl Expression for MatchExpr {}

impl<C> ConditionalExpr<C> for MatchExpr where C: Expression {}

impl<E> ExprWithBlock<E> for MatchExpr where E: Expression {}

pub struct MatchArms {
    arms: Vec<(MatchArm, FatArrow, Box<dyn Expression>, Option<Comma>)>,
    final_arm: (MatchArm, FatArrow, Box<dyn Expression>, Option<Comma>),
}

pub struct MatchArm {
    attributes: Vec<Attribute>,
    pattern: Box<dyn Pattern>,
    match_arm_guard_opt: Option<MatchArmGuard>,
}

pub struct MatchArmGuard {
    kw_if: KeywordKind,
    condition: Box<dyn Expression>,
}

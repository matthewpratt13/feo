use crate::{
    keyword::Keyword,
    pattern::Pattern,
    span::{Span, Spanned},
    type_utils::{Brace, Comma, FatArrow},
};

use super::{
    AssignableExpr, BlockExpr, BooleanOperand, Constant, ExprWithBlock, Expression, InnerAttr,
    IterableExpr, OuterAttr,
};

pub trait ConditionalExpr<E>
where
    Self: ExprWithBlock<E> + IterableExpr + BooleanOperand + Constant,
{
}

pub struct IfExpr<T> {
    kw_if: Keyword,
    conditional_operand: Box<dyn BooleanOperand>,
    block: BlockExpr<T>,
    else_if_block_opt: Option<(Keyword, Box<IfExpr<T>>)>,
    else_block_opt: Option<(Keyword, BlockExpr<T>)>,
}

impl<T, E> ConditionalExpr<E> for IfExpr<T> where T: 'static {}

impl<T> Expression for IfExpr<T> {}

impl<T, E> ExprWithBlock<E> for IfExpr<T> {}

impl<T> BooleanOperand for IfExpr<T> where T: 'static {}

impl<T> Constant for IfExpr<T> where T: 'static {}

impl<T> IterableExpr for IfExpr<T> where T: 'static {}

impl<T> Spanned for IfExpr<T> {
    fn span(&self) -> Span {
        let start_pos = self.kw_if.span().start();
        let temp_end = self.block.span().end();

        let end_pos = match &self.else_if_block_opt {
            Some(s) => match &self.else_block_opt {
                Some(t) => t.1.span().end(),
                None => s.1.span().end(),
            },

            None => temp_end,
        };

        let source = self.kw_if.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct MatchExpr {
    kw_match: Keyword,
    scrutinee: Box<dyn AssignableExpr>,
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    match_arms_opt: Option<MatchArms>,
    close_brace: Brace,
}

impl<E> ConditionalExpr<E> for MatchExpr {}

impl Expression for MatchExpr {}

impl<E> ExprWithBlock<E> for MatchExpr {}

impl BooleanOperand for MatchExpr {}

impl Constant for MatchExpr {}

impl IterableExpr for MatchExpr {}

impl Pattern for MatchExpr {}

impl Spanned for MatchExpr {
    fn span(&self) -> Span {
        let start_pos = self.kw_match.span().start();
        let end_pos = self.close_brace.span().end();
        let source = self.kw_match.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct MatchArms {
    arms: Vec<(MatchArm, FatArrow, Box<dyn Expression>, Option<Comma>)>,
    final_arm: (MatchArm, FatArrow, Box<dyn Expression>, Option<Comma>),
}

pub struct MatchArm {
    attributes: Vec<OuterAttr>,
    pattern: Box<dyn Pattern>,
    match_arm_guard_opt: Option<MatchArmGuard>,
}

pub struct MatchArmGuard {
    kw_if: Keyword,
    operand: Box<dyn Expression>,
}

use feo_types::span::{Span, Spanned};

use crate::{
    keyword::Keyword,
    pattern::Pattern,
    type_utils::{Brace, Comma, FatArrow},
};

use super::{BlockExpr, ConditionalExpr, ExprWithBlock, Expression, InnerAttr, OuterAttr};

pub struct IfExpr<T, U> {
    kw_if: Keyword,
    condition: Box<dyn Expression>, // cannot be a struct expression
    block: BlockExpr<T, U>,

    // TODO: should these rather be enum variants?
    else_if_block_opt: Option<(Keyword, Box<IfExpr<T, U>>)>,
    else_block_opt: Option<(Keyword, BlockExpr<T, U>)>,
}

impl<T, U> Expression for IfExpr<T, U> {}

impl<T, U, E> ConditionalExpr<E> for IfExpr<T, U> {}

impl<T, U, E> ExprWithBlock<E> for IfExpr<T, U> {}

impl<T, U> Spanned for IfExpr<T, U> {
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
    scrutinee: Box<dyn Expression>, // cannot be a struct expression
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    match_arms_opt: Option<MatchArms>,
    close_brace: Brace,
}

impl Expression for MatchExpr {}

impl<E> ConditionalExpr<E> for MatchExpr {}

impl<E> ExprWithBlock<E> for MatchExpr {}

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
    condition: Box<dyn Expression>,
}

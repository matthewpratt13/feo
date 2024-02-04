use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Comma, FatArrow, KwElse, KwIf, KwMatch},
};

use crate::pattern::Pattern;

use super::{Assignable, BlockExpr, BooleanOperand, Expression, InnerAttr, OuterAttr};

pub struct IfExpr<T, U> {
    kw_if: KwIf,
    condition_operand: BooleanOperand<T, U>,
    if_block: BlockExpr<T, U>,
    else_if_blocks_opt: Option<Vec<(KwElse, Box<IfExpr<T, U>>)>>,
    else_block_opt: Option<(KwElse, BlockExpr<T, U>)>,
}

impl<T, U> Spanned for IfExpr<T, U> {
    fn span(&self) -> Span {
        let s1 = self.kw_if.span();
        let s2 = match &self.else_block_opt {
            Some(e) => e.1.span(),
            None => match &self.else_if_blocks_opt {
                Some(ei) => match ei.last() {
                    Some(b) => b.1.span(),
                    None => self.if_block.span(),
                },
                None => self.if_block.span(),
            },
        };

        Span::join(s1, s2)

        // let start_pos = self.kw_if.span().start();
        // let temp_end = self.block.span().end();

        // let end_pos = match &self.else_if_block_opt {
        //     Some(s) => match &self.else_block_opt {
        //         Some(t) => t.1.span().end(),
        //         None => s.1.span().end(),
        //     },

        //     None => temp_end,
        // };

        // let source = self.kw_if.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct MatchExpr<T, U> {
    kw_match: KwMatch,
    scrutinee: Assignable<T, U>,
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    match_arms_opt: Option<MatchArms<T, U>>,
    close_brace: Brace,
}

impl<T, U> Pattern for MatchExpr<T, U> {}

impl<T, U> Spanned for MatchExpr<T, U> {
    fn span(&self) -> Span {
        let s1 = self.kw_match.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)

        // let start_pos = self.kw_match.span().start();
        // let end_pos = self.close_brace.span().end();
        // let source = self.kw_match.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct MatchArms<T, U> {
    arms: Vec<(MatchArm<T, U>, FatArrow, Expression, Option<Comma>)>,
    final_arm: (MatchArm<T, U>, FatArrow, Expression, Option<Comma>),
}

pub struct MatchArm<T, U> {
    attributes: Vec<OuterAttr>,
    pattern: Box<dyn Pattern>,
    match_arm_guard_opt: Option<MatchArmGuard<T, U>>,
}

pub struct MatchArmGuard<T, U> {
    kw_if: KwIf,
    operand: BooleanOperand<T, U>,
}

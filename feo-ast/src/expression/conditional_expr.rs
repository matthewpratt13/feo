use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Comma, FatArrow, KwElse, KwIf, KwMatch},
};

use crate::pattern::Pattern;

use super::{Assignable, BlockExpr, BooleanOperand, Expression, InnerAttr, OuterAttr};

#[derive(Clone)]
pub struct IfExpr {
    kw_if: KwIf,
    condition_operand: Box<BooleanOperand>,
    if_block: Box<BlockExpr>,
    else_if_blocks_opt: Option<Vec<(KwElse, Box<IfExpr>)>>,
    else_block_opt: Option<(KwElse, BlockExpr)>,
}

impl Spanned for IfExpr {
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

#[derive(Clone)]
pub struct MatchExpr {
    kw_match: KwMatch,
    scrutinee: Box<Assignable>, // except struct expression
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    match_arms_opt: Option<MatchArms>,
    close_brace: Brace,
}

impl Spanned for MatchExpr {
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

#[derive(Clone)]
pub struct MatchArms {
    arms: Vec<(MatchArm, FatArrow, Expression, Option<Comma>)>,
    final_arm: (MatchArm, FatArrow, Box<Expression>, Option<Comma>),
}

#[derive(Clone)]
pub struct MatchArm {
    attributes: Vec<OuterAttr>,
    pattern: Box<Pattern>,
    match_arm_guard_opt: Option<MatchArmGuard>,
}

#[derive(Clone)]
pub struct MatchArmGuard {
    kw_if: KwIf,
    operand: Box<BooleanOperand>,
}

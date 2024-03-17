use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, KwElse, KwIf, KwMatch},
};

use crate::{
    attribute::{InnerAttr, OuterAttr},
    pattern::Pattern,
};

use super::{BlockExpr, Expression, Value};

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub kw_if: KwIf,
    pub condition_operand: Box<Expression>,
    pub if_block: Box<BlockExpr>,
    pub else_if_blocks_opt: Option<Vec<(KwElse, Box<IfExpr>)>>,
    pub trailing_else_block_opt: Option<(KwElse, BlockExpr)>,
}

impl Spanned for IfExpr {
    fn span(&self) -> Span {
        let s1 = self.kw_if.span();
        let s2 = match &self.trailing_else_block_opt {
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
    }
}

#[derive(Debug, Clone)]
pub struct MatchExpr {
    pub kw_match: KwMatch,
    pub scrutinee: Box<Value>, // except struct expression
    pub open_brace: Brace,
    pub attributes_opt: Option<Vec<InnerAttr>>,
    pub match_arms_opt: Option<MatchArms>,
    pub close_brace: Brace,
}

impl Spanned for MatchExpr {
    fn span(&self) -> Span {
        let s1 = self.kw_match.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct MatchArms {
    pub arms_opt: Option<Vec<(MatchArm, Expression)>>,
    pub final_arm: (MatchArm, Box<Expression>),
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub pattern: Box<Pattern>,
    pub match_arm_guard_opt: Option<MatchArmGuard>,
}

#[derive(Debug, Clone)]
pub struct MatchArmGuard {
    pub kw_if: KwIf,
    pub operand: Box<Expression>,
}
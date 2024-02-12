use feo_types::{
    span::{Span, Spanned},
    utils::{DblDot, DotDotEquals},
};

use super::Expression;

#[derive(Clone)]
pub enum RangeExprKind {
    RangeFullExpr(RangeFullExpr),
    RangeFromToExpr(RangeFromToExpr),
    RangeFromExpr(RangeFromExpr),
    RangeToExpr(RangeToExpr),
    RangeInclusiveExpr(RangeInclusiveExpr),
    RangeToInclusiveExpr(RangeToInclusiveExpr),
}

impl Spanned for RangeExprKind {
    fn span(&self) -> Span {
        match self {
            RangeExprKind::RangeFullExpr(rfe) => rfe.span(),
            RangeExprKind::RangeFromToExpr(rft) => rft.span(),
            RangeExprKind::RangeFromExpr(rf) => rf.span(),
            RangeExprKind::RangeToExpr(rt) => rt.span(),
            RangeExprKind::RangeInclusiveExpr(ri) => ri.span(),
            RangeExprKind::RangeToInclusiveExpr(rti) => rti.span(),
        }
    }
}

pub type RangeFullExpr = DblDot;

#[derive(Clone)]
pub struct RangeFromToExpr {
    from_operand: Box<Expression>,
    dbl_dot: DblDot,
    to_operand_excl: Box<Expression>,
}

impl Spanned for RangeFromToExpr {
    fn span(&self) -> Span {
        let s1 = self.from_operand.span();
        let s2 = self.to_operand_excl.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct RangeFromExpr {
    from_operand: Box<Expression>,
    dbl_dot: DblDot,
}

impl Spanned for RangeFromExpr {
    fn span(&self) -> Span {
        let s1 = self.from_operand.span();
        let s2 = self.dbl_dot.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct RangeToExpr {
    dbl_dot: DblDot,
    to_operand: Box<Expression>,
}

impl Spanned for RangeToExpr {
    fn span(&self) -> Span {
        let s1 = self.dbl_dot.span();
        let s2 = self.to_operand.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct RangeInclusiveExpr {
    from_operand: Box<Expression>,
    dot_dot_equals: DotDotEquals,
    to_operand_incl: Box<Expression>,
}

impl Spanned for RangeInclusiveExpr {
    fn span(&self) -> Span {
        let s1 = self.from_operand.span();
        let s2 = self.to_operand_incl.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct RangeToInclusiveExpr {
    dot_dot_equals: DotDotEquals,
    to_operand_incl: Box<Expression>,
}

impl Spanned for RangeToInclusiveExpr {
    fn span(&self) -> Span {
        let s1 = self.dot_dot_equals.span();
        let s2 = self.to_operand_incl.span();

        Span::join(s1, s2)
    }
}

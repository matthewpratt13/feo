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
        let start_pos = self.from_operand.span().start();
        let end_pos = self.to_operand_excl.span().end();
        let source = self.from_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct RangeFromExpr {
    from_operand: Box<Expression>,
    dbl_dot: DblDot,
}

impl Spanned for RangeFromExpr {
    fn span(&self) -> Span {
        let start_pos = self.from_operand.span().start();
        let end_pos = self.dbl_dot.span().end();
        let source = self.from_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct RangeToExpr {
    dbl_dot: DblDot,
    to_operand: Box<Expression>,
}

impl Spanned for RangeToExpr {
    fn span(&self) -> Span {
        let start_pos = self.dbl_dot.span().start();
        let end_pos = self.to_operand.span().end();
        let source = self.dbl_dot.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
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
        let start_pos = self.from_operand.span().start();
        let end_pos = self.to_operand_incl.span().end();
        let source = self.from_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct RangeToInclusiveExpr {
    dot_dot_equals: DotDotEquals,
    to_operand_incl: Box<Expression>,
}

impl Spanned for RangeToInclusiveExpr {
    fn span(&self) -> Span {
        let start_pos = self.dot_dot_equals.span().start();
        let end_pos = self.to_operand_incl.span().end();
        let source = self.dot_dot_equals.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

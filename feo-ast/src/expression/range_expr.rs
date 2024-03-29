use feo_types::{
    span::{Span, Spanned},
    type_utils::{DblDot, DotDotEquals},
};

use super::Value;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct RangeFullExpr(pub DblDot);

impl Spanned for RangeFullExpr {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[derive(Debug, Clone)]
pub struct RangeFromToExpr {
    pub from_operand: Box<Value>,
    pub dbl_dot: DblDot,
    pub to_operand_excl: Box<Value>,
}

impl Spanned for RangeFromToExpr {
    fn span(&self) -> Span {
        let s1 = self.from_operand.span();
        let s2 = self.to_operand_excl.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct RangeFromExpr {
    pub from_operand: Box<Value>,
    pub dbl_dot: DblDot,
}

impl Spanned for RangeFromExpr {
    fn span(&self) -> Span {
        let s1 = self.from_operand.span();
        let s2 = self.dbl_dot.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct RangeToExpr {
    pub dbl_dot: DblDot,
    pub to_operand: Box<Value>,
}

impl Spanned for RangeToExpr {
    fn span(&self) -> Span {
        let s1 = self.dbl_dot.span();
        let s2 = self.to_operand.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct RangeInclusiveExpr {
    pub from_operand: Box<Value>,
    pub dot_dot_equals: DotDotEquals,
    pub to_operand_incl: Box<Value>,
}

impl Spanned for RangeInclusiveExpr {
    fn span(&self) -> Span {
        let s1 = self.from_operand.span();
        let s2 = self.to_operand_incl.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct RangeToInclusiveExpr {
    pub dot_dot_equals: DotDotEquals,
    pub to_operand_incl: Box<Value>,
}

impl Spanned for RangeToInclusiveExpr {
    fn span(&self) -> Span {
        let s1 = self.dot_dot_equals.span();
        let s2 = self.to_operand_incl.span();

        Span::join(s1, s2)
    }
}

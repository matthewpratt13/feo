use feo_types::{
    literal::{FloatType, IntType, Literal, UIntType},
    span::{Span, Spanned},
    utils::{DblDot, DotDotEquals},
    U256,
};

use crate::path::PathExpr;

#[derive(Debug, Clone)]
pub enum RangePattKind {
    RangeFromPatt(RangeFromPatt),
    RangeInclusivePatt(RangeInclusivePatt),
    RangeToInclusivePatt(RangeToInclusivePatt),
}

#[derive(Debug, Clone)]
pub enum RangePattBound {
    CharLit(Literal<char>),
    IntLit(Literal<IntType>),
    UIntLit(Literal<UIntType>),
    U256Lit(Literal<U256>),
    FloatLit(Literal<FloatType>),
    PathExpr(PathExpr),
}

impl Spanned for RangePattBound {
    fn span(&self) -> Span {
        match self {
            RangePattBound::CharLit(c) => c.span(),
            RangePattBound::IntLit(i) => i.span(),
            RangePattBound::UIntLit(ui) => ui.span(),
            RangePattBound::U256Lit(u) => u.span(),
            RangePattBound::FloatLit(f) => f.span(),
            RangePattBound::PathExpr(p) => p.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RangeFromPatt {
    pub from: RangePattBound,
    pub dbl_dot: DblDot,
}

impl Spanned for RangeFromPatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.dbl_dot.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Debug, Clone)]
pub struct RangeInclusivePatt {
    pub from: RangePattBound,
    pub dot_dot_equals: DotDotEquals,
    pub to_incl: RangePattBound,
}

impl Spanned for RangeInclusivePatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.to_incl.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Debug, Clone)]
pub struct RangeToInclusivePatt {
    pub dot_dot_equals: DotDotEquals,
    pub to_incl: RangePattBound,
}

impl Spanned for RangeToInclusivePatt {
    fn span(&self) -> Span {
        let s1 = self.dot_dot_equals.span();
        let s2 = self.to_incl.span();

        Span::join(s1, s2)
    }
}

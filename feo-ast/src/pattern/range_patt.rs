use feo_types::{
    literal::{FloatType, IntType, Literal, UIntType},
    span::{Span, Spanned},
    utils::DotDotEquals,
    U256,
};

use crate::path::PathExpr;

#[derive(Debug, Clone)]
pub enum RangePatt {
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
    from: RangePattBound,
    dot_dot_equals: DotDotEquals,
}

impl Spanned for RangeFromPatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.dot_dot_equals.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Debug, Clone)]
pub struct RangeInclusivePatt {
    from: RangePattBound,
    dot_dot_equals: DotDotEquals,
    to: RangePattBound,
}

impl Spanned for RangeInclusivePatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.to.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Debug, Clone)]
pub struct RangeToInclusivePatt {
    from: RangePattBound,
    dot_dot_equals: DotDotEquals,
    to: RangePattBound,
}

impl Spanned for RangeToInclusivePatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.to.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

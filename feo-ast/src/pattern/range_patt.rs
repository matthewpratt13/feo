use feo_types::{
    literal::{IntType, Literal, UintType},
    span::{Span, Spanned},
    utils::DotDotEquals,
    U256,
};

use crate::path::PathExpr;

#[derive(Clone)]
pub enum RangePatt {
    RangeFromPatt(RangeFromPatt),
    RangeInclusivePatt(RangeInclusivePatt),
    RangeToInclusivePatt(RangeToInclusivePatt),
}

#[derive(Clone)]
pub enum RangePattBound {
    CharLit(Literal<char>),
    IntLit(Literal<IntType>),
    UintLit(Literal<UintType>),
    U256Lit(Literal<U256>),
    F32Lit(Literal<f32>),
    F64Lit(Literal<f64>),
    PathExpr(PathExpr),
}

impl Spanned for RangePattBound {
    fn span(&self) -> Span {
        todo!()
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

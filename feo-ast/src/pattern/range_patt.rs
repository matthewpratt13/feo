use crate::{
    span::{Span, Spanned},
    type_utils::DotDotEquals,
};

use super::Pattern;

pub trait RangePatt
where
    Self: Sized + 'static + Pattern,
{
}

pub trait RangePattBound
where
    Self: Spanned,
{
}

pub struct RangeFromPatt {
    from: Box<dyn RangePattBound>,
    dot_dot_equals: DotDotEquals,
}

impl Pattern for RangeFromPatt {}

impl RangePatt for RangeFromPatt {}

impl Spanned for RangeFromPatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.dot_dot_equals.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeInclusivePatt {
    from: Box<dyn RangePattBound>,
    dot_dot_equals: DotDotEquals,
    to: Box<dyn RangePattBound>,
}

impl Pattern for RangeInclusivePatt {}

impl RangePatt for RangeInclusivePatt {}

impl Spanned for RangeInclusivePatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.to.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeToInclusivePatt {
    from: Box<dyn RangePattBound>,
    dot_dot_equals: DotDotEquals,
    to: Box<dyn RangePattBound>,
}

impl Pattern for RangeToInclusivePatt {}

impl RangePatt for RangeToInclusivePatt {}

impl Spanned for RangeToInclusivePatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.to.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

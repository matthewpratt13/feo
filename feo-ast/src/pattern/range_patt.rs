use feo_types::span::{Span, Spanned};

use crate::type_utils::DotDotEquals;

use super::{Pattern, RangePatt, RangePattBound};

pub struct RangeFromPatt<T> {
    from: Box<dyn RangePattBound<T>>,
    dot_dot_equals: DotDotEquals,
}

impl<T> Pattern for RangeFromPatt<T> {}

impl<T, R> RangePatt<R> for RangeFromPatt<T> where R: Pattern {}

impl<T> Spanned for RangeFromPatt<T> {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = self.dot_dot_equals.span().end();
        let source = self.dot_dot_equals.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeInclusivePatt<T> {
    from: Box<dyn RangePattBound<T>>,
    dot_dot_equals: DotDotEquals,
    to: Box<dyn RangePattBound<T>>,
}

impl<T> Pattern for RangeInclusivePatt<T> {}

impl<T, R> RangePatt<R> for RangeInclusivePatt<T> where R: Pattern {}

impl<T> Spanned for RangeInclusivePatt<T> {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = todo!();
        let source = self.dot_dot_equals.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeToInclusivePatt<T> {
    from: Box<dyn RangePattBound<T>>,
    dot_dot_equals: DotDotEquals,
    to: Box<dyn RangePattBound<T>>,
}

impl<T> Pattern for RangeToInclusivePatt<T> {}

impl<T, R> RangePatt<R> for RangeToInclusivePatt<T> where R: Pattern {}

impl<T> Spanned for RangeToInclusivePatt<T> {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = todo!();
        let source = self.dot_dot_equals.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

use crate::type_utils::DotDotEquals;

use super::{Pattern, RangePatt, RangePattBound};

pub struct RangeFromPatt<T> {
    from: Box<dyn RangePattBound<T>>,
    dot_dot_equals: DotDotEquals,
}

impl<T> Pattern for RangeFromPatt<T> {}

impl<T, R> RangePatt<R> for RangeFromPatt<T> where R: Pattern {}

pub struct RangeInclusivePatt<T> {
    from: Box<dyn RangePattBound<T>>,
    dot_dot_equals: DotDotEquals,
    to: Box<dyn RangePattBound<T>>,
}

impl<T> Pattern for RangeInclusivePatt<T> {}

impl<T, R> RangePatt<R> for RangeInclusivePatt<T> where R: Pattern {}

pub struct RangeToInclusivePatt<T> {
    from: Box<dyn RangePattBound<T>>,
    dot_dot_equals: DotDotEquals,
    to: Box<dyn RangePattBound<T>>,
}

impl<T> Pattern for RangeToInclusivePatt<T> {}

impl<T, R> RangePatt<R> for RangeToInclusivePatt<T> where R: Pattern {}

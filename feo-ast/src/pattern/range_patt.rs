use crate::type_utils::DotDotEquals;

use super::RangePattBound;

pub struct RangeFromPatt<T> {
    from: Box<dyn RangePattBound<T>>,
    dot_dot_equals: DotDotEquals,
}

pub struct RangeInclusivePatt<T> {
    from: Box<dyn RangePattBound<T>>,
    dot_dot_equals: DotDotEquals,
    to: Box<dyn RangePattBound<T>>,
}

pub struct RangeToInclusivePatt<T> {
    from: Box<dyn RangePattBound<T>>,
    dot_dot_equals: DotDotEquals,
    to: Box<dyn RangePattBound<T>>,
}

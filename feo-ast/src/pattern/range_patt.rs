use crate::{item::DotDotEquals, literals::{CharLiteral, IntLiteral, FloatLiteral}};

pub enum RangePattKind {
    Inclusive(RangeInclusivePatt),
    From(RangeFromPatt),
    ToInclusive(RangeToInclusivePatt),
}

pub struct RangeInclusivePatt {
    from: RangePattBound,
    dot_dot_equals: DotDotEquals,
    to: RangePattBound,
}

pub struct RangeFromPatt {
    from: RangePattBound,
    dot_dot_equals: DotDotEquals,
}

pub struct RangeToInclusivePatt {
    from: RangePattBound,
    dot_dot_equals: DotDotEquals,
    to: RangePattBound,
}

pub enum RangePattBound {
    CharLit(CharLiteral),
    IntLit(IntLiteral),
    FloatLit(FloatLiteral),
}

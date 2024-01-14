use crate::item::DotDotEquals;

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
    CharLit(char),
    IntLit(i64),
    FloatLit(f64),
}

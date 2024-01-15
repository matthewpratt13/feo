use crate::item::DotDotEquals;

pub enum RangePattKind {
    Inclusive(RangeInclusivePatt),
    From(RangeFromPatt),
    ToInclusive(RangeToInclusivePatt),
}

pub enum RangePattBound {
    CharLit(char),
    IntLit(i64),
    FloatLit(f64),
}

pub struct RangeFromPatt {
    from: RangePattBound,
    dot_dot_equals: DotDotEquals,
}

pub struct RangeInclusivePatt {
    from: RangePattBound,
    dot_dot_equals: DotDotEquals,
    to: RangePattBound,
}

pub struct RangeToInclusivePatt {
    from: RangePattBound,
    dot_dot_equals: DotDotEquals,
    to: RangePattBound,
}

use feo_types::U256;

use crate::item::DotDotEquals;

pub enum RangePattKind {
    From(RangeFromPatt),
    Inclusive(RangeInclusivePatt),
    ToInclusive(RangeToInclusivePatt),
}

pub enum RangePattBound {
    CharLit(char),
    IntLit(i64),
    UIntLit(u64),
    U256Lit(U256),
    Bytes32Lit([u8; 32]),
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

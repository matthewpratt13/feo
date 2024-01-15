use feo_types::U256;

use crate::type_utils::DotDotEquals;

pub enum RangePattBoundKind {
    CharLit(char),
    IntLit(i64),
    UIntLit(u64),
    U256Lit(U256),
    Bytes32Lit([u8; 32]),
    FloatLit(f64),
}

pub struct RangeFromPatt {
    from: RangePattBoundKind,
    dot_dot_equals: DotDotEquals,
}

pub struct RangeInclusivePatt {
    from: RangePattBoundKind,
    dot_dot_equals: DotDotEquals,
    to: RangePattBoundKind,
}

pub struct RangeToInclusivePatt {
    from: RangePattBoundKind,
    dot_dot_equals: DotDotEquals,
    to: RangePattBoundKind,
}

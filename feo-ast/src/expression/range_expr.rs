use crate::type_utils::{DblDot, DotDotEquals};

use super::Expression;

// pub enum RangeExprKind {
//     Range(RangeFromToExpr),
//     From(RangeFromExpr),
//     To(RangeToExpr),
//     Full(DblDot),
//     Inclusive(RangeInclusiveExpr),
//     ToInclusive(RangeToInclusiveExpr),
// }

pub struct RangeFromToExpr {
    from_expression: Box<dyn Expression>,
    dbl_dot: DblDot,
    to_expression_excl: Box<dyn Expression>,
}

pub struct RangeFromExpr {
    from_expression: Box<dyn Expression>,
    dbl_dot: DblDot,
}

pub struct RangeToExpr {
    dbl_dot: DblDot,
    to_expression: Box<dyn Expression>,
}

pub struct RangeInclusiveExpr {
    from_expression: Box<dyn Expression>,
    dot_dot_equals: DotDotEquals,
    to_expression_incl: Box<dyn Expression>,
}

pub struct RangeToInclusiveExpr {
    dot_dot_equals: DotDotEquals,
    to_expression_incl: Box<dyn Expression>,
}

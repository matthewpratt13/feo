use crate::type_utils::{DblDot, DotDotEquals};

use super::ExpressionKind;

pub enum RangeExprKind {
    Range(RangeFromToExpr),
    From(RangeFromExpr),
    To(RangeToExpr),
    Full(DblDot),
    Inclusive(RangeInclusiveExpr),
    ToInclusive(RangeToInclusiveExpr),
}

pub struct RangeFromToExpr {
    from_expression: Box<ExpressionKind>,
    dbl_dot: DblDot,
    to_expression_excl: Box<ExpressionKind>,
}

pub struct RangeFromExpr {
    from_expression: Box<ExpressionKind>,
    dbl_dot: DblDot,
}

pub struct RangeToExpr {
    dbl_dot: DblDot,
    to_expression: Box<ExpressionKind>,
}

pub struct RangeInclusiveExpr {
    from_expression: Box<ExpressionKind>,
    dot_dot_equals: DotDotEquals,
    to_expression_incl: Box<ExpressionKind>,
}

pub struct RangeToInclusiveExpr {
    dot_dot_equals: DotDotEquals,
    to_expression_incl: Box<ExpressionKind>,
}

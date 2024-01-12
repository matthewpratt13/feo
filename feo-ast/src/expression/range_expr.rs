use crate::item::{DblDot, DotDotEquals};

use super::Expression;

pub enum RangeExprKind {
    Range(RangeExpr),
    From(RangeFromExpr),
    To(RangeToExpr),
    Full(DblDot),
    Inclusive(RangeInclusiveExpr),
    ToInclusive(RangeToInclusiveExpr),
}

pub struct RangeExpr {
    from_expression: Box<Expression>,
    dbl_dot: DblDot,
    to_expression_excl: Box<Expression>,
}

pub struct RangeFromExpr {
    from_expression: Box<Expression>,
    dbl_dot: DblDot,
}

pub struct RangeToExpr {
    dbl_dot: DblDot,
    to_expression: Box<Expression>,
}

pub struct RangeInclusiveExpr {
    from_expression: Box<Expression>,
    dot_dot_equals: DotDotEquals,
    to_expression_incl: Box<Expression>,
}

pub struct RangeToInclusiveExpr {
    dot_dot_equals: DotDotEquals,
    to_expression_incl: Box<Expression>,
}

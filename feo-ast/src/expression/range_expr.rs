use crate::type_utils::{DblDot, DotDotEquals};

use super::{ExprWithoutBlock, Expression, RangeExpr};

pub struct RangeFromToExpr {
    from_expression: Box<dyn Expression>,
    dbl_dot: DblDot,
    to_expression_excl: Box<dyn Expression>,
}

impl Expression for RangeFromToExpr {}

impl<E> ExprWithoutBlock<E> for RangeFromToExpr where E: Expression {}

impl<R> RangeExpr<R> for RangeFromToExpr where R: Expression {}

pub struct RangeFromExpr {
    from_expression: Box<dyn Expression>,
    dbl_dot: DblDot,
}

impl Expression for RangeFromExpr {}

impl<E> ExprWithoutBlock<E> for RangeFromExpr where E: Expression {}

impl<R> RangeExpr<R> for RangeFromExpr where R: Expression {}

pub struct RangeToExpr {
    dbl_dot: DblDot,
    to_expression: Box<dyn Expression>,
}

impl Expression for RangeToExpr {}

impl<E> ExprWithoutBlock<E> for RangeToExpr where E: Expression {}

impl<R> RangeExpr<R> for RangeToExpr where R: Expression {}

pub struct RangeInclusiveExpr {
    from_expression: Box<dyn Expression>,
    dot_dot_equals: DotDotEquals,
    to_expression_incl: Box<dyn Expression>,
}

impl Expression for RangeInclusiveExpr {}

impl<E> ExprWithoutBlock<E> for RangeInclusiveExpr where E: Expression {}

impl<R> RangeExpr<R> for RangeInclusiveExpr where R: Expression {}

pub struct RangeToInclusiveExpr {
    dot_dot_equals: DotDotEquals,
    to_expression_incl: Box<dyn Expression>,
}

impl Expression for RangeToInclusiveExpr {}

impl<E> ExprWithoutBlock<E> for RangeToInclusiveExpr where E: Expression {}

impl<R> RangeExpr<R> for RangeToInclusiveExpr where R: Expression {}

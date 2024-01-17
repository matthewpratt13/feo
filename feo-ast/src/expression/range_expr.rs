use feo_types::span::{Span, Spanned};

use crate::{
    statement::Statement,
    type_utils::{DblDot, DotDotEquals},
};

use super::{ExprWithoutBlock, Expression, RangeExpr};

pub struct RangeFromToExpr {
    from_expression: Box<dyn Expression>,
    dbl_dot: DblDot,
    to_expression_excl: Box<dyn Expression>,
}

impl Expression for RangeFromToExpr {}

impl<E> ExprWithoutBlock<E> for RangeFromToExpr {}

impl<E> RangeExpr<E> for RangeFromToExpr {}

impl Statement for RangeFromToExpr {}

impl Spanned for RangeFromToExpr {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = todo!();
        let source = self.dbl_dot.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeFromExpr {
    from_expression: Box<dyn Expression>,
    dbl_dot: DblDot,
}

impl Expression for RangeFromExpr {}

impl<E> ExprWithoutBlock<E> for RangeFromExpr {}

impl<E> RangeExpr<E> for RangeFromExpr {}

impl Statement for RangeFromExpr {}

impl Spanned for RangeFromExpr {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = self.dbl_dot.span().end();
        let source = self.dbl_dot.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeToExpr {
    dbl_dot: DblDot,
    to_expression: Box<dyn Expression>,
}

impl Expression for RangeToExpr {}

impl<E> ExprWithoutBlock<E> for RangeToExpr {}

impl<E> RangeExpr<E> for RangeToExpr {}

impl Statement for RangeToExpr {}

impl Spanned for RangeToExpr {
    fn span(&self) -> Span {
        let start_pos = self.dbl_dot.span().start();
        let end_pos = todo!();
        let source = self.dbl_dot.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeInclusiveExpr {
    from_expression: Box<dyn Expression>,
    dot_dot_equals: DotDotEquals,
    to_expression_incl: Box<dyn Expression>,
}

impl Expression for RangeInclusiveExpr {}

impl<E> ExprWithoutBlock<E> for RangeInclusiveExpr {}

impl<E> RangeExpr<E> for RangeInclusiveExpr {}

impl Statement for RangeInclusiveExpr {}

impl Spanned for RangeInclusiveExpr {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = todo!();
        let source = self.dot_dot_equals.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeToInclusiveExpr {
    dot_dot_equals: DotDotEquals,
    to_expression_incl: Box<dyn Expression>,
}

impl Expression for RangeToInclusiveExpr {}

impl<E> ExprWithoutBlock<E> for RangeToInclusiveExpr {}

impl<E> RangeExpr<E> for RangeToInclusiveExpr {}

impl Statement for RangeToInclusiveExpr {}

impl Spanned for RangeToInclusiveExpr {
    fn span(&self) -> Span {
        let start_pos = self.dot_dot_equals.span().start();
        let end_pos = todo!();
        let source = self.dot_dot_equals.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

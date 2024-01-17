use feo_types::span::{Span, Spanned};

use crate::type_utils::{DblDot, DotDotEquals};

use super::{Expression, RangeExpr};

pub struct RangeFromToExpr {
    from_expression: Box<dyn Expression>,
    dbl_dot: DblDot,
    to_expression_excl: Box<dyn Expression>,
}

impl Expression for RangeFromToExpr {}

impl<R, E> RangeExpr<R, E> for RangeFromToExpr {}

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

impl<R, E> RangeExpr<R, E> for RangeFromExpr {}

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

impl<R, E> RangeExpr<R, E> for RangeToExpr {}

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

impl<R, E> RangeExpr<R, E> for RangeInclusiveExpr {}

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

impl<R, E> RangeExpr<R, E> for RangeToInclusiveExpr {}

impl Spanned for RangeToInclusiveExpr {
    fn span(&self) -> Span {
        let start_pos = self.dot_dot_equals.span().start();
        let end_pos = todo!();
        let source = self.dot_dot_equals.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

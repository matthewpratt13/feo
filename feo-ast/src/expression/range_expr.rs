use crate::{
    span::{Span, Spanned},
    statement::Statement,
    type_utils::{DblDot, DotDotEquals},
};

use super::{BooleanOperand, Constant, ExprWithoutBlock, Expression, IterableExpr};

pub trait RangeExpr<E>
where
    Self: ExprWithoutBlock<E> + IterableExpr + BooleanOperand + Constant,
{
}

pub type RangeFullExpr = DblDot;

impl<E> RangeExpr<E> for RangeFullExpr {}

impl BooleanOperand for RangeFullExpr {}

impl IterableExpr for RangeFullExpr {}

pub struct RangeFromToExpr {
    from_operand: Box<dyn Expression>,
    dbl_dot: DblDot,
    to_operand_excl: Box<dyn Expression>,
}

impl<E> RangeExpr<E> for RangeFromToExpr {}

impl Expression for RangeFromToExpr {}

impl<E> ExprWithoutBlock<E> for RangeFromToExpr {}

impl Statement for RangeFromToExpr {}

impl BooleanOperand for RangeFromToExpr {}

impl Constant for RangeFromToExpr {}

impl IterableExpr for RangeFromToExpr {}

impl Spanned for RangeFromToExpr {
    fn span(&self) -> Span {
        let start_pos = self.from_operand.span().start();
        let end_pos = self.to_operand_excl.span().end();
        let source = self.from_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeFromExpr {
    from_operand: Box<dyn Expression>,
    dbl_dot: DblDot,
}

impl<E> RangeExpr<E> for RangeFromExpr {}

impl Expression for RangeFromExpr {}

impl<E> ExprWithoutBlock<E> for RangeFromExpr {}

impl Statement for RangeFromExpr {}

impl BooleanOperand for RangeFromExpr {}

impl Constant for RangeFromExpr {}

impl IterableExpr for RangeFromExpr {}

impl Spanned for RangeFromExpr {
    fn span(&self) -> Span {
        let start_pos = self.from_operand.span().start();
        let end_pos = self.dbl_dot.span().end();
        let source = self.from_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeToExpr {
    dbl_dot: DblDot,
    to_operand: Box<dyn Expression>,
}

impl<E> RangeExpr<E> for RangeToExpr {}

impl Expression for RangeToExpr {}

impl<E> ExprWithoutBlock<E> for RangeToExpr {}

impl Statement for RangeToExpr {}

impl BooleanOperand for RangeToExpr {}

impl Constant for RangeToExpr {}

impl IterableExpr for RangeToExpr {}

impl Spanned for RangeToExpr {
    fn span(&self) -> Span {
        let start_pos = self.dbl_dot.span().start();
        let end_pos = self.to_operand.span().end();
        let source = self.dbl_dot.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeInclusiveExpr {
    from_operand: Box<dyn Expression>,
    dot_dot_equals: DotDotEquals,
    to_operand_incl: Box<dyn Expression>,
}

impl<E> RangeExpr<E> for RangeInclusiveExpr {}

impl Expression for RangeInclusiveExpr {}

impl<E> ExprWithoutBlock<E> for RangeInclusiveExpr {}

impl Statement for RangeInclusiveExpr {}

impl BooleanOperand for RangeInclusiveExpr {}

impl Constant for RangeInclusiveExpr {}

impl IterableExpr for RangeInclusiveExpr {}

impl Spanned for RangeInclusiveExpr {
    fn span(&self) -> Span {
        let start_pos = self.from_operand.span().start();
        let end_pos = self.to_operand_incl.span().end();
        let source = self.from_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RangeToInclusiveExpr {
    dot_dot_equals: DotDotEquals,
    to_operand_incl: Box<dyn Expression>,
}

impl<E> RangeExpr<E> for RangeToInclusiveExpr {}

impl Expression for RangeToInclusiveExpr {}

impl<E> ExprWithoutBlock<E> for RangeToInclusiveExpr {}

impl Statement for RangeToInclusiveExpr {}

impl BooleanOperand for RangeToInclusiveExpr {}

impl Constant for RangeToInclusiveExpr {}

impl IterableExpr for RangeToInclusiveExpr {}

impl Spanned for RangeToInclusiveExpr {
    fn span(&self) -> Span {
        let start_pos = self.dot_dot_equals.span().start();
        let end_pos = self.to_operand_incl.span().end();
        let source = self.dot_dot_equals.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

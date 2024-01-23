use feo_types::span::{Span, Spanned};

use crate::{
    keyword::Keyword,
    statement::Statement,
    ty::Type,
    type_utils::{BangOrMinus, Equals, OpArithmeticOrLogical, OpBool, OpComparison, QuestionMark},
};

use super::{Constant, ExprWithoutBlock, Expression};

pub trait OperatorExpr<E>
where
    Self: Sized + ExprWithoutBlock<E>,
{
}

pub struct ArithmeticOrLogicalExpr {
    first_operand: Box<dyn Expression>,
    operator: OpArithmeticOrLogical,
    second_operand: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for ArithmeticOrLogicalExpr {}

impl Expression for ArithmeticOrLogicalExpr {}

impl<E> ExprWithoutBlock<E> for ArithmeticOrLogicalExpr {}

impl Statement for ArithmeticOrLogicalExpr {}

impl Constant for ArithmeticOrLogicalExpr {}

impl Spanned for ArithmeticOrLogicalExpr {
    fn span(&self) -> Span {
        let start_pos = self.first_operand.span().start();
        let end_pos = self.second_operand.span().end();
        let source = self.first_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct AssignmentExpr {
    initial_value: Box<dyn Expression>,
    equals: Equals,
    new_value: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for AssignmentExpr {}

impl Expression for AssignmentExpr {}

impl<E> ExprWithoutBlock<E> for AssignmentExpr {}

impl Statement for AssignmentExpr {}

impl Constant for AssignmentExpr {}

impl Spanned for AssignmentExpr {
    fn span(&self) -> Span {
        let start_pos = self.initial_value.span().start();
        let end_pos = self.new_value.span().end();
        let source = self.initial_value.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct BoolExpr {
    first_expression: Box<dyn Expression>,
    operator: OpBool,
    second_expression: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for BoolExpr {}

impl Expression for BoolExpr {}

impl<E> ExprWithoutBlock<E> for BoolExpr {}

impl Statement for BoolExpr {}

impl Constant for BoolExpr {}

impl Spanned for BoolExpr {
    fn span(&self) -> Span {
        let start_pos = self.first_expression.span().start();
        let end_pos = self.second_expression.span().end();
        let source = self.first_expression.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct ComparisonExpr {
    first_expression: Box<dyn Expression>,
    operator: OpComparison,
    second_expression: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for ComparisonExpr {}

impl Expression for ComparisonExpr {}

impl<E> ExprWithoutBlock<E> for ComparisonExpr {}

impl Statement for ComparisonExpr {}

impl Constant for ComparisonExpr {}

impl Spanned for ComparisonExpr {
    fn span(&self) -> Span {
        let start_pos = self.first_expression.span().start();
        let end_pos = self.second_expression.span().end();
        let source = self.first_expression.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct DerefExpr {
    kw_deref: Keyword,
    expression: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for DerefExpr {}

impl Expression for DerefExpr {}

impl<E> ExprWithoutBlock<E> for DerefExpr {}

impl Statement for DerefExpr {}

impl Constant for DerefExpr {}

impl Spanned for DerefExpr {
    fn span(&self) -> Span {
        let start_pos = self.kw_deref.span().start();
        let end_pos = self.expression.span().end();
        let source = self.kw_deref.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct NegationExpr {
    negator: BangOrMinus,
    expression: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for NegationExpr {}

impl Expression for NegationExpr {}

impl<E> ExprWithoutBlock<E> for NegationExpr {}

impl Statement for NegationExpr {}

impl Constant for NegationExpr {}

impl Spanned for NegationExpr {
    fn span(&self) -> Span {
        let start_pos = self.negator.span().start();
        let end_pos = self.expression.span().end();
        let source = self.negator.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RefExpr {
    kw_ref: Keyword,
    kw_mut_opt: Option<Keyword>,
    expression: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for RefExpr {}

impl Expression for RefExpr {}

impl<E> ExprWithoutBlock<E> for RefExpr {}

impl Statement for RefExpr {}

impl Spanned for RefExpr {
    fn span(&self) -> Span {
        let start_pos = self.kw_ref.span().start();
        let end_pos = self.expression.span().end();
        let source = self.kw_ref.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct ResultUnwrapExpr {
    expression: Box<dyn Expression>, // can only be applied to `Option` and `Result`
    question_mark: QuestionMark,
}

impl<E> OperatorExpr<E> for ResultUnwrapExpr {}

impl Expression for ResultUnwrapExpr {}

impl<E> ExprWithoutBlock<E> for ResultUnwrapExpr {}

impl Statement for ResultUnwrapExpr {}

impl Spanned for ResultUnwrapExpr {
    fn span(&self) -> Span {
        let start_pos = self.expression.span().start();
        let end_pos = self.question_mark.span().end();
        let source = self.expression.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TypeCastExpr {
    original_expression: Box<dyn Expression>,
    kw_as: Keyword,
    new_type: Box<dyn Type>, // cannot be a trait object
}

impl<E> OperatorExpr<E> for TypeCastExpr {}

impl Expression for TypeCastExpr {}

impl<E> ExprWithoutBlock<E> for TypeCastExpr {}

impl Statement for TypeCastExpr {}

impl Constant for TypeCastExpr {}

impl Spanned for TypeCastExpr {
    fn span(&self) -> Span {
        let start_pos = self.original_expression.span().start();
        let end_pos = self.new_type.span().end();
        let source = self.original_expression.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

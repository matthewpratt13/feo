use feo_types::span::{Span, Spanned};

use crate::{
    keyword::Keyword,
    statement::Statement,
    ty::Type,
    type_utils::{Bang, Equals, OpArithmeticOrLogical, OpBool, OpComparison, QuestionMark},
};

use super::{ExprWithoutBlock, Expression, OperatorExpr};

pub struct ArithmeticOrLogicalExpr {
    first_operand: Box<dyn Expression>,
    operator: OpArithmeticOrLogical,
    second_operand: Box<dyn Expression>,
}

impl Expression for ArithmeticOrLogicalExpr {}

impl<E> ExprWithoutBlock<E> for ArithmeticOrLogicalExpr {}

impl<E> OperatorExpr<E> for ArithmeticOrLogicalExpr {}

impl Statement for ArithmeticOrLogicalExpr {}

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

impl Expression for AssignmentExpr {}

impl<E> ExprWithoutBlock<E> for AssignmentExpr {}

impl<E> OperatorExpr<E> for AssignmentExpr {}

impl Statement for AssignmentExpr {}

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

impl Expression for BoolExpr {}

impl<E> ExprWithoutBlock<E> for BoolExpr {}

impl<E> OperatorExpr<E> for BoolExpr {}

impl Statement for BoolExpr {}

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

impl Expression for ComparisonExpr {}

impl<E> ExprWithoutBlock<E> for ComparisonExpr {}

impl<E> OperatorExpr<E> for ComparisonExpr {}

impl Statement for ComparisonExpr {}

impl Spanned for ComparisonExpr {
    fn span(&self) -> Span {
        let start_pos = self.first_expression.span().start();
        let end_pos = self.second_expression.span().end();
        let source = self.first_expression.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct NegationExpr {
    negator: Bang,
    expression: Box<dyn Expression>,
}

impl Expression for NegationExpr {}

impl<E> ExprWithoutBlock<E> for NegationExpr {}

impl<E> OperatorExpr<E> for NegationExpr {}

impl Statement for NegationExpr {}

impl Spanned for NegationExpr {
    fn span(&self) -> Span {
        let start_pos = self.negator.span().start();
        let end_pos = self.expression.span().end();
        let source = self.negator.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct ResultUnwrapExpr {
    expression: Box<dyn Expression>,
    question_mark: QuestionMark,
}

impl Expression for ResultUnwrapExpr {}

impl<E> ExprWithoutBlock<E> for ResultUnwrapExpr {}

impl<E> OperatorExpr<E> for ResultUnwrapExpr {}

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

impl Expression for TypeCastExpr {}

impl<E> ExprWithoutBlock<E> for TypeCastExpr {}

impl<E> OperatorExpr<E> for TypeCastExpr {}

impl Statement for TypeCastExpr {}

impl Spanned for TypeCastExpr {
    fn span(&self) -> Span {
        let start_pos = self.original_expression.span().start();
        let end_pos = self.new_type.span().end();
        let source = self.original_expression.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

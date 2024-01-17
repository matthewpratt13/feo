use feo_types::span::{Span, Spanned};

use crate::{
    keyword::Keyword,
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

impl OperatorExpr for ArithmeticOrLogicalExpr {}

impl Spanned for ArithmeticOrLogicalExpr {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = todo!();
        let source = self.operator.span().source();

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

impl OperatorExpr for AssignmentExpr {}

impl Spanned for AssignmentExpr {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = todo!();
        let source = self.equals.span().source();

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

impl OperatorExpr for BoolExpr {}

impl Spanned for BoolExpr {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = todo!();
        let source = self.operator.span().source();

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

impl OperatorExpr for ComparisonExpr {}

impl Spanned for ComparisonExpr {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = todo!();
        let source = self.operator.span().source();

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

impl OperatorExpr for NegationExpr {}

impl Spanned for NegationExpr {
    fn span(&self) -> Span {
        let start_pos = self.negator.span().start();
        let end_pos = todo!();
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

impl OperatorExpr for ResultUnwrapExpr {}

impl Spanned for ResultUnwrapExpr {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = self.question_mark.span().end();
        let source = self.question_mark.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TypeCastExpr {
    original_expression: Box<dyn Expression>,
    kw_as: Keyword,
    new_type: Box<dyn Type>, // cannot be trait object
}

impl Expression for TypeCastExpr {}

impl<E> ExprWithoutBlock<E> for TypeCastExpr {}

impl OperatorExpr for TypeCastExpr {}

impl Spanned for TypeCastExpr {
    fn span(&self) -> Span {
        let start_pos = todo!();
        let end_pos = todo!();
        let source = self.kw_as.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

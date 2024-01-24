use crate::{
    keyword::Keyword,
    span::{Span, Spanned},
    statement::Statement,
    ty::Type,
    type_utils::{
        ArithmeticPuncEquals, BangOrMinus, Equals, OpArithmeticOrLogical, OpBool, OpComparison,
        QuestionMark,
    },
};

use super::{AssignableExpr, Constant, ExprWithoutBlock, Expression};

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
    first_operand: Box<dyn Expression>,
    equals: Equals,
    second_operand: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for AssignmentExpr {}

impl Expression for AssignmentExpr {}

impl<E> ExprWithoutBlock<E> for AssignmentExpr {}

impl Statement for AssignmentExpr {}

impl Constant for AssignmentExpr {}

impl Spanned for AssignmentExpr {
    fn span(&self) -> Span {
        let start_pos = self.first_operand.span().start();
        let end_pos = self.second_operand.span().end();
        let source = self.first_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct CompoundAssignmentExpr {
    first_operand: Box<dyn AssignableExpr>,
    arithmetic_punc_equals: ArithmeticPuncEquals,
    second_operand: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for CompoundAssignmentExpr {}

impl Expression for CompoundAssignmentExpr {}

impl<E> ExprWithoutBlock<E> for CompoundAssignmentExpr {}

impl Statement for CompoundAssignmentExpr {}

impl Constant for CompoundAssignmentExpr {}

impl Spanned for CompoundAssignmentExpr {
    fn span(&self) -> Span {
        let start_pos = self.first_operand.span().start();
        let end_pos = self.second_operand.span().end();
        let source = self.first_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct BoolExpr {
    first_operand: Box<dyn Expression>,
    operator: OpBool,
    second_operand: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for BoolExpr {}

impl Expression for BoolExpr {}

impl<E> ExprWithoutBlock<E> for BoolExpr {}

impl Statement for BoolExpr {}

impl Constant for BoolExpr {}

impl Spanned for BoolExpr {
    fn span(&self) -> Span {
        let start_pos = self.first_operand.span().start();
        let end_pos = self.second_operand.span().end();
        let source = self.first_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct ComparisonExpr {
    first_operand: Box<dyn Expression>,
    operator: OpComparison,
    second_operand: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for ComparisonExpr {}

impl Expression for ComparisonExpr {}

impl<E> ExprWithoutBlock<E> for ComparisonExpr {}

impl Statement for ComparisonExpr {}

impl Constant for ComparisonExpr {}

impl Spanned for ComparisonExpr {
    fn span(&self) -> Span {
        let start_pos = self.first_operand.span().start();
        let end_pos = self.second_operand.span().end();
        let source = self.first_operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct DerefExpr {
    kw_deref: Keyword,
    operand: Box<dyn AssignableExpr>,
}

impl<E> OperatorExpr<E> for DerefExpr {}

impl Expression for DerefExpr {}

impl<E> ExprWithoutBlock<E> for DerefExpr {}

impl Statement for DerefExpr {}

impl Constant for DerefExpr {}

impl Spanned for DerefExpr {
    fn span(&self) -> Span {
        let start_pos = self.kw_deref.span().start();
        let end_pos = self.operand.span().end();
        let source = self.kw_deref.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct NegationExpr {
    negator: BangOrMinus,
    operand: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for NegationExpr {}

impl Expression for NegationExpr {}

impl<E> ExprWithoutBlock<E> for NegationExpr {}

impl Statement for NegationExpr {}

impl Constant for NegationExpr {}

impl Spanned for NegationExpr {
    fn span(&self) -> Span {
        let start_pos = self.negator.span().start();
        let end_pos = self.operand.span().end();
        let source = self.negator.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RefExpr {
    kw_ref: Keyword,
    kw_mut_opt: Option<Keyword>,
    operand: Box<dyn AssignableExpr>,
}

impl<E> OperatorExpr<E> for RefExpr {}

impl Expression for RefExpr {}

impl<E> ExprWithoutBlock<E> for RefExpr {}

impl Statement for RefExpr {}

impl Spanned for RefExpr {
    fn span(&self) -> Span {
        let start_pos = self.kw_ref.span().start();
        let end_pos = self.operand.span().end();
        let source = self.kw_ref.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct UnwrapExpr {
    operand: Box<dyn Expression>, // can only be applied to `Option` and `Result`
    question_mark: QuestionMark,
}

impl<E> OperatorExpr<E> for UnwrapExpr {}

impl Expression for UnwrapExpr {}

impl<E> ExprWithoutBlock<E> for UnwrapExpr {}

impl Statement for UnwrapExpr {}

impl Spanned for UnwrapExpr {
    fn span(&self) -> Span {
        let start_pos = self.operand.span().start();
        let end_pos = self.question_mark.span().end();
        let source = self.operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TypeCastExpr {
    operand: Box<dyn Expression>,
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
        let start_pos = self.operand.span().start();
        let end_pos = self.new_type.span().end();
        let source = self.operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

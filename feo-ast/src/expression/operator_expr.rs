use feo_error::error::CompilerError;

use crate::{
    keyword::Keyword,
    punctuation::Punctuation,
    span::{Span, Spanned},
    statement::Statement,
    type_utils::{Bang, Equals, Minus, QuestionMark},
};

use super::{
    AssignableExpr, BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr,
};

pub trait OperatorExpr<E>
where
    Self: ExprWithoutBlock<E> + IterableExpr + BooleanOperand,
{
}

pub enum ArithmeticOrLogicalOperatorKind {
    Plus(Punctuation),
    Minus(Punctuation),
    Multiply(Punctuation),
    Divide(Punctuation),
    Modulus(Punctuation),
    LogicalAnd(Punctuation),
    LogicalOr(Punctuation),
}

pub enum ComparisonOperatorKind {
    Equality(Punctuation),
    NotEqual(Punctuation),
    LessThan(Punctuation),
    GreaterThan(Punctuation),
    LessThanOrEqual(Punctuation),
    GreaterThanOrEqual(Punctuation),
}

pub enum CompoundAssignmentOperatorKind {
    PlusEquals(Punctuation),
    MinusEquals(Punctuation),
    MultiplyEquals(Punctuation),
    DivideEquals(Punctuation),
    ModulusEquals(Punctuation),
}

pub enum LazyBoolOperatorKind {
    And(Punctuation),
    Or(Punctuation),
}

pub enum NegationOperatorKind {
    InvertNumeric(Minus),
    InvertBool(Bang),
}

impl Spanned for NegationOperatorKind {
    fn span(&self) -> Span {
        match self {
            NegationOperatorKind::InvertNumeric(n) => n.span(),
            NegationOperatorKind::InvertBool(b) => b.span(),
        }
    }
}

pub enum UnwrapOperationKind<T: Spanned> {
    Option(Option<T>),
    Result(Result<T, CompilerError>),
}

impl<T> Spanned for UnwrapOperationKind<T>
where
    T: Spanned,
{
    fn span(&self) -> Span {
        match self {
            UnwrapOperationKind::Option(o) => {
                if let Some(t) = o {
                    t.span()
                } else {
                    Span::default()
                }
            }
            UnwrapOperationKind::Result(r) => {
                if let Ok(t) = r {
                    t.span()
                } else {
                    Span::default()
                }
            }
        }
    }
}

pub type CastOperator = Keyword;
pub type DerefOperator = Keyword;
pub type RefOperator = Keyword;

pub struct ArithmeticOrLogicalExpr {
    lhs: Box<dyn Expression>,
    operator: ArithmeticOrLogicalOperatorKind,
    rhs: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for ArithmeticOrLogicalExpr {}

impl Expression for ArithmeticOrLogicalExpr {}

impl<E> ExprWithoutBlock<E> for ArithmeticOrLogicalExpr {}

impl Statement for ArithmeticOrLogicalExpr {}

impl BooleanOperand for ArithmeticOrLogicalExpr {}

impl Constant for ArithmeticOrLogicalExpr {}

impl IterableExpr for ArithmeticOrLogicalExpr {}

impl Spanned for ArithmeticOrLogicalExpr {
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct AssignmentExpr {
    assignee: Box<dyn Expression>,
    equals: Equals,
    new_value: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for AssignmentExpr {}

impl Expression for AssignmentExpr {}

impl<E> ExprWithoutBlock<E> for AssignmentExpr {}

impl Statement for AssignmentExpr {}

impl BooleanOperand for AssignmentExpr {}

impl Constant for AssignmentExpr {}

impl IterableExpr for AssignmentExpr {}

impl Spanned for AssignmentExpr {
    fn span(&self) -> Span {
        let start_pos = self.assignee.span().start();
        let end_pos = self.new_value.span().end();
        let source = self.assignee.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct CompoundAssignmentExpr {
    assignee: Box<dyn AssignableExpr>,
    operator: CompoundAssignmentOperatorKind,
    new_value: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for CompoundAssignmentExpr {}

impl Expression for CompoundAssignmentExpr {}

impl<E> ExprWithoutBlock<E> for CompoundAssignmentExpr {}

impl Statement for CompoundAssignmentExpr {}

impl BooleanOperand for CompoundAssignmentExpr {}

impl Constant for CompoundAssignmentExpr {}

impl IterableExpr for CompoundAssignmentExpr {}

impl Spanned for CompoundAssignmentExpr {
    fn span(&self) -> Span {
        let start_pos = self.assignee.span().start();
        let end_pos = self.new_value.span().end();
        let source = self.assignee.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct ComparisonExpr {
    lhs: Box<dyn Expression>,
    operator: ComparisonOperatorKind,
    rhs: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for ComparisonExpr {}

impl Expression for ComparisonExpr {}

impl<E> ExprWithoutBlock<E> for ComparisonExpr {}

impl Statement for ComparisonExpr {}

impl BooleanOperand for ComparisonExpr {}

impl Constant for ComparisonExpr {}

impl IterableExpr for ComparisonExpr {}

impl Spanned for ComparisonExpr {
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct DerefExpr {
    kw_deref: DerefOperator,
    operand: Box<dyn AssignableExpr>,
}

impl<E> OperatorExpr<E> for DerefExpr {}

impl Expression for DerefExpr {}

impl<E> ExprWithoutBlock<E> for DerefExpr {}

impl Statement for DerefExpr {}

impl BooleanOperand for DerefExpr {}

impl Constant for DerefExpr {}

impl IterableExpr for DerefExpr {}

impl Spanned for DerefExpr {
    fn span(&self) -> Span {
        let start_pos = self.kw_deref.span().start();
        let end_pos = self.operand.span().end();
        let source = self.kw_deref.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct LazyBoolExpr {
    lhs: Box<dyn Expression>,
    operator: LazyBoolOperatorKind,
    rhs: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for LazyBoolExpr {}

impl Expression for LazyBoolExpr {}

impl<E> ExprWithoutBlock<E> for LazyBoolExpr {}

impl Statement for LazyBoolExpr {}

impl BooleanOperand for LazyBoolExpr {}

impl Constant for LazyBoolExpr {}

impl IterableExpr for LazyBoolExpr {}

impl Spanned for LazyBoolExpr {
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct NegationExpr {
    negator: NegationOperatorKind,
    operand: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for NegationExpr {}

impl Expression for NegationExpr {}

impl<E> ExprWithoutBlock<E> for NegationExpr {}

impl Statement for NegationExpr {}

impl BooleanOperand for NegationExpr {}

impl Constant for NegationExpr {}

impl IterableExpr for NegationExpr {}

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
    kw_ref: RefOperator,
    kw_mut_opt: Option<Keyword>,
    operand: Box<dyn AssignableExpr>,
}

impl<E> OperatorExpr<E> for RefExpr {}

impl Expression for RefExpr {}

impl<E> ExprWithoutBlock<E> for RefExpr {}

impl Statement for RefExpr {}

impl BooleanOperand for RefExpr {}

impl IterableExpr for RefExpr {}

impl Spanned for RefExpr {
    fn span(&self) -> Span {
        let start_pos = self.kw_ref.span().start();
        let end_pos = self.operand.span().end();
        let source = self.kw_ref.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TypeCastExpr {
    lhs: Box<dyn Castable>,
    kw_as: CastOperator,
    rhs: Box<dyn Castable>,
}

impl<E> OperatorExpr<E> for TypeCastExpr {}

impl Expression for TypeCastExpr {}

impl<E> ExprWithoutBlock<E> for TypeCastExpr {}

impl Statement for TypeCastExpr {}

impl BooleanOperand for TypeCastExpr {}

impl Constant for TypeCastExpr {}

impl IterableExpr for TypeCastExpr {}

impl Spanned for TypeCastExpr {
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct UnwrapExpr<T: Spanned> {
    operand: UnwrapOperationKind<T>,
    question_mark: QuestionMark,
}

impl<T, E> OperatorExpr<E> for UnwrapExpr<T> where T: Spanned + 'static {}

impl<T> Expression for UnwrapExpr<T> where T: Spanned {}

impl<T, E> ExprWithoutBlock<E> for UnwrapExpr<T> where T: Spanned {}

impl<T> Statement for UnwrapExpr<T> where T: Spanned {}

impl<T> BooleanOperand for UnwrapExpr<T> where T: 'static + Spanned {}

impl<T> IterableExpr for UnwrapExpr<T> where T: 'static + Spanned {}

impl<T> Spanned for UnwrapExpr<T>
where
    T: Spanned,
{
    fn span(&self) -> Span {
        let start_pos = self.operand.span().start();
        let end_pos = self.question_mark.span().end();
        let source = self.operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

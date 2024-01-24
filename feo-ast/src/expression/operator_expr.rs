use feo_error::error::CompilerError;

use crate::{
    keyword::Keyword,
    punctuation::Punctuation,
    span::{Span, Spanned},
    statement::Statement,
    ty::Type,
    type_utils::{Bang, Equals, Minus, QuestionMark},
};

use super::{AssignableExpr, CastableExpr, Constant, ExprWithoutBlock, Expression};

pub trait OperatorExpr<E>
where
    Self: ExprWithoutBlock<E>,
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

pub struct ArithmeticOrLogicalExpr {
    first_operand: Box<dyn Expression>,
    operator: ArithmeticOrLogicalOperatorKind,
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
    operator: CompoundAssignmentOperatorKind,
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

pub struct ComparisonExpr {
    first_operand: Box<dyn Expression>,
    operator: ComparisonOperatorKind,
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

pub struct LazyBoolExpr {
    first_operand: Box<dyn Expression>,
    operator: LazyBoolOperatorKind,
    second_operand: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for LazyBoolExpr {}

impl Expression for LazyBoolExpr {}

impl<E> ExprWithoutBlock<E> for LazyBoolExpr {}

impl Statement for LazyBoolExpr {}

impl Constant for LazyBoolExpr {}

impl Spanned for LazyBoolExpr {
    fn span(&self) -> Span {
        let start_pos = self.first_operand.span().start();
        let end_pos = self.second_operand.span().end();
        let source = self.first_operand.span().source();

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

pub struct UnwrapExpr<T: Spanned> {
    operand: UnwrapOperationKind<T>,
    question_mark: QuestionMark,
}

impl<T, E> OperatorExpr<E> for UnwrapExpr<T> where T: Spanned {}

impl<T> Expression for UnwrapExpr<T> where T: Spanned {}

impl<T, E> ExprWithoutBlock<E> for UnwrapExpr<T> where T: Spanned {}

impl<T> Statement for UnwrapExpr<T> where T: Spanned {}

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

pub struct TypeCastExpr {
    operand: Box<dyn CastableExpr>,
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

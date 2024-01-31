use feo_error::error::CompilerError;

use feo_types::{
    span::{Span, Spanned},
    utils::{Bang, Equals, Minus, QuestionMark},
    Keyword, Punctuation,
};

// TODO: start using `Span::join()` from here

use crate::statement::Statement;

use super::{
    Assignable, BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr,
};

pub trait OperatorExpr<E>
where
    Self: ExprWithoutBlock<E> + BooleanOperand + IterableExpr,
{
}

pub enum ArithmeticOrLogicalOpKind {
    Plus(Punctuation),
    Minus(Punctuation),
    Multiply(Punctuation),
    Divide(Punctuation),
    Modulus(Punctuation),
    LogicalAnd(Punctuation),
    LogicalOr(Punctuation),
}

pub enum ComparisonOpKind {
    Equality(Punctuation),
    NotEqual(Punctuation),
    LessThan(Punctuation),
    GreaterThan(Punctuation),
    LessThanOrEqual(Punctuation),
    GreaterThanOrEqual(Punctuation),
}

pub enum CompoundAssignOpKind {
    PlusAssign(Punctuation),
    MinusAssign(Punctuation),
    MultiplyAssign(Punctuation),
    DivideAssign(Punctuation),
    ModulusAssign(Punctuation),
}

pub enum LazyBoolOpKind {
    LazyAnd(Punctuation),
    LazyOr(Punctuation),
}

pub enum NegationOpKind {
    InvertNumeric(Minus),
    InvertBool(Bang),
}

impl Spanned for NegationOpKind {
    fn span(&self) -> Span {
        match self {
            NegationOpKind::InvertNumeric(n) => n.span(),
            NegationOpKind::InvertBool(b) => b.span(),
        }
    }
}

pub enum UnwrapOpKind<T: Spanned> {
    Option(Option<T>),
    Result(Result<T, CompilerError>),
}

impl<T> Spanned for UnwrapOpKind<T>
where
    T: Spanned,
{
    fn span(&self) -> Span {
        match self {
            UnwrapOpKind::Option(o) => {
                if let Some(t) = o {
                    t.span()
                } else {
                    Span::default()
                }
            }
            UnwrapOpKind::Result(r) => {
                if let Ok(t) = r {
                    t.span()
                } else {
                    Span::default()
                }
            }
        }
    }
}

pub type CastOperator = Keyword; // `as`
pub type DerefOperator = Keyword; // `deref`
pub type RefOperator = Keyword; // `ref`

pub struct ArithmeticOrLogicalExpr {
    lhs: Box<dyn Expression>,
    operator: ArithmeticOrLogicalOpKind,
    rhs: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for ArithmeticOrLogicalExpr {}

impl Expression for ArithmeticOrLogicalExpr {}

impl<E> ExprWithoutBlock<E> for ArithmeticOrLogicalExpr {}

impl Statement for ArithmeticOrLogicalExpr {}

impl BooleanOperand for ArithmeticOrLogicalExpr {}

impl IterableExpr for ArithmeticOrLogicalExpr {}

impl Constant for ArithmeticOrLogicalExpr {}

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
    assignee: Box<dyn Assignable>,
    operator: CompoundAssignOpKind,
    new_value: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for CompoundAssignmentExpr {}

impl Expression for CompoundAssignmentExpr {}

impl<E> ExprWithoutBlock<E> for CompoundAssignmentExpr {}

impl Statement for CompoundAssignmentExpr {}

impl BooleanOperand for CompoundAssignmentExpr {}

impl IterableExpr for CompoundAssignmentExpr {}

impl Constant for CompoundAssignmentExpr {}

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
    operator: ComparisonOpKind,
    rhs: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for ComparisonExpr {}

impl Expression for ComparisonExpr {}

impl<E> ExprWithoutBlock<E> for ComparisonExpr {}

impl Statement for ComparisonExpr {}

impl BooleanOperand for ComparisonExpr {}

impl IterableExpr for ComparisonExpr {}

impl Constant for ComparisonExpr {}

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
    operand: Box<dyn Assignable>,
}

impl<E> OperatorExpr<E> for DerefExpr {}

impl Expression for DerefExpr {}

impl<E> ExprWithoutBlock<E> for DerefExpr {}

impl Statement for DerefExpr {}

impl BooleanOperand for DerefExpr {}

impl IterableExpr for DerefExpr {}

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
    lhs: Box<dyn Expression>,
    operator: LazyBoolOpKind,
    rhs: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for LazyBoolExpr {}

impl Expression for LazyBoolExpr {}

impl<E> ExprWithoutBlock<E> for LazyBoolExpr {}

impl Statement for LazyBoolExpr {}

impl BooleanOperand for LazyBoolExpr {}

impl IterableExpr for LazyBoolExpr {}

impl Constant for LazyBoolExpr {}

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
    negator: NegationOpKind,
    operand: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for NegationExpr {}

impl Expression for NegationExpr {}

impl<E> ExprWithoutBlock<E> for NegationExpr {}

impl Statement for NegationExpr {}

impl BooleanOperand for NegationExpr {}

impl IterableExpr for NegationExpr {}

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
    kw_ref: RefOperator,
    kw_mut_opt: Option<Keyword>,
    operand: Box<dyn Assignable>,
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

impl IterableExpr for TypeCastExpr {}

impl Constant for TypeCastExpr {}

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
    operand: UnwrapOpKind<T>,
    question_mark: QuestionMark,
}

impl<T, E> OperatorExpr<E> for UnwrapExpr<T> where T: Spanned + 'static {}

impl<T> Expression for UnwrapExpr<T> where T: Spanned {}

impl<T, E> ExprWithoutBlock<E> for UnwrapExpr<T> where T: Spanned {}

impl<T> Statement for UnwrapExpr<T> where T: Spanned {}

impl<T> BooleanOperand for UnwrapExpr<T> where T: Spanned + 'static {}

impl<T> IterableExpr for UnwrapExpr<T> where T: Spanned + 'static {}

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

impl<E> OperatorExpr<E> for Keyword {} // `ref`, `ref mut`, `deref`, `as`

impl<E> OperatorExpr<E> for Punctuation {}

use feo_error::error::CompilerError;

use feo_types::{
    span::{Span, Spanned},
    utils::{
        Ampersand, Asterisk, AsteriskEquals, Bang, BangEquals, DblAmpersand, DblEquals, DblPipe,
        Equals, ForwardSlash, ForwardSlashEquals, GreaterThan, GreaterThanEquals, KwAs, KwDeref,
        KwMut, KwRef, LessThan, LessThanEquals, Minus, MinusEquals, Percent, PercentEquals, Pipe,
        Plus, PlusEquals, QuestionMark,
    },
};

// TODO: start using `Span::join()` from here

use super::{
    Assignable, BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr,
};

pub trait OperatorExpr<E>
where
    Self: ExprWithoutBlock<E> + BooleanOperand + IterableExpr,
{
}

pub enum ArithmeticOrLogicalOperatorKind {
    Plus(Plus),
    Minus(Minus),
    Multiply(Asterisk),
    Divide(ForwardSlash),
    Modulus(Percent),
    LogicalAnd(Ampersand),
    LogicalOr(Pipe),
}

pub enum ComparisonOperatorKind {
    Equality(DblEquals),
    NotEqual(BangEquals),
    LessThan(LessThan),
    GreaterThan(GreaterThan),
    LessThanOrEqual(LessThanEquals),
    GreaterThanOrEqual(GreaterThanEquals),
}

pub enum CompoundAssignOperatorKind {
    PlusAssign(PlusEquals),
    MinusAssign(MinusEquals),
    MultiplyAssign(AsteriskEquals),
    DivideAssign(ForwardSlashEquals),
    ModulusAssign(PercentEquals),
}

pub enum LazyBoolOperatorKind {
    LazyAnd(DblAmpersand),
    LazyOr(DblPipe),
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

pub enum UnwrapOperatorKind<T: Spanned> {
    Option(Option<T>),
    Result(Result<T, CompilerError>),
}

impl<T> Spanned for UnwrapOperatorKind<T>
where
    T: Spanned,
{
    fn span(&self) -> Span {
        match self {
            UnwrapOperatorKind::Option(o) => {
                if let Some(t) = o {
                    t.span()
                } else {
                    Span::default()
                }
            }
            UnwrapOperatorKind::Result(r) => {
                if let Ok(t) = r {
                    t.span()
                } else {
                    Span::default()
                }
            }
        }
    }
}

pub type AssignOperator = Equals;
pub type CastOperator = KwAs;
pub type DerefOperator = KwDeref;
pub type RefOperator = (Option<KwMut>, KwRef);

pub struct ArithmeticOrLogicalExpr {
    lhs: Box<dyn Expression>,
    operator: ArithmeticOrLogicalOperatorKind,
    rhs: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for ArithmeticOrLogicalExpr {}

impl Expression for ArithmeticOrLogicalExpr {}

impl<E> ExprWithoutBlock<E> for ArithmeticOrLogicalExpr {}

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
    operator: AssignOperator,
    new_value: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for AssignmentExpr {}

impl Expression for AssignmentExpr {}

impl<E> ExprWithoutBlock<E> for AssignmentExpr {}

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
    operator: CompoundAssignOperatorKind,
    new_value: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for CompoundAssignmentExpr {}

impl Expression for CompoundAssignmentExpr {}

impl<E> ExprWithoutBlock<E> for CompoundAssignmentExpr {}

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
    operator: ComparisonOperatorKind,
    rhs: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for ComparisonExpr {}

impl Expression for ComparisonExpr {}

impl<E> ExprWithoutBlock<E> for ComparisonExpr {}

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
    deref_operator: DerefOperator,
    operand: Box<dyn Assignable>,
}

impl<E> OperatorExpr<E> for DerefExpr {}

impl Expression for DerefExpr {}

impl<E> ExprWithoutBlock<E> for DerefExpr {}

impl BooleanOperand for DerefExpr {}

impl IterableExpr for DerefExpr {}

impl Constant for DerefExpr {}

impl Spanned for DerefExpr {
    fn span(&self) -> Span {
        let start_pos = self.deref_operator.span().start();
        let end_pos = self.operand.span().end();
        let source = self.deref_operator.span().source();

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
    negator: NegationOperatorKind,
    operand: Box<dyn Expression>,
}

impl<E> OperatorExpr<E> for NegationExpr {}

impl Expression for NegationExpr {}

impl<E> ExprWithoutBlock<E> for NegationExpr {}

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
    ref_operator: RefOperator,
    operand: Box<dyn Assignable>,
}

impl<E> OperatorExpr<E> for RefExpr {}

impl Expression for RefExpr {}

impl<E> ExprWithoutBlock<E> for RefExpr {}

impl BooleanOperand for RefExpr {}

impl IterableExpr for RefExpr {}

impl Spanned for RefExpr {
    fn span(&self) -> Span {
        let s1 = if let Some(r) = &self.ref_operator.0 {
            r.span()
        } else {
            self.ref_operator.1.span()
        };

        let s2 = self.operand.span();

        Span::join(s1, s2)
    }
}

pub struct TypeCastExpr {
    lhs: Box<dyn Castable>,
    cast_operator: CastOperator,
    rhs: Box<dyn Castable>,
}

impl<E> OperatorExpr<E> for TypeCastExpr {}

impl Expression for TypeCastExpr {}

impl<E> ExprWithoutBlock<E> for TypeCastExpr {}

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
    operand: UnwrapOperatorKind<T>,
    question_mark: QuestionMark,
}

impl<T, E> OperatorExpr<E> for UnwrapExpr<T> where T: Spanned + 'static {}

impl<T> Expression for UnwrapExpr<T> where T: Spanned {}

impl<T, E> ExprWithoutBlock<E> for UnwrapExpr<T> where T: Spanned {}

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

use feo_error::error::CompilerError;

use feo_types::{
    span::{Span, Spanned},
    utils::{
        Ampersand, Asterisk, AsteriskEquals, Bang, BangEquals, Caret, DblAmpersand, DblEquals,
        DblGreaterThan, DblLessThan, DblPipe, Equals, ForwardSlash, ForwardSlashEquals,
        GreaterThan, GreaterThanEquals, KwAs, KwMut, LessThan, LessThanEquals, Minus, MinusEquals,
        Percent, PercentEquals, Pipe, Plus, PlusEquals, QuestionMark,
    },
};

// TODO: start using `Span::join()` from here

use crate::statement::Statement;

use super::{
    Assignable, BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr,
};

pub trait OperatorExpr
where
    Self: ExprWithoutBlock + BooleanOperand + IterableExpr + ExprWithoutBlock,
{
}

pub enum OperationExprKind<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    ArithmeticOrLogical(ArithmeticOrLogicalExpr<A, B, C, E, I, S, U>),
    Comparison(ComparisonExpr<A, B, C, E, I, S, U>),
    CompoundAssign(CompoundAssignmentExpr<A, B, C, E, I, S, U>),
    Dereference(DerefExpr<A>),
    LazyBool(LazyBoolExpr<B>),
    Negation(NegationExpr<A, B, C, E, I, S, U>),
    Reference(RefExpr<A>),
    TypeCast(TypeCastExpr<C>),
    UnwrapExpr(UnwrapExpr<U>),
}

impl<A, B, C, E, I, S, U> Spanned for OperationExprKind<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
    fn span(&self) -> Span {
        match self {
            OperationExprKind::ArithmeticOrLogical(al) => al.span(),
            OperationExprKind::Comparison(c) => c.span(),
            OperationExprKind::CompoundAssign(ca) => ca.span(),
            OperationExprKind::Dereference(d) => d.span(),
            OperationExprKind::LazyBool(lb) => lb.span(),
            OperationExprKind::Negation(n) => n.span(),
            OperationExprKind::Reference(r) => r.span(),
            OperationExprKind::TypeCast(tc) => tc.span(),
            OperationExprKind::UnwrapExpr(u) => u.span(),
        }
    }
}

pub enum ArithmeticOrLogicalOperatorKind {
    Plus(Plus),
    Minus(Minus),
    Multiply(Asterisk),
    Divide(ForwardSlash),
    Modulus(Percent),
    LogicalAnd(Ampersand),
    LogicalOr(Pipe),
    LogicalXor(Caret),
    ShiftLeft(DblLessThan),
    ShiftRight(DblGreaterThan),
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

pub enum UnwrapOperandKind<U: Spanned> {
    Option(Option<U>),
    Result(Result<U, CompilerError>),
}

impl<T> Spanned for UnwrapOperandKind<T>
where
    T: Spanned,
{
    fn span(&self) -> Span {
        match self {
            UnwrapOperandKind::Option(o) => {
                if let Some(t) = o {
                    t.span()
                } else {
                    Span::default()
                }
            }
            UnwrapOperandKind::Result(r) => {
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
pub type DerefOperator = Asterisk;
pub type RefOperator = (Ampersand, Option<KwMut>);

pub struct ArithmeticOrLogicalExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    lhs: Box<Expression<A, B, C, E, I, S, U>>,
    operator: ArithmeticOrLogicalOperatorKind,
    rhs: Box<Expression<A, B, C, E, I, S, U>>,
}

impl<A, B, C, E, I, S, U> OperatorExpr for ArithmeticOrLogicalExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for ArithmeticOrLogicalExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> BooleanOperand for ArithmeticOrLogicalExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for ArithmeticOrLogicalExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Constant for ArithmeticOrLogicalExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for ArithmeticOrLogicalExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct AssignmentExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    assignee: Box<Expression<A, B, C, E, I, S, U>>,
    operator: AssignOperator,
    new_value: Box<Expression<A, B, C, E, I, S, U>>,
}

impl<A, B, C, E, I, S, U> OperatorExpr for AssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for AssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> BooleanOperand for AssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Constant for AssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for AssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for AssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
    fn span(&self) -> Span {
        let start_pos = self.assignee.span().start();
        let end_pos = self.new_value.span().end();
        let source = self.assignee.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct CompoundAssignmentExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    assignee: A,
    operator: CompoundAssignOperatorKind,
    new_value: Box<Expression<A, B, C, E, I, S, U>>,
}

impl<A, B, C, E, I, S, U> OperatorExpr for CompoundAssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for CompoundAssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> BooleanOperand for CompoundAssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for CompoundAssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Constant for CompoundAssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for CompoundAssignmentExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
    fn span(&self) -> Span {
        let start_pos = self.assignee.span().start();
        let end_pos = self.new_value.span().end();
        let source = self.assignee.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct ComparisonExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    lhs: Box<Expression<A, B, C, E, I, S, U>>,
    operator: ComparisonOperatorKind,
    rhs: Box<Expression<A, B, C, E, I, S, U>>,
}

impl<A, B, C, E, I, S, U> OperatorExpr for ComparisonExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for ComparisonExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> BooleanOperand for ComparisonExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for ComparisonExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Constant for ComparisonExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for ComparisonExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct DerefExpr<A: Assignable> {
    operator: DerefOperator,
    operand: A,
}

impl<A> OperatorExpr for DerefExpr<A> where A: Assignable + 'static {}

impl<A> ExprWithoutBlock for DerefExpr<A> where A: Assignable + 'static {}

impl<A> BooleanOperand for DerefExpr<A> where A: Assignable + 'static {}

impl<A> IterableExpr for DerefExpr<A> where A: Assignable + 'static {}

impl<A> Constant for DerefExpr<A> where A: Assignable + 'static {}

impl<A> Spanned for DerefExpr<A>
where
    A: Assignable,
{
    fn span(&self) -> Span {
        let start_pos = self.operator.span().start();
        let end_pos = self.operand.span().end();
        let source = self.operator.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct LazyBoolExpr<B: BooleanOperand + Spanned> {
    lhs: B,
    operator: LazyBoolOperatorKind,
    rhs: B,
}

impl<B> OperatorExpr for LazyBoolExpr<B> where B: BooleanOperand + Spanned {}

impl<B> ExprWithoutBlock for LazyBoolExpr<B> where B: BooleanOperand + Spanned {}

impl<B> BooleanOperand for LazyBoolExpr<B> where B: BooleanOperand + Spanned {}

impl<B> IterableExpr for LazyBoolExpr<B> where B: BooleanOperand + Spanned {}

impl<B> Constant for LazyBoolExpr<B> where B: BooleanOperand + Spanned {}

impl<B> Spanned for LazyBoolExpr<B>
where
    B: BooleanOperand + Spanned,
{
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct NegationExpr<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    operator: NegationOperatorKind,
    operand: Box<Expression<A, B, C, E, I, S, U>>,
}

impl<A, B, C, E, I, S, U> OperatorExpr for NegationExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> ExprWithoutBlock for NegationExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> BooleanOperand for NegationExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> IterableExpr for NegationExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Constant for NegationExpr<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for NegationExpr<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
    fn span(&self) -> Span {
        let start_pos = self.operator.span().start();
        let end_pos = self.operand.span().end();
        let source = self.operator.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RefExpr<A: Assignable> {
    operator: RefOperator,
    operand: A,
}

impl<A> OperatorExpr for RefExpr<A> where A: Assignable + 'static {}

impl<A> ExprWithoutBlock for RefExpr<A> where A: Assignable {}

impl<A> BooleanOperand for RefExpr<A> where A: Assignable + 'static {}

impl<A> IterableExpr for RefExpr<A> where A: Assignable + 'static {}

impl<A> Spanned for RefExpr<A>
where
    A: Assignable,
{
    fn span(&self) -> Span {
        let s1 = self.operator.0.span();
        let s2 = self.operand.span();

        Span::join(s1, s2)
    }
}

pub struct TypeCastExpr<C: Castable> {
    lhs: C,
    operator: CastOperator,
    rhs: C,
}

impl<C> OperatorExpr for TypeCastExpr<C> where C: Castable {}

impl<C> ExprWithoutBlock for TypeCastExpr<C> where C: Castable {}

impl<C> BooleanOperand for TypeCastExpr<C> where C: Castable {}

impl<C> IterableExpr for TypeCastExpr<C> where C: Castable {}

impl<C> Constant for TypeCastExpr<C> where C: Castable {}

impl<C> Spanned for TypeCastExpr<C>
where
    C: Castable,
{
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct UnwrapExpr<U: Spanned> {
    operand: UnwrapOperandKind<U>,
    operator: QuestionMark,
}

impl<U> OperatorExpr for UnwrapExpr<U> where U: Spanned + 'static {}

impl<U> ExprWithoutBlock for UnwrapExpr<U> where U: Spanned {}

impl<U> BooleanOperand for UnwrapExpr<U> where U: Spanned + 'static {}

impl<U> IterableExpr for UnwrapExpr<U> where U: Spanned + 'static {}

impl<U> Spanned for UnwrapExpr<U>
where
    U: Spanned,
{
    fn span(&self) -> Span {
        let start_pos = self.operand.span().start();
        let end_pos = self.operator.span().end();
        let source = self.operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

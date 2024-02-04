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

use super::{Assignable, BooleanOperand, Castable, Expression};

pub enum OperatorExprKind<T, U> {
    ArithmeticOrLogical(ArithmeticOrLogicalExpr),
    Comparison(ComparisonExpr),
    CompoundAssign(CompoundAssignmentExpr<T, U>),
    Dereference(DerefExpr<T, U>),
    LazyBool(LazyBoolExpr<T, U>),
    Negation(NegationExpr),
    Reference(RefExpr<T, U>),
    TypeCast(TypeCastExpr),
    UnwrapExpr(UnwrapExpr<U>),
}

impl<T, U> Spanned for OperatorExprKind<T, U> {
    fn span(&self) -> Span {
        match self {
            OperatorExprKind::ArithmeticOrLogical(al) => al.span(),
            OperatorExprKind::Comparison(c) => c.span(),
            OperatorExprKind::CompoundAssign(ca) => ca.span(),
            OperatorExprKind::Dereference(d) => d.span(),
            OperatorExprKind::LazyBool(lb) => lb.span(),
            OperatorExprKind::Negation(n) => n.span(),
            OperatorExprKind::Reference(r) => r.span(),
            OperatorExprKind::TypeCast(tc) => tc.span(),
            OperatorExprKind::UnwrapExpr(u) => u.span(),
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

pub enum UnwrapOperandKind<T> {
    Option(Option<T>),
    Result(Result<T, CompilerError>),
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

pub struct ArithmeticOrLogicalExpr {
    lhs: Expression,
    operator: ArithmeticOrLogicalOperatorKind,
    rhs: Expression,
}

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
    assignee: Expression,
    operator: AssignOperator,
    new_value: Expression,
}

impl Spanned for AssignmentExpr {
    fn span(&self) -> Span {
        let start_pos = self.assignee.span().start();
        let end_pos = self.new_value.span().end();
        let source = self.assignee.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct CompoundAssignmentExpr<T, U> {
    assignee: Assignable<T, U>,
    operator: CompoundAssignOperatorKind,
    new_value: Expression,
}

impl<T, U> Spanned for CompoundAssignmentExpr<T, U> {
    fn span(&self) -> Span {
        let start_pos = self.assignee.span().start();
        let end_pos = self.new_value.span().end();
        let source = self.assignee.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct ComparisonExpr {
    lhs: Expression,
    operator: ComparisonOperatorKind,
    rhs: Expression,
}

impl Spanned for ComparisonExpr {
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct DerefExpr<T, U> {
    operator: DerefOperator,
    operand: Assignable<T, U>,
}

impl<T, U> Spanned for DerefExpr<T, U> {
    fn span(&self) -> Span {
        let start_pos = self.operator.span().start();
        let end_pos = self.operand.span().end();
        let source = self.operator.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct LazyBoolExpr<T, U> {
    lhs: BooleanOperand<T, U>,
    operator: LazyBoolOperatorKind,
    rhs: BooleanOperand<T, U>,
}

impl<T, U> Spanned for LazyBoolExpr<T, U> {
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct NegationExpr {
    operator: NegationOperatorKind,
    operand: Expression,
}

impl Spanned for NegationExpr {
    fn span(&self) -> Span {
        let start_pos = self.operator.span().start();
        let end_pos = self.operand.span().end();
        let source = self.operator.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct RefExpr<T, U> {
    operator: RefOperator,
    operand: Assignable<T, U>,
}

impl<T, U> Spanned for RefExpr<T, U> {
    fn span(&self) -> Span {
        let s1 = self.operator.0.span();
        let s2 = self.operand.span();

        Span::join(s1, s2)
    }
}

pub struct TypeCastExpr {
    lhs: Castable,
    operator: CastOperator,
    rhs: Castable,
}

impl Spanned for TypeCastExpr {
    fn span(&self) -> Span {
        let start_pos = self.lhs.span().start();
        let end_pos = self.rhs.span().end();
        let source = self.lhs.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct UnwrapExpr<U> {
    operand: UnwrapOperandKind<U>,
    operator: QuestionMark,
}

impl<U> Spanned for UnwrapExpr<U> {
    fn span(&self) -> Span {
        let start_pos = self.operand.span().start();
        let end_pos = self.operator.span().end();
        let source = self.operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

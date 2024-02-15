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

use super::{Assignable, BooleanOperand, Castable, Expression, Operable};

#[derive(Clone)]
pub enum OperatorExprKind {
    Assignment(AssignmentExpr),
    ArithmeticOrLogical(ArithmeticOrLogicalExpr),
    Comparison(ComparisonExpr),
    CompoundAssign(CompoundAssignmentExpr),
    Dereference(DereferenceExpr),
    LazyBool(LazyBoolExpr),
    Negation(NegationExpr),
    Reference(ReferenceExpr),
    TypeCast(TypeCastExpr),
    UnwrapExpr(UnwrapExpr),
}

impl Spanned for OperatorExprKind {
    fn span(&self) -> Span {
        match self {
            OperatorExprKind::Assignment(a) => a.span(),
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

#[derive(Clone)]
pub enum ArithmeticOrLogicalOperatorKind {
    Add(Plus),
    Subtract(Minus),
    Multiply(Asterisk),
    Divide(ForwardSlash),
    Modulus(Percent),
    LogicalAnd(Ampersand),
    LogicalOr(Pipe),
    LogicalXOr(Caret),
    ShiftLeft(DblLessThan),
    ShiftRight(DblGreaterThan),
}

#[derive(Clone)]
pub enum ComparisonOperatorKind {
    Equality(DblEquals),
    NotEqual(BangEquals),
    LessThan(LessThan),
    GreaterThan(GreaterThan),
    LessThanOrEqual(LessThanEquals),
    GreaterThanOrEqual(GreaterThanEquals),
}

#[derive(Clone)]
pub enum CompoundAssignOperatorKind {
    AddAssign(PlusEquals),
    SubtractAssign(MinusEquals),
    MultiplyAssign(AsteriskEquals),
    DivideAssign(ForwardSlashEquals),
    ModulusAssign(PercentEquals),
}

#[derive(Clone)]
pub enum LazyBoolOperatorKind {
    LazyAnd(DblAmpersand),
    LazyOr(DblPipe),
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum UnwrapOperandKind {
    Option(Option<Box<Expression>>),
    Result(Result<Box<Expression>, CompilerError>),
}

impl Spanned for UnwrapOperandKind {
    fn span(&self) -> Span {
        match self {
            UnwrapOperandKind::Option(opt) => {
                if let Some(e) = opt {
                    e.span()
                } else {
                    Span::default()
                }
            }
            UnwrapOperandKind::Result(res) => {
                if let Ok(e) = res {
                    e.span()
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

#[derive(Clone)]
pub struct ArithmeticOrLogicalExpr {
    lhs: Box<Operable>,
    operator: ArithmeticOrLogicalOperatorKind,
    rhs: Box<Operable>,
}

impl Spanned for ArithmeticOrLogicalExpr {
    fn span(&self) -> Span {
        let s1 = self.lhs.span();
        let s2 = self.rhs.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct AssignmentExpr {
    assignee: Box<Operable>,
    operator: AssignOperator,
    new_value: Box<Operable>,
}

impl Spanned for AssignmentExpr {
    fn span(&self) -> Span {
        let s1 = self.assignee.span();
        let s2 = self.new_value.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct CompoundAssignmentExpr {
    assignee: Box<Operable>,
    operator: CompoundAssignOperatorKind,
    new_value: Box<Operable>,
}

impl Spanned for CompoundAssignmentExpr {
    fn span(&self) -> Span {
        let s1 = self.assignee.span();
        let s2 = self.new_value.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct ComparisonExpr {
    lhs: Box<Operable>,
    operator: ComparisonOperatorKind,
    rhs: Box<Operable>,
}

impl Spanned for ComparisonExpr {
    fn span(&self) -> Span {
        let s1 = self.lhs.span();
        let s2 = self.rhs.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct DereferenceExpr {
    operator: DerefOperator,
    operand: Box<Assignable>,
}

impl Spanned for DereferenceExpr {
    fn span(&self) -> Span {
        let s1 = self.operator.span();
        let s2 = self.operand.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct LazyBoolExpr {
    lhs: Box<BooleanOperand>,
    operator: LazyBoolOperatorKind,
    rhs: Box<BooleanOperand>,
}

impl Spanned for LazyBoolExpr {
    fn span(&self) -> Span {
        let s1 = self.lhs.span();
        let s2 = self.rhs.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct NegationExpr {
    operator: NegationOperatorKind,
    operand: Box<Operable>,
}

impl Spanned for NegationExpr {
    fn span(&self) -> Span {
        let s1 = self.operator.span();
        let s2 = self.operand.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct ReferenceExpr {
    operator: RefOperator,
    operand: Box<Assignable>,
}

impl Spanned for ReferenceExpr {
    fn span(&self) -> Span {
        let s1 = self.operator.0.span();
        let s2 = self.operand.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct TypeCastExpr {
    lhs: Box<Castable>,
    operator: CastOperator,
    rhs: Box<Castable>,
}

impl Spanned for TypeCastExpr {
    fn span(&self) -> Span {
        let s1 = self.lhs.span();
        let s2 = self.rhs.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct UnwrapExpr {
    operand: UnwrapOperandKind,
    operator: QuestionMark,
}

impl Spanned for UnwrapExpr {
    fn span(&self) -> Span {
        let s1 = self.operand.span();
        let s2 = self.operator.span();

        Span::join(s1, s2)
    }
}

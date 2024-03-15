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

use super::{Expression, Value};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ComparisonOperatorKind {
    Equality(DblEquals),
    NotEqual(BangEquals),
    LessThan(LessThan),
    GreaterThan(GreaterThan),
    LessThanOrEqual(LessThanEquals),
    GreaterThanOrEqual(GreaterThanEquals),
}

#[derive(Debug, Clone)]
pub enum CompoundAssignOperatorKind {
    AddAssign(PlusEquals),
    SubtractAssign(MinusEquals),
    MultiplyAssign(AsteriskEquals),
    DivideAssign(ForwardSlashEquals),
    ModulusAssign(PercentEquals),
}

#[derive(Debug, Clone)]
pub enum LazyBoolOperatorKind {
    LazyAnd(DblAmpersand),
    LazyOr(DblPipe),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ArithmeticOrLogicalExpr {
    pub lhs: Box<Value>,
    pub operator: ArithmeticOrLogicalOperatorKind,
    pub rhs: Box<Value>,
}

impl Spanned for ArithmeticOrLogicalExpr {
    fn span(&self) -> Span {
        let s1 = self.lhs.span();
        let s2 = self.rhs.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub assignee: Box<Value>,
    pub operator: AssignOperator,
    pub new_value: Box<Value>,
}

impl Spanned for AssignmentExpr {
    fn span(&self) -> Span {
        let s1 = self.assignee.span();
        let s2 = self.new_value.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct CompoundAssignmentExpr {
    pub assignee: Box<Value>,
    pub operator: CompoundAssignOperatorKind,
    pub new_value: Box<Value>,
}

impl Spanned for CompoundAssignmentExpr {
    fn span(&self) -> Span {
        let s1 = self.assignee.span();
        let s2 = self.new_value.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct ComparisonExpr {
    pub lhs: Box<Value>,
    pub operator: ComparisonOperatorKind,
    pub rhs: Box<Value>,
}

impl Spanned for ComparisonExpr {
    fn span(&self) -> Span {
        let s1 = self.lhs.span();
        let s2 = self.rhs.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct DereferenceExpr {
    pub operator: DerefOperator,
    pub operand: Box<Value>,
}

impl Spanned for DereferenceExpr {
    fn span(&self) -> Span {
        let s1 = self.operator.span();
        let s2 = self.operand.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct LazyBoolExpr {
    pub lhs: Box<Value>,
    pub operator: LazyBoolOperatorKind,
    pub rhs: Box<Value>,
}

impl Spanned for LazyBoolExpr {
    fn span(&self) -> Span {
        let s1 = self.lhs.span();
        let s2 = self.rhs.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct NegationExpr {
    pub operator: NegationOperatorKind,
    pub operand: Box<Value>,
}

impl Spanned for NegationExpr {
    fn span(&self) -> Span {
        let s1 = self.operator.span();
        let s2 = self.operand.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct ReferenceExpr {
    pub operator: RefOperator,
    pub operand: Box<Value>,
}

impl Spanned for ReferenceExpr {
    fn span(&self) -> Span {
        let s1 = self.operator.0.span();
        let s2 = self.operand.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct TypeCastExpr {
    pub lhs: Box<Value>,
    pub operator: CastOperator,
    pub rhs: Box<Value>,
}

impl Spanned for TypeCastExpr {
    fn span(&self) -> Span {
        let s1 = self.lhs.span();
        let s2 = self.rhs.span();

        Span::join(s1, s2)
    }
}

// TODO: parse by scanning for `None` or `Some` identifiers
#[derive(Debug, Clone)]
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

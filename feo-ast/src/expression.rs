mod array_expr;
mod block_expr;
mod call_expr;
mod closure_expr;
mod conditional_expr;
mod field_access_expr;
mod iteration_expr;
mod operator_expr;
mod parenthesized_expr;
mod range_expr;
mod return_expr;
mod struct_expr;
mod tuple_expr;
mod underscore_expr;

use feo_types::{
    literal::LiteralKind,
    span::{Span, Spanned},
};

use crate::path::PathExpr;

pub use self::{
    array_expr::{ArrayExpr, IndexExpr},
    block_expr::BlockExpr,
    call_expr::{FunctionCallExpr, MethodCallExpr},
    closure_expr::{ClosureParam, ClosureParamsOpt, ClosureWithBlock, ClosureWithoutBlock},
    conditional_expr::{IfExpr, MatchArm, MatchArmGuard, MatchArms, MatchExpr},
    field_access_expr::FieldAccessExpr,
    iteration_expr::{
        BreakExpr, ContinueExpr, InfiniteLoopExpr, IterLoopExpr, IterationExprKind,
        PredicateLoopExpr,
    },
    operator_expr::{
        ArithmeticOrLogicalExpr, ArithmeticOrLogicalOperatorKind, AssignmentExpr, ComparisonExpr,
        ComparisonOperatorKind, CompoundAssignOperatorKind, CompoundAssignmentExpr, DerefOperator,
        DereferenceExpr, LazyBoolExpr, LazyBoolOperatorKind, NegationExpr, NegationOperatorKind,
        OperatorExprKind, RefOperator, ReferenceExpr, TypeCastExpr, UnwrapExpr,
    },
    parenthesized_expr::ParenthesizedExpr,
    range_expr::{
        RangeExprKind, RangeFromExpr, RangeFromToExpr, RangeFullExpr, RangeInclusiveExpr,
        RangeToExpr, RangeToInclusiveExpr,
    },
    return_expr::ReturnExpr,
    struct_expr::{StructExpr, StructExprField, TupleStructExpr},
    tuple_expr::{TupleExpr, TupleIndexExpr},
    underscore_expr::UnderscoreExpr,
};

/// `Expression` always produce / evaluate to a value, and may have (side) effects.
#[derive(Debug, Clone)]
pub enum Expression {
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
    BlockExpr(BlockExpr),
    FunctionCallExpr(FunctionCallExpr),
    MethodCallExpr(MethodCallExpr),
    ClosureWithBlock(ClosureWithBlock),
    ClosureWithoutBlock(ClosureWithoutBlock),
    FieldAccessExpr(FieldAccessExpr),
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
    IterationExpr(IterationExprKind),
    BreakExpr(BreakExpr),
    ContinueExpr(ContinueExpr),
    Literal(LiteralKind),
    OperatorExpr(OperatorExprKind),
    ParenthesizedExpr(ParenthesizedExpr),
    PathExpr(PathExpr),
    RangeExpr(RangeExprKind),
    ReturnExpr(ReturnExpr),
    StructExpr(StructExpr),
    TupleStructExpr(TupleStructExpr),
    TupleExpr(TupleExpr),
    TupleIndexExpr(TupleIndexExpr),
    UnderscoreExpr(UnderscoreExpr),
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Self::ArrayExpr(ae) => ae.span(),
            Self::IndexExpr(ie) => ie.span(),
            Self::BlockExpr(be) => be.span(),
            Self::FunctionCallExpr(fc) => fc.span(),
            Self::MethodCallExpr(mc) => mc.span(),
            Self::ClosureWithBlock(cwb) => cwb.span(),
            Self::ClosureWithoutBlock(c) => c.span(),
            Self::FieldAccessExpr(fa) => fa.span(),
            Self::IfExpr(ife) => ife.span(),
            Self::MatchExpr(me) => me.span(),
            Self::IterationExpr(ite) => ite.span(),
            Self::BreakExpr(be) => be.span(),
            Self::ContinueExpr(ce) => ce.span(),
            Self::Literal(lit) => lit.span(),
            Self::OperatorExpr(oe) => oe.span(),
            Self::ParenthesizedExpr(par) => par.span(),
            Self::PathExpr(pie) => pie.span(),
            Self::RangeExpr(rng) => rng.span(),
            Self::ReturnExpr(rtn) => rtn.span(),
            Self::StructExpr(se) => se.span(),
            Self::TupleStructExpr(tse) => tse.span(),
            Self::TupleExpr(te) => te.span(),
            Self::TupleIndexExpr(ti) => ti.span(),
            Self::UnderscoreExpr(ue) => ue.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprWithoutBlock {
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
    ClosureWithoutBlock(ClosureWithoutBlock),
    BreakExpr(BreakExpr),
    ContinueExpr(ContinueExpr),
    FieldAccessExpr(FieldAccessExpr),
    FunctionCallExpr(FunctionCallExpr),
    Literal(LiteralKind),
    MethodCallExpr(MethodCallExpr),
    OperatorExpr(OperatorExprKind),
    ParenthesizedExpr(ParenthesizedExpr),
    PathExpr(PathExpr),
    RangeExpr(RangeExprKind),
    ReturnExpr(ReturnExpr),
    StructExpr(StructExpr),
    TupleStructExpr(TupleStructExpr),
    TupleExpr(TupleExpr),
    TupleIndexExpr(TupleIndexExpr),
    UnderscoreExpr(UnderscoreExpr),
}

#[derive(Debug, Clone)]
pub enum ExprWithBlock {
    BlockExpr(BlockExpr),
    ClosureWithBlock(ClosureWithBlock),
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
    InfiniteLoopExpr(InfiniteLoopExpr),
    PredicateLoopExpr(PredicateLoopExpr),
    IterLoopExpr(IterLoopExpr),
}

impl Spanned for ExprWithBlock {
    fn span(&self) -> Span {
        match self {
            Self::BlockExpr(be) => be.span(),
            Self::ClosureWithBlock(cwb) => cwb.span(),
            Self::IfExpr(ife) => ife.span(),
            Self::MatchExpr(me) => me.span(),
            Self::InfiniteLoopExpr(inf) => inf.span(),
            Self::PredicateLoopExpr(ple) => ple.span(),
            Self::IterLoopExpr(ite) => ite.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TermCollection<T> {
    first_term: T,
    subsequent_terms_opt: Option<Vec<T>>,
}

impl<T> TermCollection<T> {
    pub fn new(first_term: T, subsequent_terms_opt: Option<Vec<T>>) -> Self {
        Self {
            first_term,
            subsequent_terms_opt,
        }
    }
}

impl<T: Spanned> Spanned for TermCollection<T> {
    fn span(&self) -> Span {
        let s1 = self.first_term.span();
        let s2 = match &self.subsequent_terms_opt {
            Some(s) => match s.last() {
                Some(t) => t.span(),
                None => self.first_term.span(),
            },
            None => self.first_term.span(),
        };

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    ArrayExpr(ArrayExpr),
    // IndexExpr(IndexExpr),
    // FunctionCallExpr(FunctionCallExpr),
    // MethodCallExpr(MethodCallExpr),
    FieldAccessExpr(FieldAccessExpr),
    Literal(LiteralKind),
    // ArithmeticOrLogicalExpr(ArithmeticOrLogicalExpr),
    // DereferenceExpr(DereferenceExpr),
    // NegationExpr(NegationExpr),
    // ReferenceExpr(ReferenceExpr),
    // UnwrapExpr(UnwrapExpr),
    ParenthesizedExpr(ParenthesizedExpr),
    PathExpr(PathExpr),
    StructExpr(StructExpr),
    TupleStructExpr(TupleStructExpr),
    TupleExpr(TupleExpr),
    // TupleIndexExpr(TupleIndexExpr),
    UnderscoreExpr(UnderscoreExpr),
}

impl Spanned for Value {
    fn span(&self) -> Span {
        match self {
            Self::ArrayExpr(ae) => ae.span(),
            // Self::IndexExpr(ie) => ie.span(),
            // Self::FunctionCallExpr(fc) => fc.span(),
            // Self::MethodCallExpr(mc) => mc.span(),
            Self::FieldAccessExpr(fa) => fa.span(),
            Self::Literal(lit) => lit.span(),
            // Self::ArithmeticOrLogicalExpr(ale) => ale.span(),
            // Self::DereferenceExpr(de) => de.span(),
            // Self::NegationExpr(ne) => ne.span(),
            // Self::ReferenceExpr(re) => re.span(),
            // Self::UnwrapExpr(ue) => ue.span(),
            Self::ParenthesizedExpr(par) => par.span(),
            Self::PathExpr(pth) => pth.span(),
            Self::StructExpr(se) => se.span(),
            Self::TupleStructExpr(tse) => tse.span(),
            Self::TupleExpr(tup) => tup.span(),
            // Self::TupleIndexExpr(tie) => tie.span(),
            Self::UnderscoreExpr(ue) => ue.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueCollection {
    pub first_value: Box<Value>,
    pub subsequent_values_opt: Option<Vec<Value>>,
}

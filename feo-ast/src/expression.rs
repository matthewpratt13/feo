#![allow(dead_code)]

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
    array_expr::{
        ArrayElementsCommaSeparated, ArrayElementsKind, ArrayElementsRepeatedValue, ArrayExpr,
        IndexExpr,
    },
    block_expr::BlockExpr,
    call_expr::{CallParams, FunctionCallExpr, MethodCallExpr},
    closure_expr::{
        ClosureExprKind, ClosureParam, ClosureParams, ClosureParamsOpt, ClosureWithBlock,
        ClosureWithoutBlock,
    },
    conditional_expr::{IfExpr, MatchExpr},
    field_access_expr::FieldAccessExpr,
    iteration_expr::{
        BreakExpr, ContinueExpr, InfiniteLoopExpr, IterLoopExpr, IterationExprKind,
        PredicateLoopExpr,
    },
    operator_expr::{
        ArithmeticOrLogicalExpr, ArithmeticOrLogicalOperatorKind, AssignmentExpr, ComparisonExpr,
        ComparisonOperatorKind, CompoundAssignOperatorKind, CompoundAssignmentExpr, DerefOperator,
        DereferenceExpr, LazyBoolExpr, LazyBoolOperatorKind, NegationExpr, NegationOperatorKind,
        OperatorExprKind, RefOperator, ReferenceExpr, TypeCastExpr, UnwrapExpr, UnwrapOperandKind,
    },
    parenthesized_expr::ParenthesizedExpr,
    range_expr::{
        RangeExprKind, RangeFromExpr, RangeFromToExpr, RangeInclusiveExpr, RangeToExpr,
        RangeToInclusiveExpr,
    },
    return_expr::ReturnExpr,
    struct_expr::{
        StructExpr, StructExprField, StructExprFields, StructExprKind, TupleStructExpr,
        TupleStructExprFields,
    },
    tuple_expr::{TupleExpr, TupleExprElements, TupleIndexExpr},
    underscore_expr::UnderscoreExpr,
};

// expressions always produce / evaluate to a value, and may have (side) effects

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
    StructExpr(StructExprKind),
    TupleExpr(TupleExpr),
    TupleIndexExpr(TupleIndexExpr),
    UnderscoreExpr(UnderscoreExpr),
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Expression::ArrayExpr(ae) => ae.span(),
            Expression::IndexExpr(ie) => ie.span(),
            Expression::BlockExpr(be) => be.span(),
            Expression::FunctionCallExpr(fc) => fc.span(),
            Expression::MethodCallExpr(mc) => mc.span(),
            Expression::ClosureWithBlock(cwb) => cwb.span(),
            Expression::ClosureWithoutBlock(c) => c.span(),
            Expression::FieldAccessExpr(fa) => fa.span(),
            Expression::IfExpr(ife) => ife.span(),
            Expression::MatchExpr(me) => me.span(),
            Expression::IterationExpr(ite) => ite.span(),
            Expression::BreakExpr(be) => be.span(),
            Expression::ContinueExpr(ce) => ce.span(),
            Expression::Literal(lit) => lit.span(),
            Expression::OperatorExpr(oe) => oe.span(),
            Expression::ParenthesizedExpr(par) => par.span(),
            Expression::PathExpr(pie) => pie.span(),
            Expression::RangeExpr(rng) => rng.span(),
            Expression::ReturnExpr(rtn) => rtn.span(),
            Expression::StructExpr(se) => se.span(),
            Expression::TupleExpr(te) => te.span(),
            Expression::TupleIndexExpr(ti) => ti.span(),
            Expression::UnderscoreExpr(ue) => ue.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprWithBlock {
    BlockExpr(BlockExpr),
    ClosureWithBlock(ClosureWithBlock),
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
    IterationExpr(IterationExprKind),
}

impl Spanned for ExprWithBlock {
    fn span(&self) -> Span {
        match self {
            ExprWithBlock::BlockExpr(be) => be.span(),
            ExprWithBlock::ClosureWithBlock(cwb) => cwb.span(),
            ExprWithBlock::IfExpr(ife) => ife.span(),
            ExprWithBlock::MatchExpr(me) => me.span(),
            ExprWithBlock::IterationExpr(ite) => ite.span(),
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
    StructExpr(StructExprKind),
    TupleExpr(TupleExpr),
    TupleIndexExpr(TupleIndexExpr),
    UnderscoreExpr(UnderscoreExpr),
}

#[derive(Debug, Clone)]
pub enum Term {
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
    FunctionCallExpr(FunctionCallExpr),
    MethodCallExpr(MethodCallExpr),
    FieldAccessExpr(FieldAccessExpr),
    Literal(LiteralKind),
    ArithmeticOrLogicalExpr(ArithmeticOrLogicalExpr),
    NegationExpr(NegationExpr),
    ParenthesizedExpr(ParenthesizedExpr),
    PathExpr(PathExpr),
    StructExpr(StructExprKind),
    TupleExpr(TupleExpr),
    TupleIndexExpr(TupleIndexExpr),
}

impl Spanned for Term {
    fn span(&self) -> Span {
        match self {
            Term::ArrayExpr(ae) => ae.span(),
            Term::IndexExpr(ie) => ie.span(),
            Term::FunctionCallExpr(fc) => fc.span(),
            Term::MethodCallExpr(mc) => mc.span(),
            Term::FieldAccessExpr(fa) => fa.span(),
            Term::Literal(lit) => lit.span(),
            Term::ArithmeticOrLogicalExpr(ale) => ale.span(),
            Term::NegationExpr(ne) => ne.span(),
            Term::ParenthesizedExpr(par) => par.span(),
            Term::PathExpr(pth) => pth.span(),
            Term::StructExpr(se) => se.span(),
            Term::TupleExpr(tup) => tup.span(),
            Term::TupleIndexExpr(tie) => tie.span(),
        }
    }
}

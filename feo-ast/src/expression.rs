#![allow(dead_code)]

mod array_expr;
mod attribute;
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

use feo_types::{
    span::{Span, Spanned},
    Identifier, Keyword, Punctuation,
};

use crate::{literal::LiteralKind, path::PathExpr, statement::Statement};

use self::{
    array_expr::{ArrayExpr, IndexExpr},
    call_expr::{FunctionCallExpr, MethodCallExpr},
    closure_expr::{ClosureWithBlock, ClosureWithoutBlock},
    conditional_expr::{IfExpr, MatchExpr},
    field_access_expr::FieldAccessExpr,
    iteration_expr::IterationExprKind,
    operator_expr::OperationExprKind,
    parenthesized_expr::ParenthesizedExpr,
    range_expr::RangeExprKind,
    return_expr::ReturnExpr,
    struct_expr::StructKind,
    tuple_expr::TupleExpr,
};
pub use self::{
    attribute::{AttributeKind, InnerAttr, OuterAttr},
    block_expr::BlockExpr,
    operator_expr::{ArithmeticOrLogicalOperatorKind, DerefOperator},
    struct_expr::{Struct, StructExpr, StructExprField, StructExprFields},
};

// expressions always produce / evaluate to a value, and may have (side) effects

// expressions:
// - array, index
// - block
// - function / method call
// - closure
// - if, match
// - field access
// - grouped
// - literal (char, string, int, uint, float, bytes32, bool)
// - loop / while / for..in
// - operators:
//      - arithmetic / logical
//      - assignment / compound assignment
//      - bool
//      - borrow
//      - comparison
//      - deref
//      - negation
//      - result / option unwrap
//      - type cast
// - path
// - range:
//      - from..to
//      - from..
//      - ..to
//      - from..=to
//      - ..=to
// - return
// - struct / tuple struct / unit struct
// - tuple

pub trait Assignable
where
    Self: Spanned,
{
}

impl Assignable for Identifier {}

pub trait BooleanOperand
where
    Self: 'static,
{
}

impl BooleanOperand for Keyword {}

pub trait Castable
where
    Self: Spanned + 'static,
{
}

pub trait Constant
where
    Self: Sized + Spanned + 'static,
{
}

impl Constant for Punctuation {}

// pub trait Expression
// where
//     Self: Spanned,
// {
// }

// impl Expression for Keyword {}

// impl Expression for Punctuation {}

pub trait ExprWithBlock {}

pub trait ExprWithoutBlock {}

impl ExprWithoutBlock for Keyword {}

impl ExprWithoutBlock for Punctuation {}

pub trait IterableExpr
where
    Self: 'static,
{
}

impl IterableExpr for Keyword {}

pub enum Expression<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    ArrayExpr(ArrayExpr<I>),
    IndexExpr(IndexExpr<I>),
    BlockExpr(BlockExpr<E, S>),
    FunctionCallExpr(FunctionCallExpr<A, B, C, E, I, S, U>),
    MethodCallExpr(MethodCallExpr<A, B, C, E, I, S, U>),
    ClosureWithBlock(ClosureWithBlock<E, S>),
    ClosureWithoutBlock(ClosureWithoutBlock<A, B, C, E, I, S, U>),
    FieldAccessExpr(FieldAccessExpr<A>),
    IfExpr(IfExpr<B, E, S>),
    IterationExpr(IterationExprKind<B, E, I, S>),
    LiteralExpr(LiteralKind),
    MatchExpr(MatchExpr<A, B, C, E, I, S, U>),
    OperationExpr(OperationExprKind<A, B, C, E, I, S, U>),
    ParenthesizedExpr(ParenthesizedExpr<A, B, C, E, I, S, U>),
    PathExpr(PathExpr),
    RangeExpr(RangeExprKind<A, B, C, E, I, S, U>),
    ReturnExpr(ReturnExpr<A, B, C, E, I, S, U>),
    StructExpr(StructKind<A, B, C, E, I, S, U>),
    TupleExpr(TupleExpr<A, B, C, E, I, S, U>),
}

impl<A, B, C, E, I, S, U> Spanned for Expression<A, B, C, E, I, S, U>
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
            Expression::ArrayExpr(arr) => arr.span(),
            Expression::IndexExpr(ind) => ind.span(),
            Expression::BlockExpr(bl) => bl.span(),
            Expression::FunctionCallExpr(fun) => fun.span(),
            Expression::MethodCallExpr(met) => met.span(),
            Expression::ClosureWithBlock(cwb) => cwb.span(),
            Expression::ClosureWithoutBlock(c) => c.span(),
            Expression::FieldAccessExpr(fa) => fa.span(),
            Expression::IfExpr(ife) => ife.span(),
            Expression::IterationExpr(ite) => ite.span(),
            Expression::LiteralExpr(lit) => lit.span(),
            Expression::MatchExpr(mat) => mat.span(),
            Expression::OperationExpr(op) => op.span(),
            Expression::ParenthesizedExpr(par) => par.span(),
            Expression::PathExpr(pat) => pat.span(),
            Expression::RangeExpr(rng) => rng.span(),
            Expression::ReturnExpr(ret) => ret.span(),
            Expression::StructExpr(st) => st.span(),
            Expression::TupleExpr(tup) => tup.span(),
        }
    }
}

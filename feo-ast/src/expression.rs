#![allow(dead_code)]

use feo_types::U256;

use crate::{
    identifier::Identifier,
    keyword::KeywordKind,
    path::SimplePath,
    punctuation::PuncKind,
    statement::Statement,
    type_utils::{Bracket, Dot, HashSign, Parenthesis},
};

mod array_expr;
mod block_expr;
mod call_expr;
mod conditional_expr;
mod iteration_expr;
mod operator_expr;
mod range_expr;
mod struct_expr;
mod tuple_expr;

use self::{
    array_expr::{ArrayExpr, IndexExpr},
    call_expr::{FunctionCallExpr, MethodCallExpr},
    conditional_expr::{IfExpr, MatchExpr},
    iteration_expr::{InfiniteLoopExpr, IterLoopExpr, PredicateLoopExpr},
    operator_expr::{
        ArithmeticOrLogicalExpr, AssignmentExpr, BoolExpr, ComparisonExpr, NegationExpr,
        ResultUnwrapExpr, TypeCastExpr,
    },
    range_expr::{
        RangeFromExpr, RangeFromToExpr, RangeInclusiveExpr, RangeToExpr, RangeToInclusiveExpr,
    },
    tuple_expr::{TupleExpr, TupleIndexingExpr},
};

pub use self::block_expr::BlockExpr;

pub trait Expression {}

pub trait ExprWithBlock<E>
where
    E: Expression,
{
}

pub trait ExprWithoutBlock<E>
where
    E: Expression,
{
}

pub trait ConditionalExpr<C>
where
    C: Expression,
{
}

pub trait IterationExpr<I>
where
    I: Expression,
{
}

pub trait LiteralExpr<L>
where
    L: Expression,
{
}

pub trait OperatorExpr<O>
where
    O: Expression,
{
}

pub trait RangeExpr<R>
where
    R: Expression,
{
}

pub trait StructExpr<S>
where
    S: Expression,
{
}

impl Expression for ArithmeticOrLogicalExpr {}
impl<E> ExprWithoutBlock<E> for ArithmeticOrLogicalExpr where E: Expression {}
impl<O> OperatorExpr<O> for ArithmeticOrLogicalExpr where O: Expression {}

impl Expression for ArrayExpr {}
impl<E> ExprWithoutBlock<E> for ArrayExpr where E: Expression {}

impl Expression for AssignmentExpr {}
impl<E> ExprWithoutBlock<E> for AssignmentExpr where E: Expression {}
impl<O> OperatorExpr<O> for AssignmentExpr where O: Expression {}

impl Expression for Attribute {}
impl<E> ExprWithBlock<E> for Attribute where E: Expression {}
impl<E> ExprWithoutBlock<E> for Attribute where E: Expression {}

impl<T, U> Expression for BlockExpr<T, U> {}
impl<T, U, E> ExprWithBlock<E> for BlockExpr<T, U> where E: Expression {}

impl Expression for BoolExpr {}
impl<E> ExprWithoutBlock<E> for BoolExpr where E: Expression {}
impl<O> OperatorExpr<O> for BoolExpr where O: Expression {}

impl Expression for ComparisonExpr {}
impl<E> ExprWithoutBlock<E> for ComparisonExpr where E: Expression {}
impl<O> OperatorExpr<O> for ComparisonExpr where O: Expression {}

impl<C> Expression for dyn ConditionalExpr<C> {}
impl<C, E> ExprWithBlock<E> for dyn ConditionalExpr<C> where E: Expression {}

impl<E> Expression for dyn ExprWithBlock<E> {}

impl<E> Expression for dyn ExprWithoutBlock<E> {}
impl<E> Statement for dyn ExprWithoutBlock<E> {}

impl Expression for FieldAccessExpr {}
impl<E> ExprWithoutBlock<E> for FieldAccessExpr where E: Expression {}

impl Expression for FunctionCallExpr {}
impl<E> ExprWithoutBlock<E> for FunctionCallExpr where E: Expression {}

impl Expression for GroupedExpr {}
impl<E> ExprWithoutBlock<E> for GroupedExpr where E: Expression {}

impl<T, U> Expression for IfExpr<T, U> {}
impl<T, U, C> ConditionalExpr<C> for IfExpr<T, U> where C: Expression {}
impl<T, U, E> ExprWithBlock<E> for IfExpr<T, U> where E: Expression {}

impl Expression for IndexExpr {}
impl<E> ExprWithoutBlock<E> for IndexExpr where E: Expression {}

impl<T, U> Expression for InfiniteLoopExpr<T, U> {}
impl<T, U, E> ExprWithBlock<E> for InfiniteLoopExpr<T, U> where E: Expression {}
impl<T, U, I> IterationExpr<I> for InfiniteLoopExpr<T, U> where I: Expression {}

impl<I> Expression for dyn IterationExpr<I> where I: Expression {}
impl<I, E> ExprWithBlock<E> for dyn IterationExpr<I> where E: Expression {}

impl<T, U> Expression for IterLoopExpr<T, U> {}
impl<T, U, I> IterationExpr<I> for IterLoopExpr<T, U> where I: Expression {}

impl Expression for KeywordKind {}
impl<E> ExprWithoutBlock<E> for KeywordKind where E: Expression {}

impl<L> Expression for dyn LiteralExpr<L> where L: Expression {}
impl<L, E> ExprWithoutBlock<E> for dyn LiteralExpr<L> where E: Expression {}

impl Expression for MatchExpr {}
impl<C> ConditionalExpr<C> for MatchExpr where C: Expression {}
impl<E> ExprWithBlock<E> for MatchExpr where E: Expression {}

impl Expression for MethodCallExpr {}
impl<E> ExprWithoutBlock<E> for MethodCallExpr where E: Expression {}

impl Expression for NegationExpr {}
impl<E> ExprWithoutBlock<E> for NegationExpr where E: Expression {}
impl<O> OperatorExpr<O> for NegationExpr where O: Expression {}

impl<O> Expression for dyn OperatorExpr<O> where O: Expression {}
impl<O, E> ExprWithoutBlock<E> for dyn OperatorExpr<O> where E: Expression {}

impl<T, U> Expression for PredicateLoopExpr<T, U> {}
impl<T, U, E> ExprWithBlock<E> for PredicateLoopExpr<T, U> where E: Expression {}
impl<T, U, I> IterationExpr<I> for PredicateLoopExpr<T, U> where I: Expression {}

impl Expression for PuncKind {}
impl<E> ExprWithoutBlock<E> for PuncKind where E: Expression {}
impl<R> RangeExpr<R> for PuncKind where R: Expression {}

impl<R> Expression for dyn RangeExpr<R> where R: Expression {}
impl<R, E> ExprWithoutBlock<E> for dyn RangeExpr<R> where E: Expression {}

impl Expression for RangeFromToExpr {}
impl<E> ExprWithoutBlock<E> for RangeFromToExpr where E: Expression {}
impl<R> RangeExpr<R> for RangeFromToExpr where R: Expression {}

impl Expression for RangeFromExpr {}
impl<E> ExprWithoutBlock<E> for RangeFromExpr where E: Expression {}
impl<R> RangeExpr<R> for RangeFromExpr where R: Expression {}

impl Expression for RangeToExpr {}
impl<E> ExprWithoutBlock<E> for RangeToExpr where E: Expression {}
impl<R> RangeExpr<R> for RangeToExpr where R: Expression {}

impl Expression for RangeInclusiveExpr {}
impl<E> ExprWithoutBlock<E> for RangeInclusiveExpr where E: Expression {}
impl<R> RangeExpr<R> for RangeInclusiveExpr where R: Expression {}

impl Expression for RangeToInclusiveExpr {}
impl<E> ExprWithoutBlock<E> for RangeToInclusiveExpr where E: Expression {}
impl<R> RangeExpr<R> for RangeToInclusiveExpr where R: Expression {}

impl Expression for ResultUnwrapExpr {}
impl<E> ExprWithoutBlock<E> for ResultUnwrapExpr where E: Expression {}
impl<O> OperatorExpr<O> for ResultUnwrapExpr where O: Expression {}

impl Expression for ReturnExpr {}
impl<E> ExprWithoutBlock<E> for ReturnExpr where E: Expression {}

impl Expression for SimplePath {}
impl<E> ExprWithoutBlock<E> for SimplePath where E: Expression {}

impl<S> Expression for dyn StructExpr<S> where S: Expression {}
impl<S, E> ExprWithoutBlock<E> for dyn StructExpr<S> where E: Expression {}

impl Expression for self::struct_expr::Struct {}
impl<E> ExprWithoutBlock<E> for self::struct_expr::Struct where E: Expression {}
impl<S> StructExpr<S> for self::struct_expr::Struct where S: Expression {}

impl Expression for TupleExpr {}
impl<E> ExprWithoutBlock<E> for TupleExpr where E: Expression {}

impl Expression for TupleIndexingExpr {}
impl<E> ExprWithoutBlock<E> for TupleIndexingExpr where E: Expression {}

impl Expression for self::struct_expr::TupleStruct {}
impl<E> ExprWithoutBlock<E> for self::struct_expr::TupleStruct where E: Expression {}
impl<S> StructExpr<S> for self::struct_expr::TupleStruct where S: Expression {}

impl Expression for TypeCastExpr {}
impl<E> ExprWithoutBlock<E> for TypeCastExpr where E: Expression {}
impl<O> OperatorExpr<O> for TypeCastExpr where O: Expression {}

impl Expression for self::struct_expr::UnitStruct {}
impl<E> ExprWithoutBlock<E> for self::struct_expr::UnitStruct where E: Expression {}
impl<S> StructExpr<S> for self::struct_expr::UnitStruct where S: Expression {}

impl Expression for char {}

impl Expression for &'static str {}

impl Expression for i64 {}

impl Expression for u64 {}

impl Expression for U256 {}

impl Expression for [u8; 32] {}

impl Expression for bool {}

// pub enum ExpressionKind {
//     WithoutBlock(ExprWithoutBlockKind),
//     WithBlock(ExprWithBlockKind),
// }

// pub enum ExprWithoutBlockKind {
//     Attr(Attribute),
//     Array(ArrayExpr),
//     FunctionCall(FunctionCallExpr),
//     MethodCall(MethodCallExpr),
//     Break(KeywordKind),
//     Continue(KeywordKind),
//     FieldAccess(FieldAccessExpr),
//     Grouped(GroupedExpr),
//     Index(IndexExpr),
//     Literal(LiteralExprKind),
//     Operator(OperatorExprKind),
//     Path(SimplePath),
//     Range(RangeExprKind),
//     Struct(StructExprKind),
//     Tuple(TupleExpr),
//     TupleIndexing(TupleIndexingExpr),
//     Return(ReturnExpr),
//     Underscore(PuncKind),
// }

// pub enum ExprWithBlockKind {
//     Attr(Attribute),
//     Block(BlockExpr),
//     Conditional(ConditionalExprKind),
//     Iteration(IterationExprKind),
// }

// pub enum LiteralExprKind {
//     Char(char),
//     Str(&'static str),
//     Int(i64),
//     UInt(u64),
//     U256(U256),
//     Float(f64),
//     Bytes32([u8; 32]),
//     Bool(bool),
// }

pub struct Attribute {
    hash: HashSign,
    open_bracket: Bracket,
    attribute_path: SimplePath,
    close_bracket: Bracket,
}

pub struct FieldAccessExpr {
    object: Box<dyn Expression>,
    dot: Dot,
    field_name: Identifier,
}

pub struct GroupedExpr {
    open_parenthesis: Parenthesis,
    expression: Box<dyn Expression>,
    close_parenthesis: Parenthesis,
}

pub struct ReturnExpr {
    kw_return: KeywordKind,
    expression_opt: Option<Box<dyn Expression>>,
}

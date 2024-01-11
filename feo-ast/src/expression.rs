use crate::{keyword::KeywordKind, path::SimplePath, punctuation::PuncKind};

mod array_expr;
mod block_expr;
mod call_expr;
mod conditional_expr;
mod iteration_expr;
mod literal_expr;
mod operator_expr;
mod struct_expr;
mod tuple_expr;

use self::{
    array_expr::{ArrayExpr, IndexExpr},
    block_expr::BlockExpr,
    call_expr::{FunctionCallExpr, MethodCallExpr},
    conditional_expr::ConditionalExpr,
    iteration_expr::IterationExpr,
    literal_expr::LiteralExpr,
    operator_expr::OperatorExpr,
    struct_expr::StructExpr,
    tuple_expr::{TupleExpr, TupleIndexingExpr},
};

pub enum Expression {
    WithBlock(ExprWithBlock),
    WithoutBlock(ExprWithoutBlock),
}

pub enum ExprWithoutBlock {
    Attr(Attribute),
    Array(ArrayExpr),
    FunctionCall(FunctionCallExpr),
    MethodCall(MethodCallExpr),
    Break(KeywordKind),
    Continue(KeywordKind),
    FieldAccess(FieldAccessExpr),
    Grouped(GroupedExpr),
    Index(IndexExpr),
    Literal(LiteralExpr),
    Operator(OperatorExpr),
    Path(SimplePath),
    Struct(StructExpr),
    Tuple(TupleExpr),
    TupleIndexing(TupleIndexingExpr),
    Return(ReturnExpr),
    Underscore(PuncKind),
}

pub enum ExprWithBlock {
    Attr(Attribute),
    Block(BlockExpr),
    Conditional(ConditionalExpr),
    Iteration(IterationExpr),
}

pub struct Attribute {}

pub struct FieldAccessExpr {}

pub struct GroupedExpr {}

pub struct ReturnExpr {}

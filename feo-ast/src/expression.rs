use crate::{keyword::KeywordKind, path::SimplePath, punctuation::PuncKind};

mod array_expr;
mod block_expr;
mod call_expr;
mod conditional_expr;
mod iterator_expr;
mod literal_expr;
mod operator_expr;
mod struct_expr;
mod tuple_expr;

use self::{
    array_expr::{ArrayExpr, IndexExpr},
    block_expr::BlockExpr,
    call_expr::{FunctionCallExpr, MethodCallExpr},
    conditional_expr::{IfExpr, MatchExpr},
    iterator_expr::IteratorExpr,
    literal_expr::LiteralExpr,
    operator_expr::OperatorExpr,
    struct_expr::StructExpr,
    tuple_expr::{TupleExpr, TupleIndexingExpr},
};

pub enum Expression {
    WithoutBlock(ExprWithoutBlock),
    WithBlock(ExprWithBlock),
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
    Underscore(UnderscoreExpr),
}

pub enum ExprWithBlock {
    Attr(Attribute),
    Block(BlockExpr),
    IfExpr(IfExpr),
    IterExpr(IteratorExpr),
    MatchExpr(MatchExpr),
}

pub type UnderscoreExpr = PuncKind;

pub struct Attribute {}

pub struct GroupedExpr {}

pub struct FieldAccessExpr {}

pub struct ReturnExpr {}

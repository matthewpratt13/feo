mod array_expr;
mod block_expr;
mod call_expr;
mod conditional_expr;
mod iterator_expr;
mod literal_expr;
mod operator_expr;
mod tuple_expr;

mod path_expr;

use crate::keyword::KeywordKind;

pub use self::path_expr::PathExpr;
use self::{
    array_expr::{ArrayExpr, IndexExpr},
    block_expr::BlockExpr,
    call_expr::{FunctionCallExpr, MethodCallExpr},
    conditional_expr::{IfExpr, MatchExpr},
    iterator_expr::LoopExpr,
    literal_expr::LiteralExpr,
    operator_expr::OperatorExpr,
    struct_expr::StructExpr,
    tuple_expr::{TupleExpr, TupleIndexingExpr},
};

mod struct_expr;

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
    Path(PathExpr),
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
    LoopExpr(LoopExpr),
    MatchExpr(MatchExpr),
}

pub struct Attribute {}

pub struct GroupedExpr {}

pub struct FieldAccessExpr {}

pub struct ReturnExpr {}

pub struct UnderscoreExpr {}

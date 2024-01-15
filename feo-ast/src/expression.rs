#![allow(dead_code)]

use feo_types::U256;

use crate::{
    identifier::Identifier,
    item::{Bracket, Dot, HashSign, Parenthesis},
    keyword::KeywordKind,
    path::SimplePath,
    punctuation::PuncKind,
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
    block_expr::BlockExpr,
    call_expr::{FunctionCallExpr, MethodCallExpr},
    conditional_expr::ConditionalExpr,
    iteration_expr::IterationExpr,
    operator_expr::OperatorExpr,
    range_expr::RangeExprKind,
    struct_expr::StructExprKind,
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
    Range(RangeExprKind),
    Struct(StructExprKind),
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

pub enum LiteralExpr {
    Char(char),
    Str(&'static str),
    Int(i64),
    UInt(u64),
    U256(U256),
    Float(f64),
    Bytes32([u8; 32]),
    Bool(bool),
}

pub struct Attribute {
    hash: HashSign,
    open_bracket: Bracket,
    attribute_path: SimplePath,
    close_bracket: Bracket,
}

pub struct FieldAccessExpr {
    object: Box<Expression>,
    dot: Dot,
    field_name: Identifier,
}

pub struct GroupedExpr {
    open_parenthesis: Parenthesis,
    expression: Box<Expression>,
    close_parenthesis: Parenthesis,
}

pub struct ReturnExpr {
    kw_return: KeywordKind,
    expression_opt: Option<Box<Expression>>,
}

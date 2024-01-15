#![allow(dead_code)]

use feo_types::U256;

use crate::{
    identifier::Identifier,
    keyword::KeywordKind,
    path::SimplePath,
    punctuation::PuncKind,
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
    conditional_expr::ConditionalExprKind,
    iteration_expr::IterationExprKind,
    operator_expr::OperatorExprKind,
    range_expr::RangeExprKind,
    struct_expr::StructExprKind,
    tuple_expr::{TupleExpr, TupleIndexingExpr},
};

pub use self::block_expr::BlockExpr;

pub enum ExpressionKind {
    WithoutBlock(ExprWithoutBlockKind),
    WithBlock(ExprWithBlockKind),
}

pub enum ExprWithoutBlockKind {
    Attr(Attribute),
    Array(ArrayExpr),
    FunctionCall(FunctionCallExpr),
    MethodCall(MethodCallExpr),
    Break(KeywordKind),
    Continue(KeywordKind),
    FieldAccess(FieldAccessExpr),
    Grouped(GroupedExpr),
    Index(IndexExpr),
    Literal(LiteralExprKind),
    Operator(OperatorExprKind),
    Path(SimplePath),
    Range(RangeExprKind),
    Struct(StructExprKind),
    Tuple(TupleExpr),
    TupleIndexing(TupleIndexingExpr),
    Return(ReturnExpr),
    Underscore(PuncKind),
}

pub enum ExprWithBlockKind {
    Attr(Attribute),
    Block(BlockExpr),
    Conditional(ConditionalExprKind),
    Iteration(IterationExprKind),
}

pub enum LiteralExprKind {
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
    object: Box<ExpressionKind>,
    dot: Dot,
    field_name: Identifier,
}

pub struct GroupedExpr {
    open_parenthesis: Parenthesis,
    expression: Box<ExpressionKind>,
    close_parenthesis: Parenthesis,
}

pub struct ReturnExpr {
    kw_return: KeywordKind,
    expression_opt: Option<Box<ExpressionKind>>,
}

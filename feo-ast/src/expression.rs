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
    utils::Underscore,
    Identifier, U256,
};

use crate::{
    item::{ConstantItem, EnumItem, EnumVariantStruct, EnumVariantTuple, StaticItem},
    literal::{Literal, LiteralKind},
    path::PathExpr,
    statement::{ExprStatement, LetStatement},
};

use self::{
    array_expr::{ArrayExpr, IndexExpr},
    call_expr::{FunctionCallExpr, MethodCallExpr},
    closure_expr::{ClosureWithBlock, ClosureWithoutBlock},
    conditional_expr::{IfExpr, MatchExpr},
    field_access_expr::FieldAccessExpr,
    iteration_expr::{
        BreakExpr, ContinueExpr, InfiniteLoopExpr, IterationExprKind, PredicateLoopExpr,
    },
    operator_expr::{
        ArithmeticOrLogicalExpr, AssignmentExpr, ComparisonExpr, CompoundAssignmentExpr,
        DereferenceExpr, LazyBoolExpr, NegationExpr, OperatorExprKind, TypeCastExpr,
    },
    parenthesized_expr::ParenthesizedExpr,
    range_expr::RangeExprKind,
    return_expr::ReturnExpr,
    struct_expr::{StructKind, TupleStruct, UnitStruct},
    tuple_expr::{TupleExpr, TupleIndexExpr},
};
pub use self::{
    attribute::{AttributeKind, InnerAttr, OuterAttr},
    block_expr::BlockExpr,
    operator_expr::{ArithmeticOrLogicalOperatorKind, DerefOperator},
    struct_expr::{Struct, StructExprField, StructExprFields},
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

// pub trait Assignable
// where
//     Self: Spanned,
// {
// }

pub enum Assignable {
    Identifier(Identifier),
    ArrayExpr(ArrayExpr),
    StructExpr(Struct),
    TupleStructExpr(TupleStruct),
    UnitStructExpr(UnitStruct),
    TupleExpr(TupleExpr),
    PathExpr(PathExpr),
    UnderscoreExpr(Underscore),
}

impl Spanned for Assignable {
    fn span(&self) -> Span {
        todo!()
    }
}

// impl Assignable for Identifier {}

// pub trait BooleanOperand
// where
//     Self: 'static,
// {
// }

pub enum BooleanOperand {
    BreakExpr(BreakExpr),
    ContinueExpr(ContinueExpr),
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
    BlockExpr(BlockExpr),
    FunctionCallExpr(FunctionCallExpr),
    MethodCallExpr(MethodCallExpr),
    ClosureWithBlock(ClosureWithBlock),
    ClosureWithoutBlock(ClosureWithoutBlock),
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
    FieldAccessExpr(FieldAccessExpr),
    IterationExpr(IterationExprKind),
    OperatorExpr(OperatorExprKind),
    ParenthesizedExpr(ParenthesizedExpr),
    RangeExpr(RangeExprKind),
    ReturnExpr(ReturnExpr),
    TupleExpr(TupleExpr),
    TupleIndexExpr(TupleIndexExpr),
    LiteralExpr(LiteralKind),
    PathExpr(PathExpr),
    UnderscoreExpr(Underscore),
}

impl Spanned for BooleanOperand {
    fn span(&self) -> Span {
        todo!()
    }
}

// impl BooleanOperand for Keyword {}

// pub trait Castable
// where
//     Self: Spanned + 'static,
// {
// }

pub enum Castable {
    Char(Literal<char>),
    Bool(Literal<bool>),
    I32(Literal<i32>),
    I64(Literal<i64>),
    U8(Literal<u8>),
    U16(Literal<u16>),
    U32(Literal<u32>),
    U64(Literal<u64>),
    U256(Literal<U256>),
    F32(Literal<f32>),
    F64(Literal<f64>),
}

impl Spanned for Castable {
    fn span(&self) -> Span {
        todo!()
    }
}

// pub trait Constant
// where
//     Self: Sized + Spanned + 'static,
// {
// }

pub enum Constant {
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
    BlockExpr(BlockExpr),
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
    BreakExpr(BreakExpr),
    ContinueExpr(ContinueExpr),
    FieldAccessExpr(FieldAccessExpr),
    InfiniteLoopExpr(InfiniteLoopExpr),
    PredicateLoopExpr(PredicateLoopExpr),
    ArithmeticOrLogicalExpr(ArithmeticOrLogicalExpr),
    AssignmentExpr(AssignmentExpr),
    CompoundAssignmentExpr(CompoundAssignmentExpr),
    ComparisonAssignmentExpr(ComparisonExpr),
    DerefExpr(DereferenceExpr),
    LazyBoolExpr(LazyBoolExpr),
    NegationExpr(NegationExpr),
    TypeCastExpr(TypeCastExpr),
    ParenthesizedExpr(ParenthesizedExpr),
    RangeExpr(RangeExprKind),
    StructExpr(StructKind),
    TupleExpr(TupleExpr),
    ConstantItem(ConstantItem),
    StaticItem(StaticItem),
    EnumItem(EnumItem),
    EnumVariantStruct(EnumVariantStruct),
    EnumVariantTuple(EnumVariantTuple),
    UnitStructItem(UnitStruct),
    Literal(LiteralKind),
    PathExpr(PathExpr),
    ExprStatement(ExprStatement),
    LetStatement(LetStatement),
    UnderscoreExpr(Underscore),
}

// pub trait Expression
// where
//     Self: Spanned,
// {
// }

// pub trait ExprWithBlock {}

pub enum ExprWithBlock {
    OuterAttr(OuterAttr),
    BlockExpr(BlockExpr),
    ClosureWithBlock(ClosureWithBlock),
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
    IterationExpr(IterationExprKind),
}

impl Spanned for ExprWithBlock {
    fn span(&self) -> Span {
        todo!()
    }
}

// pub trait ExprWithoutBlock {}

pub enum ExprWithoutBlock {
    BreakExpr(BreakExpr),
    ContinueExpr(ContinueExpr),
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
    InnerAttr(InnerAttr),
    OuterAttr(OuterAttr),
    FunctionCallExpr(FunctionCallExpr),
    MethodCallExpr(MethodCallExpr),
    ClosureWithoutBlock(ClosureWithoutBlock),
    FieldAccessExpr(FieldAccessExpr),
    OperatorExpr(OperatorExprKind),
    ParenthesizedExpr(ParenthesizedExpr),
    RangeExpr(RangeExprKind),
    ReturnExpr(ReturnExpr),
    StructExpr(StructKind),
    TupleExpr(TupleExpr),
    TupleIndexExpr(TupleIndexExpr),
    EnumItem(EnumItem),
    EnumVariantStruct(EnumVariantStruct),
    Literal(LiteralKind),
    PathExpr(PathExpr),
    UnderscoreExpr(Underscore),
}

// pub trait IterableExpr
// where
//     Self: 'static,
// {
// }

// impl IterableExpr for Keyword {}

pub enum IterableExpr {
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
    BlockExpr(BlockExpr),
    FunctionCallExpr(FunctionCallExpr),
    MethodCallExpr(MethodCallExpr),
    ClosureWithBlock(ClosureWithBlock),
    ClosureWithoutBlock(ClosureWithoutBlock),
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
    FieldAccessExpr(FieldAccessExpr),
    IterationExpr(IterationExprKind),
    OperatorExpr(OperatorExprKind),
    ParenthesizedExpr(ParenthesizedExpr),
    RangeExpr(RangeExprKind),
    ReturnExpr(ReturnExpr),
    TupleExpr(TupleExpr),
    LiteralExpr(LiteralKind),
    PathExpr(PathExpr),
}

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
    IterationExpr(IterationExprKind),
    BreakExpr(BreakExpr),
    ContinueExpr(ContinueExpr),
    LiteralExpr(LiteralKind),
    MatchExpr(MatchExpr),
    OperatorExpr(OperatorExprKind),
    ParenthesizedExpr(ParenthesizedExpr),
    PathExpr(PathExpr),
    RangeExpr(RangeExprKind),
    ReturnExpr(ReturnExpr),
    StructExpr(StructKind),
    TupleExpr(TupleExpr),
    UnderscoreExpr(Underscore),
}

impl Spanned for Expression {
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
            Expression::OperatorExpr(op) => op.span(),
            Expression::ParenthesizedExpr(par) => par.span(),
            Expression::PathExpr(pat) => pat.span(),
            Expression::RangeExpr(rng) => rng.span(),
            Expression::ReturnExpr(ret) => ret.span(),
            Expression::StructExpr(st) => st.span(),
            Expression::TupleExpr(tup) => tup.span(),
            Expression::BreakExpr(_) => todo!(),
            Expression::ContinueExpr(_) => todo!(),
            Expression::UnderscoreExpr(_) => todo!(),
        }
    }
}

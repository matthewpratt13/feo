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
    literal::{FloatType, IntType, Literal, LiteralKind, UIntType},
    span::{Span, Spanned},
    utils::Underscore,
    Identifier, U256,
};

use crate::{
    item::{ConstantVarDef, EnumDef, EnumVariantStruct, EnumVariantTuple, StaticVarDef},
    path::PathExpr,
    statement::{ExprStatement, LetStatement},
};

pub use self::{
    array_expr::{
        ArrayElementsCommaSeparated, ArrayElementsKind, ArrayElementsRepeatedValue, ArrayExpr,
        IndexExpr,
    },
    attribute::{AttributeKind, InnerAttr, OuterAttr},
    block_expr::BlockExpr,
    call_expr::CallParams,
    closure_expr::{ClosureExprKind, ClosureParam, ClosureParams},
    conditional_expr::{IfExpr, MatchExpr},
    iteration_expr::IterLoopExpr,
    operator_expr::{
        ArithmeticOrLogicalOperatorKind, ComparisonOperatorKind, CompoundAssignOperatorKind,
        DerefOperator, LazyBoolOperatorKind, UnwrapOperandKind,
    },
    struct_expr::{StructExpr, StructExprField, StructExprFields, StructExprKind},
};

use self::{
    call_expr::{FunctionCallExpr, MethodCallExpr},
    closure_expr::{ClosureWithBlock, ClosureWithoutBlock},
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
    struct_expr::{TupleStructExpr, UnitStructExpr},
    tuple_expr::{TupleExpr, TupleIndexExpr},
};

// expressions always produce / evaluate to a value, and may have (side) effects

#[derive(Clone)]
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
    InnerAttr(InnerAttr),
    OuterAttr(OuterAttr),
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
    StructExpr(StructExprKind),
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
            Expression::BreakExpr(be) => be.span(),
            Expression::ContinueExpr(ce) => ce.span(),
            Expression::UnderscoreExpr(ue) => ue.span(),
            Expression::InnerAttr(inn) => inn.span(),
            Expression::OuterAttr(out) => out.span(),
        }
    }
}

#[derive(Clone)]
pub enum Assignable {
    Identifier(Identifier),
    ArrayExpr(ArrayExpr),
    StructExpr(StructExpr),
    TupleStructExpr(TupleStructExpr),
    UnitStructExpr(UnitStructExpr),
    TupleExpr(TupleExpr),
    PathExpr(PathExpr),
    UnderscoreExpr(Underscore),
}

impl Spanned for Assignable {
    fn span(&self) -> Span {
        match self {
            Assignable::Identifier(id) => id.span(),
            Assignable::ArrayExpr(ae) => ae.span(),
            Assignable::StructExpr(se) => se.span(),
            Assignable::TupleStructExpr(ts) => ts.span(),
            Assignable::UnitStructExpr(us) => us.span(),
            Assignable::TupleExpr(te) => te.span(),
            Assignable::PathExpr(pe) => pe.span(),
            Assignable::UnderscoreExpr(ue) => ue.span(),
        }
    }
}

#[derive(Clone)]
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
        match self {
            BooleanOperand::BreakExpr(be) => be.span(),
            BooleanOperand::ContinueExpr(ce) => ce.span(),
            BooleanOperand::ArrayExpr(ae) => ae.span(),
            BooleanOperand::IndexExpr(ie) => ie.span(),
            BooleanOperand::BlockExpr(be) => be.span(),
            BooleanOperand::FunctionCallExpr(fc) => fc.span(),
            BooleanOperand::MethodCallExpr(mc) => mc.span(),
            BooleanOperand::ClosureWithBlock(cwb) => cwb.span(),
            BooleanOperand::ClosureWithoutBlock(cb) => cb.span(),
            BooleanOperand::IfExpr(ife) => ife.span(),
            BooleanOperand::MatchExpr(me) => me.span(),
            BooleanOperand::FieldAccessExpr(fa) => fa.span(),
            BooleanOperand::IterationExpr(ite) => ite.span(),
            BooleanOperand::OperatorExpr(oe) => oe.span(),
            BooleanOperand::ParenthesizedExpr(par) => par.span(),
            BooleanOperand::RangeExpr(rae) => rae.span(),
            BooleanOperand::ReturnExpr(re) => re.span(),
            BooleanOperand::TupleExpr(te) => te.span(),
            BooleanOperand::TupleIndexExpr(tie) => tie.span(),
            BooleanOperand::LiteralExpr(le) => le.span(),
            BooleanOperand::PathExpr(pat) => pat.span(),
            BooleanOperand::UnderscoreExpr(ue) => ue.span(),
        }
    }
}

#[derive(Clone)]
pub enum Castable {
    Char(Literal<char>),
    Bool(Literal<bool>),
    I32(Literal<IntType>),
    I64(Literal<IntType>),
    U8(Literal<UIntType>),
    U16(Literal<UIntType>),
    U32(Literal<UIntType>),
    U64(Literal<UIntType>),
    U256(Literal<U256>),
    F32(Literal<FloatType>),
    F64(Literal<FloatType>),
}

impl Spanned for Castable {
    fn span(&self) -> Span {
        match self {
            Castable::Char(c) => c.span(),
            Castable::Bool(b) => b.span(),
            Castable::I32(i) => i.span(),
            Castable::I64(i) => i.span(),
            Castable::U8(ui) => ui.span(),
            Castable::U16(ui) => ui.span(),
            Castable::U32(ui) => ui.span(),
            Castable::U64(ui) => ui.span(),
            Castable::U256(u) => u.span(),
            Castable::F32(f) => f.span(),
            Castable::F64(f) => f.span(),
        }
    }
}

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
    StructExpr(StructExprKind),
    TupleExpr(TupleExpr),
    ConstantVarDef(ConstantVarDef),
    StaticVarDef(StaticVarDef),
    EnumDef(EnumDef),
    EnumVariantStruct(EnumVariantStruct),
    EnumVariantTuple(EnumVariantTuple),
    UnitStructExpr(UnitStructExpr),
    Literal(LiteralKind),
    PathExpr(PathExpr),
    ExprStatement(ExprStatement),
    LetStatement(LetStatement),
    UnderscoreExpr(Underscore),
}

#[derive(Clone)]
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
        match self {
            ExprWithBlock::OuterAttr(oa) => oa.span(),
            ExprWithBlock::BlockExpr(be) => be.span(),
            ExprWithBlock::ClosureWithBlock(cwb) => cwb.span(),
            ExprWithBlock::IfExpr(ife) => ife.span(),
            ExprWithBlock::MatchExpr(me) => me.span(),
            ExprWithBlock::IterationExpr(ite) => ite.span(),
        }
    }
}

#[derive(Clone)]
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
    StructExpr(StructExprKind),
    TupleExpr(TupleExpr),
    TupleIndexExpr(TupleIndexExpr),
    EnumDef(EnumDef),
    EnumVariantStruct(EnumVariantStruct),
    Literal(LiteralKind),
    PathExpr(PathExpr),
    UnderscoreExpr(Underscore),
}

#[derive(Clone)]
pub enum IterableExpr {
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
    BlockExpr(BlockExpr),
    FunctionCallExpr(FunctionCallExpr),
    MethodCallExpr(MethodCallExpr),
    ClosureWithBlock(ClosureWithBlock),
    ClosureWithoutBlock(ClosureWithoutBlock),
    IfExpr(IfExpr),
    InnerAttr(InnerAttr),
    OuterAttr(OuterAttr),
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

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
    literal::{FloatType, IntType, Literal, LiteralKind, UIntType},
    span::{Span, Spanned},
    U256,
};

use crate::{
    item::{ConstantVarDef, EnumDef, EnumVariantStruct, EnumVariantTuple, StaticVarDef},
    path::PathExpr,
    statement::{ExprStatement, LetStatement},
};

use self::iteration_expr::{BreakExpr, ContinueExpr};

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
    iteration_expr::{InfiniteLoopExpr, IterLoopExpr, IterationExprKind, PredicateLoopExpr},
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
        StructExpr, StructExprField, StructExprFields, StructExprKind, TupleStructElements,
        TupleStructExpr,
    },
    tuple_expr::{TupleElements, TupleExpr, TupleIndexExpr},
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
    IterationExpr(IterationExprKind),
    BreakExpr(BreakExpr),
    ContinueExpr(ContinueExpr),
    Literal(LiteralKind),
    MatchExpr(MatchExpr),
    OperatorExpr(OperatorExprKind),
    ParenthesizedExpr(ParenthesizedExpr),
    PathExpr(PathExpr),
    RangeExpr(RangeExprKind),
    ReturnExpr(ReturnExpr),
    StructExpr(StructExprKind),
    TupleExpr(TupleExpr),
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
            Expression::IterationExpr(ite) => ite.span(),
            Expression::Literal(lit) => lit.span(),
            Expression::MatchExpr(me) => me.span(),
            Expression::OperatorExpr(oe) => oe.span(),
            Expression::ParenthesizedExpr(par) => par.span(),
            Expression::PathExpr(pie) => pie.span(),
            Expression::RangeExpr(rng) => rng.span(),
            Expression::ReturnExpr(rtn) => rtn.span(),
            Expression::StructExpr(se) => se.span(),
            Expression::TupleExpr(te) => te.span(),
            Expression::BreakExpr(be) => be.span(),
            Expression::ContinueExpr(ce) => ce.span(),
            Expression::UnderscoreExpr(ue) => ue.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Assignable {
    ArrayExpr(ArrayExpr),
    StructExpr(StructExprKind),
    TupleExpr(TupleExpr),
    PathExpr(PathExpr),
    UnderscoreExpr(UnderscoreExpr),
}

impl Spanned for Assignable {
    fn span(&self) -> Span {
        match self {
            Assignable::ArrayExpr(ae) => ae.span(),
            Assignable::StructExpr(se) => se.span(),
            Assignable::TupleExpr(te) => te.span(),
            Assignable::PathExpr(pe) => pe.span(),
            Assignable::UnderscoreExpr(ue) => ue.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BooleanOperand {
    IndexExpr(IndexExpr),
    BlockExpr(BlockExpr),
    FunctionCallExpr(FunctionCallExpr),
    MethodCallExpr(MethodCallExpr),
    FieldAccessExpr(FieldAccessExpr),
    ComparisonExpr(ComparisonExpr),
    NegationExpr(NegationExpr),
    UnwrapExpr(UnwrapExpr),
    ParenthesizedExpr(ParenthesizedExpr),
    TupleIndexExpr(TupleIndexExpr),
    Literal(LiteralKind),
    PathExpr(PathExpr),
    UnderscoreExpr(UnderscoreExpr),
}

impl Spanned for BooleanOperand {
    fn span(&self) -> Span {
        match self {
            BooleanOperand::IndexExpr(ie) => ie.span(),
            BooleanOperand::BlockExpr(be) => be.span(),
            BooleanOperand::FunctionCallExpr(fc) => fc.span(),
            BooleanOperand::MethodCallExpr(mc) => mc.span(),
            BooleanOperand::FieldAccessExpr(fa) => fa.span(),
            BooleanOperand::ComparisonExpr(ce) => ce.span(),
            BooleanOperand::NegationExpr(ne) => ne.span(),
            BooleanOperand::UnwrapExpr(ue) => ue.span(),
            BooleanOperand::ParenthesizedExpr(par) => par.span(),
            BooleanOperand::TupleIndexExpr(tie) => tie.span(),
            BooleanOperand::Literal(lit) => lit.span(),
            BooleanOperand::PathExpr(pth) => pth.span(),
            BooleanOperand::UnderscoreExpr(und) => und.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Callable {
    PathExpr(PathExpr),
    ParenthesizedExpr(ParenthesizedExpr),
}

impl Spanned for Callable {
    fn span(&self) -> Span {
        match self {
            Callable::PathExpr(pth) => pth.span(),
            Callable::ParenthesizedExpr(par) => par.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Castable {
    Char(Literal<char>),
    Bool(Literal<bool>),
    Int(Literal<IntType>),
    UInt(Literal<UIntType>),
    U256(Literal<U256>),
    Float(Literal<FloatType>),
    PathExpr(PathExpr),
}

impl Spanned for Castable {
    fn span(&self) -> Span {
        match self {
            Castable::Char(c) => c.span(),
            Castable::Bool(b) => b.span(),
            Castable::Int(i) => i.span(),
            Castable::UInt(ui) => ui.span(),
            Castable::U256(u) => u.span(),
            Castable::Float(f) => f.span(),
            Castable::PathExpr(pth) => pth.span(),
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
    TupleIndexExpr(TupleIndexExpr),
    ConstantVarDef(ConstantVarDef),
    StaticVarDef(StaticVarDef),
    EnumDef(EnumDef),
    EnumVariantStruct(EnumVariantStruct),
    EnumVariantTuple(EnumVariantTuple),
    Literal(LiteralKind),
    PathExpr(PathExpr),
    ExprStatement(ExprStatement),
    LetStatement(LetStatement),
    UnderscoreExpr(UnderscoreExpr),
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
    BreakExpr(BreakExpr),
    ContinueExpr(ContinueExpr),
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
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
    UnderscoreExpr(UnderscoreExpr),
}

#[derive(Debug, Clone)]
pub enum Iterable {
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
    BlockExpr(BlockExpr),
    FunctionCallExpr(FunctionCallExpr),
    MethodCallExpr(MethodCallExpr),
    FieldAccessExpr(FieldAccessExpr),
    DereferenceExpr(DereferenceExpr),
    ReferenceExpr(ReferenceExpr),
    TypeCastExpr(TypeCastExpr),
    UnwrapExpr(UnwrapExpr),
    ParenthesizedExpr(ParenthesizedExpr),
    RangeExpr(RangeExprKind),
    TupleExpr(TupleExpr),
    TupleIndexExpr(TupleIndexExpr),
    Literal(LiteralKind),
    PathExpr(PathExpr),
}

#[derive(Debug, Clone)]
pub enum Operable {
    IndexExpr(IndexExpr),
    TupleIndexExpr(TupleIndexExpr),
    FunctionCallExpr(FunctionCallExpr),
    MethodCallExpr(MethodCallExpr),
    FieldAccessExpr(FieldAccessExpr),
    Literal(LiteralKind),
    PathExpr(PathExpr),
    ParenthesizedExpr(ParenthesizedExpr),
    ArithmeticOrLogicalExpr(ArithmeticOrLogicalExpr),
    DereferenceExpr(DereferenceExpr),
    NegationExpr(NegationExpr),
    ReferenceExpr(ReferenceExpr),
    TypeCastExpr(TypeCastExpr),
    UnwrapExpr(UnwrapExpr),
}

impl Spanned for Operable {
    fn span(&self) -> Span {
        match self {
            Operable::IndexExpr(ie) => ie.span(),
            Operable::TupleIndexExpr(ti) => ti.span(),
            Operable::FunctionCallExpr(fc) => fc.span(),
            Operable::MethodCallExpr(mc) => mc.span(),
            Operable::FieldAccessExpr(fa) => fa.span(),
            Operable::Literal(lit) => lit.span(),
            Operable::PathExpr(pth) => pth.span(),
            Operable::ParenthesizedExpr(par) => par.span(),
            Operable::ArithmeticOrLogicalExpr(al) => al.span(),
            Operable::DereferenceExpr(de) => de.span(),
            Operable::NegationExpr(ne) => ne.span(),
            Operable::ReferenceExpr(re) => re.span(),
            Operable::TypeCastExpr(tc) => tc.span(),
            Operable::UnwrapExpr(ue) => ue.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Returnable {
    ArrayExpr(ArrayExpr),
    IndexExpr(IndexExpr),
    TupleIndexExpr(TupleIndexExpr),
    FunctionCallExpr(FunctionCallExpr),
    MethodCallExpr(MethodCallExpr),
    ClosureWithBlock(ClosureWithBlock),
    ClosureWithoutBlock(ClosureWithoutBlock),
    FieldAccessExpr(FieldAccessExpr),
    Literal(LiteralKind),
    PathExpr(PathExpr),
    StructExpr(StructExprKind),
    TupleExpr(TupleExpr),
    ParenthesizedExpr(ParenthesizedExpr),
    ArithmeticOrLogicalExpr(ArithmeticOrLogicalExpr),
    DereferenceExpr(DereferenceExpr),
    NegationExpr(NegationExpr),
    ReferenceExpr(ReferenceExpr),
    TypeCastExpr(TypeCastExpr),
    UnwrapExpr(UnwrapExpr),
    UnderscoreExpr(UnderscoreExpr),
}

impl Spanned for Returnable {
    fn span(&self) -> Span {
        match self {
            Returnable::ArrayExpr(ae) => ae.span(),
            Returnable::IndexExpr(ie) => ie.span(),
            Returnable::TupleIndexExpr(ti) => ti.span(),
            Returnable::FunctionCallExpr(fc) => fc.span(),
            Returnable::MethodCallExpr(mc) => mc.span(),
            Returnable::ClosureWithBlock(cwb) => cwb.span(),
            Returnable::ClosureWithoutBlock(c) => c.span(),
            Returnable::FieldAccessExpr(fa) => fa.span(),
            Returnable::Literal(lit) => lit.span(),
            Returnable::PathExpr(pth) => pth.span(),
            Returnable::StructExpr(se) => se.span(),
            Returnable::TupleExpr(te) => te.span(),
            Returnable::ParenthesizedExpr(par) => par.span(),
            Returnable::ArithmeticOrLogicalExpr(al) => al.span(),
            Returnable::DereferenceExpr(de) => de.span(),
            Returnable::NegationExpr(ne) => ne.span(),
            Returnable::ReferenceExpr(re) => re.span(),
            Returnable::TypeCastExpr(tc) => tc.span(),
            Returnable::UnwrapExpr(ue) => ue.span(),
            Returnable::UnderscoreExpr(und) => und.span(),
        }
    }
}

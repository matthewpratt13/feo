#![allow(dead_code)]

mod identifier_patt;
mod parenthesized_patt;
mod range_patt;
mod reference_patt;
mod struct_patt;
mod tuple_patt;

use feo_types::{
    literal::LiteralKind,
    span::{Span, Spanned},
    utils::Underscore,
};

use crate::{
    expression::{CallParams, ClosureParam, ClosureParams, IfExpr, IterLoopExpr, MatchExpr},
    item::{ConstantVarDef, EnumVariantStruct, EnumVariantTuple},
    path::PathPatt,
    statement::LetStatement,
};

pub use self::range_patt::RangePattBound;
use self::{
    identifier_patt::IdentifierPatt,
    parenthesized_patt::ParenthesizedPatt,
    range_patt::RangePatt,
    reference_patt::ReferencePatt,
    struct_patt::{StructPatt, TupleStructPatt},
    tuple_patt::TuplePatt,
};

// patterns are used: to match values against structures; in variable declarations; as func params

#[derive(Clone)]
pub enum Pattern {
    CallParams(CallParams),
    ClosureParam(ClosureParam),
    ClosureParams(ClosureParams),
    ConstantVarDef(ConstantVarDef),
    EnumVariantStruct(EnumVariantStruct),
    EnumVariantTuple(EnumVariantTuple),
    LetStatement(LetStatement),
    Literal(LiteralKind),
    IdentifierPatt(IdentifierPatt),
    IfExpr(IfExpr),
    IterLoopExpr(IterLoopExpr),
    MatchExpr(MatchExpr),
    ParenthesizedPatt(ParenthesizedPatt),
    RangePatt(RangePatt),
    PathPatt(PathPatt),
    ReferencePatt(ReferencePatt),
    StructPatt(StructPatt),
    TupleStructPatt(TupleStructPatt),
    TuplePatt(TuplePatt),
    WildcardPatt(Underscore),
}

impl Spanned for Pattern {
    fn span(&self) -> Span {
        todo!()
    }
}

#[derive(Clone)]
pub enum PatternWithoutRange {
    Literal(LiteralKind),
    IdentifierPatt(IdentifierPatt),
    ParenthesizedPatt(ParenthesizedPatt),
    PathPatt(PathPatt),
    ReferencePatt(ReferencePatt),
    StructPatt(StructPatt),
    TupleStructPatt(TupleStructPatt),
    TuplePatt(TuplePatt),
}

impl Spanned for PatternWithoutRange {
    fn span(&self) -> Span {
        todo!()
    }
}

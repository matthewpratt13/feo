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

use self::{
    identifier_patt::IdentifierPatt,
    parenthesized_patt::ParenthesizedPatt,
    range_patt::RangePatt,
    reference_patt::ReferencePatt,
    struct_patt::{StructPatt, TupleStructPatt},
    tuple_patt::TuplePatt,
};

// patterns are used: to match values against structures; in variable declarations; as func params

#[derive(Debug, Clone)]
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
        match self {
            Pattern::CallParams(cal) => cal.span(),
            Pattern::ClosureParam(clp) => clp.span(),
            Pattern::ClosureParams(cls) => cls.span(),
            Pattern::ConstantVarDef(cvd) => cvd.span(),
            Pattern::EnumVariantStruct(evs) => evs.span(),
            Pattern::EnumVariantTuple(evt) => evt.span(),
            Pattern::LetStatement(ls) => ls.span(),
            Pattern::Literal(lit) => lit.span(),
            Pattern::IdentifierPatt(idp) => idp.span(),
            Pattern::IfExpr(ife) => ife.span(),
            Pattern::IterLoopExpr(ite) => ite.span(),
            Pattern::MatchExpr(me) => me.span(),
            Pattern::ParenthesizedPatt(par) => par.span(),
            Pattern::RangePatt(rng) => match rng {
                RangePatt::RangeFromPatt(rfp) => rfp.span(),
                RangePatt::RangeInclusivePatt(rip) => rip.span(),
                RangePatt::RangeToInclusivePatt(rti) => rti.span(),
            },
            Pattern::PathPatt(pat) => pat.span(),
            Pattern::ReferencePatt(rfp) => rfp.span(),
            Pattern::StructPatt(sp) => sp.span(),
            Pattern::TupleStructPatt(tsp) => tsp.span(),
            Pattern::TuplePatt(tup) => tup.span(),
            Pattern::WildcardPatt(wcp) => wcp.span(),
        }
    }
}

#[derive(Debug, Clone)]
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
        match self {
            PatternWithoutRange::Literal(lit) => lit.span(),
            PatternWithoutRange::IdentifierPatt(id) => id.span(),
            PatternWithoutRange::ParenthesizedPatt(par) => par.span(),
            PatternWithoutRange::PathPatt(pat) => pat.span(),
            PatternWithoutRange::ReferencePatt(rp) => rp.span(),
            PatternWithoutRange::StructPatt(sp) => sp.span(),
            PatternWithoutRange::TupleStructPatt(tsp) => tsp.span(),
            PatternWithoutRange::TuplePatt(tup) => tup.span(),
        }
    }
}

#![allow(dead_code)]

mod identifier_patt;
mod parenthesized_patt;
mod range_patt;
mod struct_patt;
mod tuple_patt;
mod wildcard_patt;

use feo_types::{
    literal::LiteralKind,
    span::{Span, Spanned},
};

use crate::path::PathPatt;

pub use self::{
    identifier_patt::IdentifierPatt,
    parenthesized_patt::ParenthesizedPatt,
    range_patt::{
        RangeFromPatt, RangeInclusivePatt, RangePattBound, RangePattKind, RangeToInclusivePatt,
    },
    struct_patt::{StructPatt, StructPattField, TupleStructPatt, TupleStructPattField},
    tuple_patt::{TuplePatt, TuplePattElements},
    wildcard_patt::WildcardPatt,
};

// patterns are used: to match values against structures; in variable declarations; as func params

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(LiteralKind),
    IdentifierPatt(IdentifierPatt),
    ParenthesizedPatt(ParenthesizedPatt),
    RangePatt(RangePattKind),
    PathPatt(PathPatt),
    StructPatt(StructPatt),
    TupleStructPatt(TupleStructPatt),
    TuplePatt(TuplePatt),
    WildcardPatt(WildcardPatt),
}

impl Spanned for Pattern {
    fn span(&self) -> Span {
        match self {
            Pattern::Literal(lit) => lit.span(),
            Pattern::IdentifierPatt(idp) => idp.span(),
            Pattern::ParenthesizedPatt(par) => par.span(),
            Pattern::RangePatt(rng) => match rng {
                RangePattKind::RangeFromPatt(rfp) => rfp.span(),
                RangePattKind::RangeInclusivePatt(rip) => rip.span(),
                RangePattKind::RangeToInclusivePatt(rti) => rti.span(),
            },
            Pattern::PathPatt(pat) => pat.span(),
            Pattern::StructPatt(sp) => sp.span(),
            Pattern::TupleStructPatt(tsp) => tsp.span(),
            Pattern::TuplePatt(tup) => tup.span(),
            Pattern::WildcardPatt(wcp) => wcp.span(),
        }
    }
}

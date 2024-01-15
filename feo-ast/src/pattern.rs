#![allow(dead_code)]

use feo_types::U256;

use crate::{path::SimplePath, punctuation::PuncKind, type_utils::Parenthesis};

mod range_patt;
mod struct_patt;
mod tuple_patt;

use self::{
    identifier_patt::IdentifierPatt,
    range_patt::{RangeFromPatt, RangeInclusivePatt, RangeToInclusivePatt},
    struct_patt::{StructPatt, TupleStructPatt},
};

pub trait Pattern {}

pub trait LiteralPatt<L>
where
    L: Pattern,
{
}

pub trait RangePatt<R>
where
    R: Pattern,
{
}

impl Pattern for GroupedPatt {}

impl Pattern for IdentifierPatt {}

impl<L> Pattern for dyn LiteralPatt<L> {}

impl Pattern for PuncKind {}
impl<R> RangePatt<R> for PuncKind where R: Pattern {}

impl<R> Pattern for dyn RangePatt<R> {}

impl Pattern for RangeFromPatt {}
impl<R> RangePatt<R> for RangeFromPatt where R: Pattern {}

impl Pattern for RangeInclusivePatt {}
impl<R> RangePatt<R> for RangeInclusivePatt where R: Pattern {}

impl Pattern for RangeToInclusivePatt {}
impl<R> RangePatt<R> for RangeToInclusivePatt where R: Pattern {}

impl Pattern for SimplePath {}

impl Pattern for StructPatt {}

impl Pattern for TupleStructPatt {}

impl Pattern for char {}
impl<L> LiteralPatt<L> for char where L: Pattern {}

impl Pattern for &'static str {}
impl<L> LiteralPatt<L> for &'static str where L: Pattern {}

impl Pattern for i64 {}
impl<L> LiteralPatt<L> for i64 where L: Pattern {}

impl Pattern for u64 {}
impl<L> LiteralPatt<L> for u64 where L: Pattern {}

impl Pattern for U256 {}
impl<L> LiteralPatt<L> for U256 where L: Pattern {}

impl Pattern for f64 {}
impl<L> LiteralPatt<L> for f64 where L: Pattern {}

impl Pattern for [u8; 32] {}
impl<L> LiteralPatt<L> for [u8; 32] where L: Pattern {}

impl Pattern for bool {}
impl<L> LiteralPatt<L> for bool where L: Pattern {}

pub struct GroupedPatt {
    open_parenthesis: Parenthesis,
    pattern: Box<dyn Pattern>,
    close_parenthesis: Parenthesis,
}

mod identifier_patt {
    use crate::{identifier::Identifier, keyword::KeywordKind};

    pub struct IdentifierPatt {
        kw_ref_opt: Option<KeywordKind>,
        kw_mut_opt: Option<KeywordKind>,
        name: Identifier,
    }
}

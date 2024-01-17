#![allow(dead_code)]

use feo_types::span::Spanned;

mod grouped_patt;
mod identifier_patt;
mod range_patt;
mod struct_patt;
mod tuple_patt;

pub trait Pattern
where
    Self: Spanned,
{
}

pub trait LiteralPatt
where
    Self: Pattern,
{
}

pub trait RangePatt
where
    Self: Pattern,
{
}

pub trait RangePattBound
where
    Self: Pattern,
{
}

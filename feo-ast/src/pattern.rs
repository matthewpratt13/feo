#![allow(dead_code)]

mod grouped_patt;
mod identifier_patt;
mod range_patt;
mod struct_patt;
mod tuple_patt;

use feo_types::span::Spanned;

pub use self::range_patt::{RangePatt, RangePattBound};

pub trait Pattern
where
    Self: Spanned,
{
}

pub trait LiteralPatt
where
    Self: Sized + 'static + Pattern,
{
}

#![allow(dead_code)]

use feo_types::span::Spanned;

mod grouped_patt;
mod identifier_patt;
mod range_patt;
mod struct_patt;
mod tuple_patt;

pub use self::range_patt::{RangePatt, RangePattBound};

pub trait Pattern
where
    Self: Spanned,
{
}

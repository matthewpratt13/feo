#![allow(dead_code)]

mod parenthesized_patt;
mod identifier_patt;
mod range_patt;
mod struct_patt;
mod tuple_patt;

use crate::span::Spanned;

pub use self::range_patt::{RangePatt, RangePattBound};

// patterns are used: to match values against structures; in variable declarations; as func params

// patterns:
// - literals (char, string, int, uint, float, bytes32, bool)
// - identifier
// - struct, tuple struct
// - tuple
// - grouped
// - path

pub trait Pattern
where
    Self: Spanned,
{
}

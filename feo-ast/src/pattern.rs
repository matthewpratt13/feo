#![allow(dead_code)]

mod identifier_patt;
mod parenthesized_patt;
mod range_patt;
mod reference_patt;
mod struct_patt;
mod tuple_patt;

use feo_types::{span::Spanned, utils::Underscore};

pub use self::range_patt::RangePattBound;

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

pub trait PatternWithoutRange
where
    Self: Pattern,
{
}

impl Pattern for Underscore {}

// TODO: `OrPatt`

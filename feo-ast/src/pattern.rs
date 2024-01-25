#![allow(dead_code)]

mod identifier_patt;
mod parenthesized_patt;
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

pub trait PatternWithoutRange
where
    Self: Pattern,
{
}

mod ref_pattern {
    use crate::{
        keyword::Keyword,
        span::{Span, Spanned},
    };

    use super::{Pattern, PatternWithoutRange};

    pub struct RefPattern {
        kw_ref: Keyword,
        kw_mut_opt: Option<Keyword>,
        pattern: Box<dyn PatternWithoutRange>,
    }

    impl PatternWithoutRange for RefPattern {}

    impl Pattern for RefPattern {}

    impl Spanned for RefPattern {
        fn span(&self) -> Span {
            let start_pos = self.kw_ref.span().start();
            let end_pos = self.pattern.span().end();
            let source = self.kw_ref.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}

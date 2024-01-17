#![allow(dead_code)]

use feo_types::U256;

mod range_patt;
mod struct_patt;
mod tuple_patt;

pub trait Pattern {}

pub trait LiteralPatt<L>
where
    L: Pattern,
{
}

impl<L> Pattern for dyn LiteralPatt<L> {}

pub trait RangePatt<R>
where
    R: Pattern,
{
}

pub trait RangePattBound<R>
where
    R: Pattern,
{
}

impl<R> Pattern for dyn RangePatt<R> {}

impl Pattern for char {}
impl<L> LiteralPatt<L> for char where L: Pattern {}
impl<R> RangePattBound<R> for char where R: Pattern {}

impl Pattern for &'static str {}
impl<L> LiteralPatt<L> for &'static str where L: Pattern {}
impl<R> RangePattBound<R> for &'static str where R: Pattern {}

impl Pattern for i64 {}
impl<L> LiteralPatt<L> for i64 where L: Pattern {}
impl<R> RangePattBound<R> for i64 where R: Pattern {}

impl Pattern for u64 {}
impl<L> LiteralPatt<L> for u64 where L: Pattern {}
impl<R> RangePattBound<R> for u64 where R: Pattern {}

impl Pattern for U256 {}
impl<L> LiteralPatt<L> for U256 where L: Pattern {}
impl<R> RangePattBound<R> for U256 where R: Pattern {}

impl Pattern for f64 {}
impl<L> LiteralPatt<L> for f64 where L: Pattern {}
impl<R> RangePattBound<R> for f64 where R: Pattern {}

impl Pattern for &[u8; 32] {}
impl<L> LiteralPatt<L> for &[u8; 32] where L: Pattern {}
impl<R> RangePattBound<R> for &[u8; 32] where R: Pattern {}

impl Pattern for bool {}
impl<L> LiteralPatt<L> for bool where L: Pattern {}
impl<R> RangePattBound<R> for bool where R: Pattern {}

mod grouped_pattern {
    use feo_types::span::{Span, Spanned};

    use crate::type_utils::Parenthesis;

    use super::Pattern;

    pub struct GroupedPatt {
        open_parenthesis: Parenthesis,
        pattern: Box<dyn Pattern>,
        close_parenthesis: Parenthesis,
    }

    impl Pattern for GroupedPatt {}

    impl Spanned for GroupedPatt {
        fn span(&self) -> Span {
            let start_pos = self.open_parenthesis.span().start();
            let end_pos = self.close_parenthesis.span().end();
            let source = self.open_parenthesis.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}

mod identifier_patt {
    use crate::{identifier::Identifier, keyword::KeywordKind};

    use super::Pattern;

    pub struct IdentifierPatt {
        kw_ref_opt: Option<KeywordKind>,
        kw_mut_opt: Option<KeywordKind>,
        name: Identifier,
    }

    impl Pattern for IdentifierPatt {}
}

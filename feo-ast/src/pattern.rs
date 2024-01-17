#![allow(dead_code)]

use feo_types::span::Spanned;

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
    use feo_types::span::{Span, Spanned};

    use crate::{identifier::Identifier, keyword::Keyword};

    use super::Pattern;

    pub struct IdentifierPatt {
        kw_ref_opt: Option<Keyword>,
        kw_mut_opt: Option<Keyword>,
        name: Identifier,
    }

    impl Pattern for IdentifierPatt {}

    impl Spanned for IdentifierPatt {
        fn span(&self) -> Span {
            let start_pos = match &self.kw_ref_opt {
                Some(kwr) => match &self.kw_mut_opt {
                    Some(kwm) => kwm.span().start(),
                    None => kwr.span().start(),
                },
                None => self.name.span().start(),
            };

            let end_pos = self.name.span().end();
            let source = self.name.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}

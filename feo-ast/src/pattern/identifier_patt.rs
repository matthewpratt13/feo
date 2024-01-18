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
            Some(kwr) => kwr.span().start(),
            None => match &self.kw_mut_opt {
                Some(kwm) => kwm.span().start(),
                None => self.name.span().start(),
            },
        };

        let end_pos = self.name.span().end();
        let source = self.name.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

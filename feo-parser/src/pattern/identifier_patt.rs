use feo_ast::pattern::IdentifierPatt;
use feo_error::error::CompilerError;
use feo_types::{keyword::KeywordKind, Identifier, Keyword};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for IdentifierPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_ref_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwRef,
            ..
        }) = kw_ref_opt
        {
            parser.next_token();
        }

        let kw_mut_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwMut,
            ..
        }) = kw_mut_opt
        {
            parser.next_token();
        }

        if let Some(name) = parser.peek_current::<Identifier>() {
            parser.next_token();

            return Ok(Some(IdentifierPatt {
                kw_ref_opt,
                kw_mut_opt,
                name,
            }));
        }

        Ok(None)
    }
}

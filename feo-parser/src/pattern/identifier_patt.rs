use feo_ast::pattern::IdentifierPatt;
use feo_error::error::CompilerError;
use feo_types::{keyword::KeywordKind, Identifier, Keyword};

use crate::{parse::ParsePatt, parser::Parser};

impl ParsePatt for IdentifierPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_ref_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwRef,
            ..
        }) = kw_ref_opt
        {
            parser.next_token();
        }

        let kw_mut_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwMut,
            ..
        }) = kw_mut_opt
        {
            parser.next_token();
        }

        if let Some(name) = parser.peek_current::<Identifier>() {
            return Ok(Some(IdentifierPatt {
                kw_ref_opt,
                kw_mut_opt,
                name,
            }));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_identifier_patt() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"ref mut foo"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let identifier_patt =
            IdentifierPatt::parse(&mut parser).expect("unable to parse identifier pattern");

        Ok(println!("{:#?}", identifier_patt))
    }
}

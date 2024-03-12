use feo_ast::pattern::{PatternWithoutRange, ReferencePatt};
use feo_error::error::CompilerError;
use feo_types::{keyword::KeywordKind, Keyword};

use crate::{parse::ParsePatt, parser::Parser};

impl ParsePatt for ReferencePatt {
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
            let kw_mut_opt = parser.peek_current::<Keyword>();

            if let Some(Keyword {
                keyword_kind: KeywordKind::KwMut,
                ..
            }) = kw_mut_opt
            {
                parser.next_token();
            } else {
                if let Some(pattern) = PatternWithoutRange::parse(parser)? {
                    parser.next_token();

                    return Ok(Some(ReferencePatt {
                        kw_ref: kw_ref_opt.unwrap(),
                        kw_mut_opt,
                        pattern: Box::new(pattern),
                    }));
                }
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils;

    use super::*;

    #[ignore] // TODO: fix – getting stack overflow (check use of `Box<PatternWithoutRange>`)
    #[test]
    fn parse_reference_patt() {
        let source_code = r#"ref mut x"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let reference_patt =
            ReferencePatt::parse(&mut parser).expect("unable to parse reference pattern");

        println!("{:#?}", reference_patt);
    }
}

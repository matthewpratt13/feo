use feo_ast::{
    pattern::{Pattern, ReferencePatt},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{parse::ParsePatt, parser::Parser};

impl ParsePatt for ReferencePatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let ampersand_opt = parser.peek_current();

        if let Some(Punctuation {
            punc_kind: PuncKind::Ampersand,
            ..
        }) = ampersand_opt
        {
            parser.next_token();

            let kw_mut_opt = parser.peek_current();

            if let Some(Keyword {
                keyword_kind: KeywordKind::KwMut,
                ..
            }) = kw_mut_opt
            {
                parser.next_token();
            }

            if let Some(pattern) = Pattern::parse(parser)? {
                return Ok(Some(ReferencePatt {
                    ampersand: ampersand_opt.unwrap(),
                    kw_mut_opt,
                    pattern: Box::new(pattern),
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "pattern".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
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

    #[test]
    fn parse_reference_patt() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"&mut parser"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let reference_patt =
            ReferencePatt::parse(&mut parser).expect("unable to parse reference pattern");

        Ok(println!("{:#?}", reference_patt))
    }
}

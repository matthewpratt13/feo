use feo_ast::{token::Token, ty::ReferenceType, Type};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{parse::ParseType, parser::Parser};

impl ParseType for ReferenceType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let ampersand_opt = parser.peek_current::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::Ampersand,
            ..
        }) = ampersand_opt
        {
            parser.next_token();

            let kw_mut_opt = parser.peek_current::<Keyword>();

            if let Some(Keyword {
                keyword_kind: KeywordKind::KwMut,
                ..
            }) = kw_mut_opt
            {
                parser.next_token();
            }

            if let Some(ty) = Type::parse(parser)? {
                parser.next_token();

                return Ok(Some(ReferenceType(
                    ampersand_opt.unwrap(),
                    kw_mut_opt,
                    Box::new(ty),
                )));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`Type`".to_string(),
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

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_reference_type() {
        let source_code = r#"&mut bool"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let reference_type =
            ReferenceType::parse(&mut parser).expect("unable to parse reference type");

        println!("{:#?}", reference_type);
    }
}

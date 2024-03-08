use feo_ast::{token::Token, ty::ReferenceType, Type};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for ReferenceType {
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

            if let Some(t) = Type::parse(parser)? {
                parser.next_token();

                return Ok(Some(ReferenceType(
                    ampersand_opt.unwrap(),
                    kw_mut_opt,
                    Box::new(t),
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

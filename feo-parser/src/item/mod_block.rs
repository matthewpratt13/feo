use feo_ast::{item::ModWithoutBody, token::Token};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Identifier, Keyword, Punctuation};

use crate::{parse::ParseItem, parser::Parser, utils};

impl ParseItem for ModWithoutBody {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_mod_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwMod,
            ..
        }) = kw_mod_opt
        {
            parser.next_token();

            if let Some(mod_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let semicolon_opt = parser.peek_current();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Semicolon,
                    ..
                }) = semicolon_opt
                {
                    parser.next_token();

                    return Ok(Some(ModWithoutBody {
                        attributes_opt,
                        visibility_opt,
                        kw_mod: kw_mod_opt.unwrap(),
                        mod_name,
                        semicolon: semicolon_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`;`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

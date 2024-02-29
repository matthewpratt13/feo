use feo_ast::{expression::UnderscoreExpr, token::Token};
use feo_error::parser_error::ParserErrorKind;
use feo_types::{punctuation::PuncKind, Punctuation};

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for UnderscoreExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<feo_error::error::CompilerError>>
    where
        Self: Sized,
    {
        let underscore_opt = parser.peek_current::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::Underscore,
            ..
        }) = underscore_opt
        {
            parser.next_token();

            return Ok(Some(UnderscoreExpr {
                underscore: underscore_opt.unwrap(),
            }));
        }

        parser.log_error(ParserErrorKind::UnexpectedToken {
            expected: "`_`".to_string(),
            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
        });

        Err(parser.errors())
    }
}

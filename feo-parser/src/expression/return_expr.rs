use feo_ast::{
    expression::{ReturnExpr, Returnable},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, Keyword};

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for ReturnExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_return_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwReturn,
            ..
        }) = kw_return_opt
        {
            parser.next_token();

            if let Some(expr) = Returnable::parse(parser)? {
                parser.next_token();

                return Ok(Some(ReturnExpr {
                    kw_return: kw_return_opt.unwrap(),
                    expression_opt: Some(Box::new(expr)),
                }));
            }

            parser.next_token();

            return Ok(Some(ReturnExpr {
                kw_return: kw_return_opt.unwrap(),
                expression_opt: None,
            }));
        }

        parser.log_error(ParserErrorKind::UnexpectedToken {
            expected: "`Returnable`".to_string(),
            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
        });

        Err(parser.errors())
    }
}

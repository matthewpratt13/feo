use feo_ast::expression::{Expression, ReturnExpr};
use feo_error::error::CompilerError;
use feo_types::{keyword::KeywordKind, Keyword};

use crate::{
    parse::ParseExpr,
    parser::Parser,
    test_utils::{self, LogMsgType},
};

impl ParseExpr for ReturnExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_return_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwReturn,
            ..
        }) = kw_return_opt
        {
            test_utils::log_msg(LogMsgType::Enter, "return expression", parser);

            parser.next_token();

            let expression_opt = if let Some(e) = Expression::parse(parser)? {
                Some(Box::new(e))
            } else {
                None
            };

            test_utils::log_msg(LogMsgType::Exit, "return expression", parser);

            return Ok(Some(ReturnExpr {
                kw_return: kw_return_opt.unwrap(),
                expression_opt,
            }));
        } else {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_return_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"return x + 2"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let return_expr =
            ReturnExpr::parse(&mut parser).expect("unable to parse return expression");

        Ok(println!("{:#?}", return_expr))
    }
}

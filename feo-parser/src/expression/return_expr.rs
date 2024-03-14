use feo_ast::expression::{Expression, ReturnExpr};
use feo_error::error::CompilerError;
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

            let expression_opt = if let Some(e) = Expression::parse(parser)? {
                parser.next_token();
                Some(Box::new(e))
            } else {
                None
            };

            return Ok(Some(ReturnExpr {
                kw_return: kw_return_opt.unwrap(),
                expression_opt,
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
    fn parse_return_expr() {
        let source_code = r#"return x"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let return_expr =
            ReturnExpr::parse(&mut parser).expect("unable to parse return expression");

        println!("{:#?}", return_expr);
    }
}

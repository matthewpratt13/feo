use feo_ast::expression::{ReturnExpr, Returnable};
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

            let expression_opt = if let Some(e) = Returnable::parse(parser)? {
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
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_return_expr() {
        let source_code = r#"return x"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let return_expr =
            ReturnExpr::parse(&mut parser).expect("unable to parse return expression");

        println!("{:#?}", return_expr);
    }
}

use feo_ast::{
    expression::Expression,
    statement::{ExprStatement, LetStatement},
};
use feo_error::error::CompilerError;
use feo_types::{punctuation::PuncKind, Punctuation};

use crate::{
    parse::{ParseExpr, ParseStatement},
    parser::Parser,
};

impl ParseStatement for ExprStatement {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(expression) = Expression::parse(parser)? {
            parser.next_token();

            let semicolon_opt = parser.peek_current::<Punctuation>();

            match &semicolon_opt {
                Some(Punctuation {
                    punc_kind: PuncKind::Semicolon,
                    ..
                })
                | None => {
                    parser.next_token();

                    return Ok(Some(ExprStatement {
                        expression,
                        semicolon_opt,
                    }));
                }

                _ => return Ok(None),
            }
        } else {
            return Ok(None);
        }
    }
}

impl ParseStatement for LetStatement {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_expr_statement() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"x + 2"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let expr_statement =
            ExprStatement::parse(&mut parser).expect("unable to parse expression statement");

        Ok(println!("{:#?}", expr_statement))
    }
}

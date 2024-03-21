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

            let semicolon_opt = parser.peek_current();

            if let Some(Punctuation {
                punc_kind: PuncKind::Semicolon,
                ..
            }) = semicolon_opt
            {
                parser.next_token();

                return Ok(Some(ExprStatement {
                    expression,
                    semicolon_opt,
                }));
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
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

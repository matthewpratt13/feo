use feo_ast::{
    expression::{BlockExpr, ExprWithoutBlock},
    statement::Statement,
};
use feo_error::error::CompilerError;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    Delimiter,
};

use crate::{
    parse::{ParseExpr, ParseStatement},
    parser::Parser,
};

impl ParseExpr for BlockExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut statements: Vec<Statement> = Vec::new();
        let open_brace_opt = parser.peek_current();

        if let Some(Delimiter {
            delim: (DelimKind::Brace, DelimOrientation::Open),
            ..
        }) = open_brace_opt
        {
            parser.next_token();

            while let Some(s) = Statement::parse(parser)? {
                statements.push(s);
                parser.next_token();
            }

            let final_operand_opt = if let Some(e) = ExprWithoutBlock::parse(parser)? {
                parser.next_token();
                Some(Box::new(e))
            } else {
                None
            };

            let close_brace_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Close),
                ..
            }) = close_brace_opt
            {
                match statements.is_empty() {
                    true => {
                        return Ok(Some(BlockExpr {
                            open_brace: open_brace_opt.unwrap(),
                            statements_opt: None,
                            final_operand_opt,
                            close_brace: close_brace_opt.unwrap(),
                        }))
                    }
                    false => {
                        return Ok(Some(BlockExpr {
                            open_brace: open_brace_opt.unwrap(),
                            statements_opt: Some(statements),
                            final_operand_opt,
                            close_brace: close_brace_opt.unwrap(),
                        }))
                    }
                }
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

use feo_ast::{
    expression::{BlockExpr, ExprWithoutBlock},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    Delimiter,
};

use crate::{
    parse::ParseExpr,
    parser::Parser,
    utils::{self, LogMsgType},
};

impl ParseExpr for BlockExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        utils::log_msg(LogMsgType::Enter, "block expression", parser);

        let open_brace_opt = parser.peek_current();

        if let Some(Delimiter {
            delim: (DelimKind::Brace, DelimOrientation::Open),
            ..
        }) = open_brace_opt
        {
            parser.next_token();

            let statements_opt = utils::get_statements(parser)?;

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
                utils::log_msg(LogMsgType::Exit, "block expression", parser);

                return Ok(Some(BlockExpr {
                    open_brace: open_brace_opt.unwrap(),
                    statements_opt,
                    final_operand_opt,
                    close_brace: close_brace_opt.unwrap(),
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`}`".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_block_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        {
            foo(bar, 12, true);
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let block_expr = BlockExpr::parse(&mut parser).expect("unable to parse block expression");

        Ok(println!("{:#?}", block_expr))
    }
}

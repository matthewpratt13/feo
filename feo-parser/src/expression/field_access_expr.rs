use feo_ast::{
    expression::{FieldAccessExpr, Value},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{punctuation::PuncKind, Identifier, Punctuation};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
    test_utils::{self, LogMsgType},
};

impl ParseExpr for FieldAccessExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        // TODO: find a way to prevent stack overflow when accessing fields (as a field access expression)
        if let Some(container_operand) = Value::parse(parser)? {
            if let Some(Punctuation {
                punc_kind: PuncKind::FullStop,
                ..
            }) = parser.peek_next()
            {
                test_utils::log_msg(
                    LogMsgType::Detect,
                    "field access expression container operand",
                    parser,
                );
                parser.next_token();

                test_utils::log_msg(LogMsgType::Detect, "full stop", parser);

                if let Some(field_name) = parser.peek_next::<Identifier>() {
                    parser.next_token();

                    test_utils::log_msg(LogMsgType::Detect, "identifier", parser);

                    return Ok(Some(FieldAccessExpr {
                        container_operand: Box::new(container_operand),
                        field_name,
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                return Ok(None);
            }
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
    fn parse_field_access_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"hello.world"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let field_access_expr =
            FieldAccessExpr::parse(&mut parser).expect("unable to parse field access expression");

        Ok(println!("{:#?}", field_access_expr))
    }
}

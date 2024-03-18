use feo_ast::{
    expression::{TupleExpr, TupleIndexExpr, Value},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    literal::UIntType,
    punctuation::PuncKind,
    Delimiter, Literal, Punctuation, U64Primitive,
};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
    utils,
};

impl ParseExpr for TupleExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let open_parenthesis_opt = parser.peek_current::<Delimiter>();

        if let Some(Delimiter {
            delim: (DelimKind::Parenthesis, DelimOrientation::Open),
            ..
        }) = open_parenthesis_opt
        {
            parser.next_token();

            if let Some(elements) = utils::get_value_collection(parser)? {
                let close_parenthesis_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                    ..
                }) = close_parenthesis_opt
                {
                    parser.next_token();

                    return Ok(Some(TupleExpr {
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        elements,
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: ")".to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`TupleExprElements`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for TupleIndexExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(operand) = Value::parse(parser)? {
            parser.next_token();

            if let Some(Punctuation {
                punc_kind: PuncKind::FullStop,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(index) = parser.peek_current::<Literal<UIntType>>() {
                    parser.next_token();

                    return Ok(Some(TupleIndexExpr {
                        operand: Box::new(operand),
                        index: U64Primitive::try_from(index)
                            .expect("error converting `Literal<UIntType` to `U64Primitive`"),
                    }));
                }
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Literal<UIntType>`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`.`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
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
    fn parse_tuple_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"(1, "a", x)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_expr = TupleExpr::parse(&mut parser).expect("unable to parse tuple expression");

        Ok(println!("{:#?}", tuple_expr))
    }

    #[test]
    fn parse_tuple_index_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"foo.0"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_index_expr =
            TupleIndexExpr::parse(&mut parser).expect("unable to parse tuple index expression");

        Ok(println!("{:#?}", tuple_index_expr))
    }
}
